use super::*;
use semio_framework_pack_json::{JsonMemberPolicy, Value};

fn fixture() -> Value {
    semio_framework_pack_json::parse(include_str!("../🧫️fixtures/🔣️.json"), JsonMemberPolicy::Reject).expect("closed shared path fixture")
}

fn expected(value: &Value) -> RustPathLiteral {
    RustPathLiteral {
        value: value["value"].as_str().unwrap().into(),
        path: value["path"].as_str().unwrap().into(),
        root: value["root"].as_str().unwrap().into(),
        call: value["call"].as_str().unwrap().into(),
        context: match value["context"].as_str().unwrap() {
            "method" => RustPathLiteralContext::Method,
            "qualified" => RustPathLiteralContext::Qualified,
            _ => panic!("unknown closed literal context"),
        },
        start: value["start"].as_u64().unwrap().try_into().unwrap(),
        end: value["end"].as_u64().unwrap().try_into().unwrap(),
        line: value["line"].as_u64().unwrap().try_into().unwrap(),
    }
}

#[test]
fn rust_path_literals_match_every_shared_utf16_reference_and_unresolved_context() {
    let fixture = fixture();
    let roots: Vec<&str> = fixture["roots"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect();
    for case in fixture["cases"].as_array().unwrap() {
        let references: Vec<RustPathLiteral> = case["expected"].as_array().unwrap().iter().map(expected).collect();
        let actual = inspect_rust_path_literals(case["source"].as_str().unwrap(), &roots).unwrap();
        assert_eq!(actual, references, "{}", case["id"].as_str().unwrap());
        eprintln!("[DEBUG] Rust shared path {}: {actual:?}", case["id"].as_str().unwrap());
    }
}

#[test]
fn rust_path_roots_refuse_every_shared_invalid_input_with_owned_error() {
    let fixture = fixture();
    for case in fixture["invalidRoots"].as_array().unwrap() {
        let roots: Vec<&str> = case["roots"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect();
        let error: RustPathRootError = inspect_rust_path_literals("", &roots).unwrap_err();
        assert_eq!(error.code(), case["refusal"].as_str().unwrap(), "{}", case["id"].as_str().unwrap());
        assert_eq!(error.to_string(), "Rust path root must be unique and canonical relative");
        eprintln!("[DEBUG] Rust shared invalid root {}: {}", case["id"].as_str().unwrap(), error.code());
    }
}
