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

/// ⚖️ LAW (design §19): a committed operator field is ONE absolute `change-widget-input` leaf of the port's literal type;
/// an unchanged value is no edit, a text source's text is a text input, a connected or unknown input is refused before
/// any document mutation.
#[test]
fn widget_input_is_one_typed_leaf_and_rejects_connected_ports() {
    use semio_framework_os_flow::FlowHost;
    let _serial = crate::test_serial::lock();
    input_metadata();
    let mut host = FlowHost::default().with_operator_registry(semio_framework_os_flow::flow_operator_registry());
    let source = host.add_widget(r#"{"kind":"inputSlider","id":"width"}"#, 0.0, 0.0).unwrap();
    let shape = host.add_widget(r#"{"kind":"neuron","id":"shape","neuronKind":"brep.mesh.box"}"#, 200.0, 0.0).unwrap();
    let note = host.add_widget(r#"{"kind":"inputNote","text":"hi"}"#, 0.0, 200.0).unwrap();
    let mut payload = SetWidgetInput { widget_id: shape.clone(), channel: "width".into(), value: "2.5".into(), component: None };
    assert_eq!(input_leaf(&host.host_snapshot, &payload).unwrap(), Some(change_widget_input(&shape, "width", WidgetInputValue::Number(2.5))));
    host.set_neuron_params(&shape, r#"{"width":{"$schema":"number","value":2.5}}"#).unwrap();
    assert_eq!(input_leaf(&host.host_snapshot, &payload).unwrap(), None, "an unchanged field is no edit");
    let text = SetWidgetInput { widget_id: note.clone(), channel: "text".into(), value: "hello".into(), component: None };
    assert_eq!(input_leaf(&host.host_snapshot, &text).unwrap(), Some(change_widget_input(&note, "text", WidgetInputValue::Text("hello".into()))));
    host.connect_ports(&source, "number", &shape, "width").unwrap();
    payload.value = "3.5".into();
    assert!(input_leaf(&host.host_snapshot, &payload).unwrap_err().contains("connected"));
    payload.channel = "missing".into();
    assert!(input_leaf(&host.host_snapshot, &payload).is_err());
    host.retire_cold();
}
