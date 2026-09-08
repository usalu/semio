
use super::*;
use flow_extension_sdk::{FlowExtensionCommand, build_manifest_json, evaluate_json};

fn sample_dict() -> Dictionary {
    Dictionary::with_schema("dictionary").insert("number", Value::Dictionary(number_dictionary(3.0))).insert("text", Value::Dictionary(text_dictionary("hi".into())))
}

/// 🌱️ Wire-shape twin of [`sample_dict`], built with the first-party `pack::json::Value`
/// instead of `Dictionary`'s own `serde` codec — for JSON-text tests only.
fn json_sample_dict() -> pack::json::Value {
    let number = pack::json::object([("$schema".to_string(), pack::json::Value::from("number")), ("value".to_string(), pack::json::Value::from(3.0))]);
    let text = pack::json::object([("$schema".to_string(), pack::json::Value::from("text")), ("value".to_string(), pack::json::Value::from("hi"))]);
    pack::json::object([("$schema".to_string(), pack::json::Value::from("dictionary")), ("number".to_string(), number), ("text".to_string(), text)])
}

#[semio_framework_async_macros::async_test]
async fn get_reads_value() {
    let mut reg = Registry::new();
    register(&mut reg);
    let input = Dictionary::new().insert("dictionary", Value::Dictionary(sample_dict())).insert("key", Value::Dictionary(text_dictionary("number".into())));
    let out = reg.dispatch("dictionary.get", &input).unwrap();
    let value = out.get("value").and_then(|v| v.as_dictionary()).expect("value channel");
    assert_eq!(value.schema(), Some("number"));
}

#[semio_framework_async_macros::async_test]
async fn set_inserts_key() {
    let mut reg = Registry::new();
    register(&mut reg);
    let input = Dictionary::new().insert("dictionary", Value::Dictionary(Dictionary::with_schema("dictionary"))).insert("key", Value::Dictionary(text_dictionary("text".into()))).insert("value", Value::Dictionary(text_dictionary("new".into())));
    let out = reg.dispatch("dictionary.set", &input).unwrap();
    let dictionary = out.get("dictionary").and_then(|v| v.as_dictionary()).expect("dictionary channel");
    assert!(dictionary.get("text").is_some());
}

#[semio_framework_async_macros::async_test]
async fn merge_combines_dicts() {
    let mut reg = Registry::new();
    register(&mut reg);
    let items = Dictionary::new()
        .insert("0", Value::Dictionary(Dictionary::with_schema("dictionary").insert("a", Value::Dictionary(number_dictionary(1.0)))))
        .insert("1", Value::Dictionary(Dictionary::with_schema("dictionary").insert("b", Value::Dictionary(text_dictionary("x".into())))));
    let out = reg.dispatch("dictionary.merge", &Dictionary::new().insert("items", Value::Dictionary(items))).unwrap();
    let dictionary = out.get("dictionary").and_then(|v| v.as_dictionary()).expect("dictionary channel");
    assert_eq!(dictionary.schema(), Some("dictionary"));
    assert!(dictionary.get("a").is_some());
    assert!(dictionary.get("b").is_some());
}

#[semio_framework_async_macros::async_test]
async fn manifest_lists_dictionary_operators() {
    let json = build_manifest_json(
        "dictionary",
        "Dictionary",
        "0.1.0",
        &neural_engine::ColdOwner::new(module_registry()),
        vec!["onStartup".into()],
        vec![],
        vec![FlowExtensionCommand { id: "dictionary.showHelp".into(), title: "Dictionary: Show Help".into() }],
        vec![],
    );
    assert!(json.contains("dictionary.get"));
}

#[semio_framework_async_macros::async_test]
async fn evaluate_json_pack() {
    let out_json = evaluate_json(&neural_engine::ColdOwner::new(module_registry()), "dictionary.pack", &pack::json::to_string(&json_sample_dict()));
    let out = pack::json::parse(&out_json).unwrap();
    let dictionary = out.get("dictionary").expect("dictionary channel");
    assert_eq!(dictionary.get("$schema").and_then(pack::json::Value::as_str), Some("dictionary"));
}
