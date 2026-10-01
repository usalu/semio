use super::*;

/// 📇️ Supplies typed input metadata for this document-editing law without a geometry evaluator.
fn input_metadata() {
    fn install(registry: &mut semio_framework_os_flow::neural::Registry) {
        use semio_framework_os_flow::neural::{ChannelSpec, OperatorInfo};
        registry.register_operator(OperatorInfo { id: "brep.mesh.box".into(), extension: "widget-input-law".into(), name: "Box".into(), inputs: vec![ChannelSpec::number_default("width", 1.0, &[] as &[&str])], ..Default::default() }, Vec::new(), &[]);
    }
    semio_framework_os_flow::install_flow_extension(semio_framework_os_flow::FlowExtensionSpec { id: "widget-input-law".into(), name: "Widget Input Law".into(), version: "1".into(), install }).expect("typed input metadata admission");
}

#[test]
fn widget_input_language_neutral_cases() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for entry in fixture["cases"].as_array().unwrap() {
        let types: Vec<String> = entry["types"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_string()).collect();
        let current = if entry["current"].is_null() { None } else { Some(dsl::DslValue::from(entry["current"].clone())) };
        let result = edit_input_value(&types, current.as_ref(), entry["value"].as_str().unwrap(), entry.get("component").and_then(serde_json::Value::as_str));
        if entry["error"] == true { assert!(result.is_err(), "{}", entry["id"]); }
        else { assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&result.unwrap())).unwrap(), entry["expected"], "{}", entry["id"]); }
    }
}

#[test]
fn widget_input_edits_real_operator_params_and_rejects_connected_ports() {
    let _serial = crate::test_serial::lock();
    input_metadata();
    let mut host = FlowHost::default().with_operator_registry(semio_framework_os_flow::flow_operator_registry());
    let source = host.add_widget(r#"{"kind":"inputSlider","id":"width"}"#, 0.0, 0.0).unwrap();
    let shape = host.add_widget(r#"{"kind":"neuron","id":"shape","neuronKind":"brep.mesh.box"}"#, 200.0, 0.0).unwrap();
    let mut payload = SetWidgetInput { widget_id: shape.clone(), channel: "width".into(), value: "2.5".into(), component: None };
    apply_to_host(&mut host, &payload).unwrap();
    let params = host.host_snapshot.widgets.iter().find_map(|widget| match widget { Widget::Neuron { id, params, .. } if id == &shape => Some(params.to_value()), _ => None }).unwrap();
    assert_eq!(params.get("width").and_then(|value| value.get("value")).and_then(dsl::DslValue::as_f64), Some(2.5));
    host.connect_ports(&source, "number", &shape, "width").unwrap();
    payload.value = "3.5".into();
    assert!(apply_to_host(&mut host, &payload).unwrap_err().contains("connected"));
    payload.channel = "missing".into();
    assert!(apply_to_host(&mut host, &payload).is_err());
    host.retire_cold();
}
