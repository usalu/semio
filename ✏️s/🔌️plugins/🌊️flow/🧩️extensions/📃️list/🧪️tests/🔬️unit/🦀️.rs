use super::*;
use flow_extension_sdk::{build_manifest_json, evaluate_json, FlowExtensionCommand};

fn sample_list() -> Dictionary {
    Dictionary::with_schema("list").insert("0", Value::Dictionary(number_dictionary(1.0))).insert("1", Value::Dictionary(number_dictionary(2.0))).insert("2", Value::Dictionary(number_dictionary(3.0)))
}

/// 🌱️ Wire-shape twin of [`super::number_dictionary`], built with the first-party
/// `pack::json::Value` instead of `Dictionary`'s own `serde` codec — for JSON-text tests only.
fn json_number(value: f64) -> pack::json::Value {
    pack::json::object([("$schema".to_string(), pack::json::Value::from("number")), ("value".to_string(), pack::json::Value::from(value))])
}

#[semio_framework_async_macros::async_test]
async fn empty_creates_list() {
    let mut reg = Registry::new();
    register(&mut reg);
    let out = reg.dispatch("list.empty", &Dictionary::new()).unwrap();
    let list = out.get("list").and_then(|v| v.as_dictionary()).expect("list channel");
    assert_eq!(list.schema(), Some("list"));
}

#[semio_framework_async_macros::async_test]
async fn get_reads_index() {
    let mut reg = Registry::new();
    register(&mut reg);
    let input = Dictionary::new()
        .insert("list", Value::Dictionary(sample_list()))
        .insert("index", Value::Dictionary(number_dictionary(1.0)))
        .insert("wrap", Value::Dictionary(Dictionary::with_schema("boolean").insert("value", Value::Atom(Atom::Boolean(false)))));
    let out = reg.dispatch("list.get", &input).unwrap();
    let value = out.get("0").and_then(|v| v.as_dictionary()).expect("output 0");
    assert_eq!(value.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(2.0));
}

#[semio_framework_async_macros::async_test]
async fn get_reads_consecutive_outputs() {
    let mut reg = Registry::new();
    register(&mut reg);
    let input = Dictionary::new()
        .insert("list", Value::Dictionary(sample_list()))
        .insert("index", Value::Dictionary(number_dictionary(0.0)))
        .insert("wrap", Value::Dictionary(Dictionary::with_schema("boolean").insert("value", Value::Atom(Atom::Boolean(false)))))
        .insert("count", Value::Dictionary(number_dictionary(2.0)));
    let out = reg.dispatch("list.get", &input).unwrap();
    let first = out.get("0").and_then(|v| v.as_dictionary()).expect("output 0");
    let second = out.get("1").and_then(|v| v.as_dictionary()).expect("output 1");
    assert_eq!(first.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(1.0));
    assert_eq!(second.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(2.0));
}

#[semio_framework_async_macros::async_test]
async fn get_wraps_consecutive_outputs() {
    let mut reg = Registry::new();
    register(&mut reg);
    let input = Dictionary::new()
        .insert("list", Value::Dictionary(sample_list()))
        .insert("index", Value::Dictionary(number_dictionary(2.0)))
        .insert("wrap", Value::Dictionary(Dictionary::with_schema("boolean").insert("value", Value::Atom(Atom::Boolean(true)))))
        .insert("count", Value::Dictionary(number_dictionary(2.0)));
    let out = reg.dispatch("list.get", &input).unwrap();
    let first = out.get("0").and_then(|v| v.as_dictionary()).expect("output 0");
    let second = out.get("1").and_then(|v| v.as_dictionary()).expect("output 1");
    assert_eq!(first.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
    assert_eq!(second.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(1.0));
}

#[semio_framework_async_macros::async_test]
async fn append_adds_next_index() {
    let mut reg = Registry::new();
    register(&mut reg);
    let input = Dictionary::new().insert("list", Value::Dictionary(sample_list())).insert("value", Value::Dictionary(number_dictionary(4.0)));
    let out = reg.dispatch("list.append", &input).unwrap();
    let list = out.get("list").and_then(|v| v.as_dictionary()).expect("list channel");
    assert_eq!(list.get("3").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(4.0));
}

#[semio_framework_async_macros::async_test]
async fn heterogeneous_list_input_rejected_at_evaluate() {
    let mut reg = Registry::new();
    register(&mut reg);
    let input = Dictionary::new()
        .insert("list", Value::Dictionary(Dictionary::with_schema("list").insert("0", Value::Dictionary(number_dictionary(1.0))).insert("1", Value::Dictionary(Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String("x".into())))))));
    let err = reg.dispatch("list.size", &input).unwrap_err();
    assert!(matches!(err, EvalError::HeterogeneousList(_)));
}

#[semio_framework_async_macros::async_test]
async fn manifest_lists_operators() {
    let mut reg = Registry::new();
    register(&mut reg);
    let json = build_manifest_json("list", "List", "0.1.0", &reg, vec!["onStartup".into()], vec![], vec![FlowExtensionCommand { id: "list.test".into(), title: "Test".into() }], vec![]);
    assert!(json.contains("list.get"));
    assert!(json.contains("operators"));
}

#[semio_framework_async_macros::async_test]
async fn evaluate_json_round_trips() {
    let reg = module_registry();
    let list = pack::json::object([("$schema".to_string(), pack::json::Value::from("list")), ("0".to_string(), json_number(1.0)), ("1".to_string(), json_number(2.0)), ("2".to_string(), json_number(3.0))]);
    let input_json = pack::json::to_string(&pack::json::object([("list".to_string(), list)]));
    let out_json = evaluate_json(&reg, "list.size", &input_json);
    assert!(out_json.contains("\"count\""));
    assert!(out_json.contains("\"value\""));
}
