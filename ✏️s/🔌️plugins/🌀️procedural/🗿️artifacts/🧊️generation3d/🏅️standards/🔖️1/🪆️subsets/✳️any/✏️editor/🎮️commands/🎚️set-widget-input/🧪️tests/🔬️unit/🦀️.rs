use super::*;

#[test]
fn widget_input_language_neutral_cases() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for entry in fixture["cases"].as_array().unwrap() {
        let types: Vec<String> = entry["types"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_string()).collect();
        let current = if entry["current"].is_null() { None } else { Some(dsl::json::parse(&entry["current"].to_string()).unwrap()) };
        let result = edit_input_value(&types, current.as_ref(), entry["value"].as_str().unwrap(), entry.get("component").and_then(serde_json::Value::as_str));
        if entry["error"] == true { assert!(result.is_err(), "{}", entry["id"]); }
        else { assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&result.unwrap())).unwrap(), entry["expected"], "{}", entry["id"]); }
    }
}

#[test]
fn widget_input_edits_real_operator_params_and_rejects_connected_ports() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    crate::flow_operators::installed();
    let mut host = FlowHost::default();
    let source = host.add_widget(r#"{"kind":"inputSlider","id":"width"}"#, 0.0, 0.0).unwrap();
    let shape = host.add_widget(r#"{"kind":"neuron","id":"shape","neuronKind":"brep.mesh.box"}"#, 200.0, 0.0).unwrap();
    let mut payload = SetWidgetInput { widget_id: shape.clone(), channel: "width".into(), value: "2.5".into(), component: None, gesture: None };
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

#[semio_framework_async_macros::async_test]
async fn widget_input_publication_supports_scalar_events_and_undo_redo() {
    use crate::editor::generation3d::{unit_tests::context, Generation3dCommand, Generation3dPlayApp};
    use semio_framework_plugin::ArtifactEditor;
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = context::app().await;
    context::dispatch(&mut app, Generation3dCommand::AddWidget(crate::editor::generation3d::commands::add_widget::AddWidget { kind: "neuron".into(), neuron_kind: Some("brep.mesh.box".into()), format: None, action: None, x: None, y: None })).await;
    let before = context::snapshot(&app);
    let id = crate::widget_id(before.host_snapshot.widgets.last().unwrap()).to_string();
    let args: dsl::DslValue = serde_json::json!({"widgetId":id,"channel":"width","value":2.5}).into();
    let command = Generation3dPlayApp::command_from_action("setWidgetInput", Some(&args)).unwrap();
    context::dispatch(&mut app, command).await;
    let after = context::snapshot(&app);
    let params = after.host_snapshot.widgets.iter().find_map(|widget| match widget { Widget::Neuron { id: candidate, params, .. } if candidate == &id => Some(params.to_value()), _ => None }).unwrap();
    assert_eq!(params.get("width").and_then(|value| value.get("value")).and_then(dsl::DslValue::as_f64), Some(2.5));
    eprintln!("[DEBUG] inspector input {id}.width published 2.5");
    semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut *app, "undo", 1).await;
    assert_eq!(context::snapshot(&app), before);
    semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut *app, "redo", 1).await;
    assert_eq!(context::snapshot(&app), after);
    eprintln!("[DEBUG] inspector input {id}.width undo/redo restored typed parameters");
}
