//! 🧯️ The mounted host cancels one visible retained operation with exact authority and no document edit.
use super::{context, mounted};
use semio_plugin::{PluginApp, artifact_app_laws};
use semio_plugin::app::TypedOperationResultLane;
use serde_json::Value;

fn cancellation_binding(value: &Value, action: &str) -> Option<Value> {
    match value {
        Value::Object(fields) => {
            if fields.get("action").is_some_and(|address| address["name"] == action) && fields.contains_key("args") { return Some(value.clone()); }
            fields.values().find_map(|value| cancellation_binding(value, action))
        }
        Value::Array(values) => values.iter().find_map(|value| cancellation_binding(value, action)),
        _ => None,
    }
}

fn cancellation_control<'a>(value: &'a Value, action: &str, args: &Value) -> Option<&'a Value> {
    match value {
        Value::Object(fields) => {
            if fields.get("bindings").and_then(Value::as_array).is_some_and(|bindings| bindings.iter().any(|binding| binding["action"]["name"] == action && &binding["args"] == args)) { return Some(value); }
            fields.values().find_map(|value| cancellation_control(value, action, args))
        }
        Value::Array(values) => values.iter().find_map(|value| cancellation_control(value, action, args)),
        _ => None,
    }
}

async fn tick(app: &mut context::LowpolyApp) {
    semio_framework_async::yield_once().await;
    app.0.maintenance_step(app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).unwrap();
    app.0.advance_typed_operation_publication(&mut artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn visible_operation_cancel_rejects_stale_generation_and_preserves_document_history() {
    let fixture: Value = serde_json::from_str(include_str!("cancellation.json")).unwrap();
    let mut app = mounted();
    let selected: Vec<(&str, &str)> = fixture["selection"].as_array().unwrap().iter().map(|row| (row[0].as_str().unwrap(), row[1].as_str().unwrap())).collect();
    context::select(&mut app, &selected).await;
    let before = app.snapshot().unwrap();
    let edits_before = context::committed_edits(&mut app).await;
    let meta = context::action_meta();
    let args = semio_framework_value::DslValue::from(&fixture["arguments"]);
    artifact_app_laws::dispatch_fixture_action(&mut app.0, fixture["action"].as_str().unwrap(), Some(args), &meta, &mut artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap();
    assert!(app.has_pending_typed_operations());
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    let binding = loop {
        assert!(std::time::Instant::now() < deadline, "operation never exposed retained progress");
        let tree: Value = serde_json::from_str(&context::render(&mut app, fixture["body"].as_str().unwrap()).await).unwrap();
        if let Some(binding) = cancellation_binding(&tree, fixture["cancelAction"].as_str().unwrap()) { break binding; }
        assert_eq!(app.snapshot().unwrap(), before, "operation published before visible cancellation boundary");
        assert!(app.has_pending_typed_operations(), "operation ended before visible cancellation boundary");
        tick(&mut app).await;
    };
    assert!(app.has_pending_typed_operations(), "visible operation is still live");
    assert_eq!(app.snapshot().unwrap(), before, "visible operation is not yet published");
    let definition = semio_s_artifact_lowpoly_lowpoly_editor::create_lowpoly_app();
    assert_eq!(binding["action"]["scope"].as_str(), Some(definition.controller_id.as_str()));
    let exact = binding["args"].clone();
    let generation = u64::from_str_radix(exact["generation"].as_str().unwrap(), 16).unwrap();
    let mut stale = exact.clone();
    stale["generation"] = Value::String(format!("{:016x}", generation ^ u64::from_str_radix(fixture["staleGenerationXor"].as_str().unwrap(), 16).unwrap()));
    let stale_rejected = artifact_app_laws::dispatch_fixture_action(&mut app.0, fixture["cancelAction"].as_str().unwrap(), Some(semio_framework_value::DslValue::from(&stale)), &meta, &mut artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.is_err();
    assert!(stale_rejected);
    let after_stale: Value = serde_json::from_str(&context::render(&mut app, fixture["body"].as_str().unwrap()).await).unwrap();
    let control = cancellation_control(&after_stale, fixture["cancelAction"].as_str().unwrap(), &exact).expect("stale rejection preserves the same live control");
    assert_eq!(control["disabled"], false, "stale generation does not request cancellation");
    assert!(app.has_pending_typed_operations());
    assert_eq!(app.snapshot().unwrap(), before);
    assert_eq!(context::committed_edits(&mut app).await, edits_before);
    artifact_app_laws::dispatch_fixture_action(&mut app.0, fixture["cancelAction"].as_str().unwrap(), Some(semio_framework_value::DslValue::from(&exact)), &meta, &mut artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap();
    let mut terminal = false;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    while app.has_pending_typed_operations() {
        assert!(std::time::Instant::now() < deadline, "cancelled operation did not retire");
        tick(&mut app).await;
        while let Some(page) = app.take_typed_operation_result_page(context::INSTANCE) {
            assert_ne!(page.lane, TypedOperationResultLane::Fault, "user cancellation is a terminal outcome");
            terminal |= page.lane == TypedOperationResultLane::Terminal;
            assert!(app.acknowledge_typed_operation_result(page.token).unwrap());
        }
        while app.take_typed_operation_effect().is_some() {}
        while app.take_typed_operation_event().is_some() {}
        while app.take_typed_operation_ui_scope().is_some() {}
        while app.take_typed_operation_completion().await.unwrap().is_some() {}
    }
    let retired: Value = serde_json::from_str(&context::render(&mut app, fixture["body"].as_str().unwrap()).await).unwrap();
    let mut late_completion = false;
    for _ in 0..fixture["lateTurns"].as_u64().unwrap() {
        tick(&mut app).await;
        late_completion |= app.take_typed_operation_completion().await.unwrap().is_some();
        assert!(app.take_typed_operation_result_page(context::INSTANCE).is_none());
        assert!(app.take_typed_operation_effect().is_none());
    }
    let observed = serde_json::json!({
        "staleRejected": stale_rejected,
        "cancelTerminal": terminal,
        "snapshotUnchanged": app.snapshot().unwrap() == before,
        "committedEditsUnchanged": context::committed_edits(&mut app).await == edits_before,
        "progressRetired": cancellation_binding(&retired, fixture["cancelAction"].as_str().unwrap()).is_none(),
        "lateCompletionSuppressed": !late_completion && !app.has_pending_typed_operations()
    });
    assert_eq!(observed, fixture["expected"]);
}
