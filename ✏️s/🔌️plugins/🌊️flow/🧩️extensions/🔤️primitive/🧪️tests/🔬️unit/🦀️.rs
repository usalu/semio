
use super::*;
use flow_extension_sdk::{build_manifest_json, evaluate_json};

#[semio_framework_async_macros::async_test]
async fn variable_relay_forwards_named_channel() {
    let mut registry = Registry::new();
    register(&mut registry);
    let input = Dictionary::new().insert("name", Value::Atom(Atom::String("width".into()))).insert("schema", Value::Atom(Atom::String("number".into()))).insert("width", Value::Dictionary(number_dictionary(2.0)));
    let out = registry.dispatch("core.variable", &input).unwrap();
    let width = out.get("width").and_then(|v| v.as_dictionary()).expect("width channel");
    assert_eq!(width.schema(), Some("number"));
}

#[semio_framework_async_macros::async_test]
async fn number_emits_schema_dictionary() {
    let mut registry = Registry::new();
    register(&mut registry);
    let out = registry.dispatch("core.number", &Dictionary::new().insert("value", Value::Atom(Atom::Decimal(2.5)))).unwrap();
    let number = out.get("number").and_then(|v| v.as_dictionary()).expect("number channel");
    assert_eq!(number.schema(), Some("number"));
    assert_eq!(number.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(2.5));
}

#[semio_framework_async_macros::async_test]
async fn manifest_lists_schemas_and_operators() {
    let json = build_manifest_json("core", "Core", "0.1.0", &neural_engine::ColdOwner::new(module_registry()), vec!["onStartup".into()], vec![], vec![], vec![]);
    assert!(json.contains("\"schemas\""));
    assert!(json.contains("core.number"));
    assert!(json.contains("\"number\""));
}

#[semio_framework_async_macros::async_test]
async fn evaluate_json_text() {
    let input_json = pack::json::to_string(&pack::json::object([("value".to_string(), pack::json::Value::from("hi"))]));
    let out_json = evaluate_json(&neural_engine::ColdOwner::new(module_registry()), "core.text", &input_json);
    let out = pack::json::parse(&out_json).unwrap();
    let text = out.get("text").expect("text channel");
    assert_eq!(text.get("$schema").and_then(pack::json::Value::as_str), Some("text"));
    assert_eq!(text.get("value").and_then(pack::json::Value::as_str), Some("hi"));
}
