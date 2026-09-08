
use super::*;
use flow_extension_sdk::{FlowExtensionCommand, build_manifest_json, evaluate_json};

#[semio_framework_async_macros::async_test]
async fn greater_compares_numbers() {
    let mut reg = Registry::new();
    register(&mut reg);
    let input = Dictionary::new().insert("a", Value::Dictionary(number_dictionary(5.0))).insert("b", Value::Dictionary(number_dictionary(2.0)));
    let out = reg.dispatch("logic.greater", &input).unwrap();
    let boolean = out.get("boolean").and_then(|v| v.as_dictionary()).expect("boolean channel");
    assert_eq!(boolean.schema(), Some("boolean"));
    assert_eq!(boolean.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_bool()), Some(true));
}

#[semio_framework_async_macros::async_test]
async fn manifest_lists_logic_operators() {
    let json = build_manifest_json("logic", "Logic", "0.1.0", &neural_engine::ColdOwner::new(module_registry()), vec!["onStartup".into()], vec![], vec![FlowExtensionCommand { id: "logic.showHelp".into(), title: "Logic: Show Help".into() }], vec![]);
    assert!(json.contains("logic.greater"));
}

#[semio_framework_async_macros::async_test]
async fn evaluate_json_greater() {
    let json_number = |value: f64| pack::json::object([("$schema".to_string(), pack::json::Value::from("number")), ("value".to_string(), pack::json::Value::from(value))]);
    let input_json = pack::json::to_string(&pack::json::object([("a".to_string(), json_number(5.0)), ("b".to_string(), json_number(2.0))]));
    let out_json = evaluate_json(&neural_engine::ColdOwner::new(module_registry()), "logic.greater", &input_json);
    let out = pack::json::parse(&out_json).unwrap();
    let boolean = out.get("boolean").expect("boolean channel");
    assert_eq!(boolean.get("value").and_then(pack::json::Value::as_bool), Some(true));
}
