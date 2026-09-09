use super::*;
use flow_extension_sdk::{build_manifest_json, evaluate_json, FlowExtensionCommand, FlowExtensionSetting};

#[semio_framework_async_macros::async_test]
async fn add_sums_number_dictionaries() {
    let mut reg = Registry::new();
    register(&mut reg);
    let input = Dictionary::new().insert("a", Value::Dictionary(number_dictionary(3.0))).insert("b", Value::Dictionary(number_dictionary(1.1)));
    let out = reg.dispatch("math.add", &input).unwrap();
    let sum = out.get("sum").and_then(|v| v.as_dictionary()).expect("sum channel");
    assert_eq!(sum.schema(), Some("number"));
    assert_eq!(sum.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(4.1));
}

#[semio_framework_async_macros::async_test]
async fn construct_vector_uses_xyz_channels() {
    let mut reg = Registry::new();
    register(&mut reg);
    let input = Dictionary::new().insert("x", Value::Dictionary(number_dictionary(1.0))).insert("y", Value::Dictionary(number_dictionary(2.0))).insert("z", Value::Dictionary(number_dictionary(3.0)));
    let out = reg.dispatch("math.vector", &input).unwrap();
    let vector = out.get("vector").and_then(|v| v.as_dictionary()).expect("vector channel");
    assert_eq!(vector.schema(), Some("vector"));
    assert_eq!(vector.get("z").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
}

#[semio_framework_async_macros::async_test]
async fn schema_component_round_trips_vector() {
    let reg = module_registry();
    let built = reg.dispatch("math.vector", &Dictionary::new().insert("x", Value::Dictionary(number_dictionary(1.0))).insert("y", Value::Dictionary(number_dictionary(2.0))).insert("z", Value::Dictionary(number_dictionary(3.0)))).unwrap();
    let vector = built.get("vector").and_then(|value| value.as_dictionary()).expect("vector");
    let deconstructed = reg.dispatch("math.vector", &Dictionary::new().insert("vector", Value::Dictionary(vector.clone()))).unwrap();
    assert_eq!(deconstructed.get("y").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(2.0));
}

#[semio_framework_async_macros::async_test]
async fn move_translates_point() {
    let mut reg = Registry::new();
    register(&mut reg);
    let input = Dictionary::new().insert("subject", Value::Dictionary(xyz_dictionary("point", Vec3::new(1.0, 2.0, 3.0)))).insert("vector", Value::Dictionary(xyz_dictionary("vector", Vec3::new(4.0, 5.0, 6.0))));
    let out = reg.dispatch("math.move", &input).unwrap();
    let point = out.get("point").and_then(|v| v.as_dictionary()).expect("point channel");
    assert_eq!(point.schema(), Some("point"));
    assert_eq!(point.get("x").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(5.0));
}

#[semio_framework_async_macros::async_test]
async fn manifest_lists_math_operators_and_schemas() {
    let json = build_manifest_json(
        "math",
        "Math",
        "0.2.0",
        &neural_engine::ColdOwner::new(module_registry()),
        vec!["onStartup".into()],
        vec![],
        vec![FlowExtensionCommand { id: "math.showHelp".into(), title: "Math: Show Help".into() }],
        vec![FlowExtensionSetting { id: "math.defaultPrecision".into(), setting_type: "number".into(), default: flow_extension_sdk::integer_setting_default(1), description: "Decimal places for number preview".into() }],
    );
    assert!(json.contains("flow.extension"));
    assert!(json.contains("math.vector"));
    assert!(json.contains("vector"));
}

#[semio_framework_async_macros::async_test]
async fn evaluate_json_adds_numbers() {
    let reg = module_registry();
    let json_number = |value: f64| pack::json::object([("$schema".to_string(), pack::json::Value::from("number")), ("value".to_string(), pack::json::Value::from(value))]);
    let input_json = pack::json::to_string(&pack::json::object([("a".to_string(), json_number(2.0)), ("b".to_string(), json_number(1.0))]));
    let out_json = evaluate_json(&reg, "math.add", &input_json);
    let out = pack::json::parse(&out_json).unwrap();
    let sum = out.get("sum").expect("sum channel");
    assert_eq!(sum.get("$schema").and_then(pack::json::Value::as_str), Some("number"));
    assert_eq!(sum.get("value").and_then(pack::json::Value::as_f64), Some(3.0));
}

#[semio_framework_async_macros::async_test]
async fn random_is_deterministic_with_seed() {
    let mut reg = Registry::new();
    register(&mut reg);
    let input = Dictionary::new().insert("seed", Value::Dictionary(number_dictionary(42.0))).insert("min", Value::Dictionary(number_dictionary(0.0))).insert("max", Value::Dictionary(number_dictionary(1.0)));
    let first = reg.dispatch("math.random", &input).unwrap();
    let second = reg.dispatch("math.random", &input).unwrap();
    let first_value = first.get("random").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
    let second_value = second.get("random").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
    assert_eq!(first_value, second_value);
}

#[semio_framework_async_macros::async_test]
async fn divide_rejects_zero() {
    let mut reg = Registry::new();
    register(&mut reg);
    let input = Dictionary::new().insert("a", Value::Dictionary(number_dictionary(1.0))).insert("b", Value::Dictionary(number_dictionary(0.0)));
    assert!(reg.dispatch("math.divide", &input).is_err());
}
