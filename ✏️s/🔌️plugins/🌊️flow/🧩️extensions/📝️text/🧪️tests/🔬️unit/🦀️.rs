
use super::*;
use flow_extension_sdk::{FlowExtensionCommand, build_manifest_json, evaluate_json};

#[semio_framework_async_macros::async_test]
async fn concat_joins_text() {
    let mut reg = Registry::new();
    register(&mut reg);
    let input = Dictionary::new().insert("a", Value::Dictionary(text_dictionary("hi".into()))).insert("b", Value::Dictionary(text_dictionary("!".into())));
    let out = reg.dispatch("text.concat", &input).unwrap();
    let text = out.get("text").and_then(|v| v.as_dictionary()).expect("text channel");
    assert_eq!(text.schema(), Some("text"));
    assert_eq!(text.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("hi!"));
}

#[semio_framework_async_macros::async_test]
async fn manifest_lists_text_operators() {
    let json = build_manifest_json("text", "Text", "0.1.0", &neural_engine::ColdOwner::new(module_registry()), vec!["onStartup".into()], vec![], vec![FlowExtensionCommand { id: "text.showHelp".into(), title: "Text: Show Help".into() }], vec![]);
    assert!(json.contains("text.concat"));
    assert!(json.contains("\"operators\""));
}

#[semio_framework_async_macros::async_test]
async fn evaluate_json_uppercases_text() {
    let text_value = pack::json::object([("$schema".to_string(), pack::json::Value::from("text")), ("value".to_string(), pack::json::Value::from("hi"))]);
    let input_json = pack::json::to_string(&pack::json::object([("text".to_string(), text_value)]));
    let out_json = evaluate_json(&neural_engine::ColdOwner::new(module_registry()), "text.upper", &input_json);
    let out = pack::json::parse(&out_json).unwrap();
    let text = out.get("text").expect("text channel");
    assert_eq!(text.get("$schema").and_then(pack::json::Value::as_str), Some("text"));
    assert_eq!(text.get("value").and_then(pack::json::Value::as_str), Some("HI"));
}
