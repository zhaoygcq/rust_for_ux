extern crate wasm_bindgen;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
struct Position {
    start: usize,
    end: usize
}

#[derive(std::cmp::PartialEq)]
enum State {
    Pendding,
    Start,
    Symbol,
    FirstStartMustache,
    SecondStartMustache,
    FirstEndMustache,
    SecondEndMustache
}

#[derive(Serialize, Deserialize)]
enum Section {
    String(String),
    Position(Position)
}

#[wasm_bindgen]
pub fn greet(html: &str) -> JsValue {
    let mut res = vec![];
    let mut last_state = State::Pendding;
    let mut last_index = 0;
    let mut mustache_count = 0;
    let bytes = html.chars();

    for (i, item) in bytes.enumerate() {
        if item == '=' {
            if last_state == State::Pendding {
                last_state = State::Start;
            }
        } else if item == '\'' || item == '\"' {
            if last_state == State::Start {
                last_state = State::Symbol;
            } else if last_state == State::SecondEndMustache {
                last_state = State::Pendding;
                last_index = i + 1;
            }
        } else if item == '{' {
            if last_state == State::Symbol {
                last_state = State::FirstStartMustache;
            } else if last_state == State::FirstStartMustache {
                last_state = State::SecondStartMustache;
                // 处理{{前的内容
                res.push(Section::Position(Position{
                    start: last_index,
                    end: i - 2
                }));
                res.push(Section::String(String::from("{  ")));
                last_index = i;
            }

            // 记录当前处理的{的数量
            mustache_count = mustache_count + 1;
        } else if item == '}' {
            if last_state == State::SecondStartMustache {
                // 处理{{}}之间的内容
                if mustache_count == 2 {
                    last_state = State::FirstEndMustache;
                    res.push(Section::Position(Position{
                        start: last_index,
                        end: i
                    }));
                    res.push(Section::String(String::from("  }")));
                }
            } else if last_state == State::FirstEndMustache {
                last_state = State::SecondEndMustache;
            }

            mustache_count = mustache_count - 1;
        } else {
            if last_state != State::SecondStartMustache {
                last_state = State::Pendding;
            }
        }

    }
    JsValue::from_serde(&res).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
