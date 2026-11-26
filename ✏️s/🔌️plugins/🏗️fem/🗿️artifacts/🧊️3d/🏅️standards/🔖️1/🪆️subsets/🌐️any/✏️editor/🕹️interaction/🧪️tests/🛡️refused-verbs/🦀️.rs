use crate::context::dispatch;
use crate::context::fem3d_demo_app;
use crate::context::Fem3dApp;
use semio_framework_value::DslValue;
use semio_plugin::artifact_app_laws::{dispatch_fixture_action, fixture_identity, fixture_mounted_policy, meta, settle_registered_typed_operation};
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::set_active_example;

fn args(pairs: &[(&str, &str)]) -> Option<DslValue> {
    Some(DslValue::object(pairs.iter().map(|(key, value)| (key.to_string(), DslValue::String(value.to_string())))))
}

async fn verb(app: &mut Fem3dApp, action: &str, args: Option<DslValue>) -> Result<(), String> {
    let admitted = dispatch_fixture_action(&mut app.0, action, args, &meta("local"), &mut fixture_identity(), app.1.cx(), fixture_mounted_policy().close).await.map_err(|fault| format!("{fault:?}"))?;
    semio_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted, app.1.cx()).await.map_err(|fault| format!("{fault:?}"))?;
    settle_registered_typed_operation(&mut app.0, 1, fixture_mounted_policy(), &mut fixture_identity(), app.1.cx()).await.map_err(|fault| format!("{fault:?}"))?;
    Ok(())
}

/// 🛡️ LAW (3D): a refused reserved verb (Set Selection Mode on the undeclared domain `viewport`) is answered as its own refusal and never poisons the session — Select All selects the elements, Clear Selection empties them, the typed domain is accepted, and a typed command still commits afterwards.
#[semio_framework_async_macros::async_test]
async fn a_refused_reserved_verb_does_not_poison_the_session_3d() {
    let mut app = fem3d_demo_app().await;
    let error = verb(&mut app, "setSelectionMode", args(&[("domainId", "viewport"), ("mode", "single")])).await.expect_err("an undeclared domain is refused");
    assert!(error.contains("viewport"), "{error}");
    verb(&mut app, "selectAll", None).await.expect("select all after a refusal");
    let elements = app.snapshot().expect("snapshot").elements.len();
    let selected = app.interaction_state().await.selection.get("fem3d").map_or(0, |selection| selection.ids.len());
    assert_eq!(selected, elements, "Select All selects every element");
    verb(&mut app, "clearSelection", None).await.expect("clear selection after a refusal");
    assert!(app.interaction_state().await.selection.get("fem3d").is_none_or(|selection| selection.ids.is_empty()));
    verb(&mut app, "setSelectionMode", args(&[("domainId", "fem3d"), ("mode", "multiple")])).await.expect("the typed domain is accepted");
    dispatch(&mut app, Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "empty".into() })).await;
    assert!(app.snapshot().expect("snapshot").nodes.is_empty(), "a typed command commits after the refusal");
}
