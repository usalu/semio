//! 🏭️ A normal generation3d action reaches its typed command through the framework-declared action producer: the plugin declares none, yet a widget value edit and its undo run end to end on the mounted app.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "../🔬️app-fixture/🦀️.rs"]
mod app_fixture;
use app_fixture::{app, snapshot, Generation3dApp};
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_plugin::artifact_app_laws::{dispatch_fixture_action, fixture_identity, meta, settle_history_verb, settle_registered_typed_operation};
use semio_framework_plugin::{ArtifactApp, ArtifactEditor, EditorApp, MountedOwnerPolicyV1};
use semio_s_artifact_procedural_3d_editor::Generation3dPlayApp;

/// 🎚️ The first input slider of the fixture document with the nearest other value it accepts.
fn slider(app: &Generation3dApp) -> (String, f64, f64) {
    snapshot(app).host_snapshot.widgets.iter().find_map(|widget| match widget {
        Widget::InputSlider { id, value, min, max, step, .. } => Some((id.clone(), *value, if *value + *step <= *max { *value + *step } else { (*value - *step).max(*min) })),
        _ => None,
    }).expect("the fixture document holds an input slider")
}

#[semio_framework_async_macros::async_test]
async fn widget_value_edit_and_undo_reach_the_command_through_the_framework_declared_producer() {
    let _serial = semio_s_artifact_procedural_3d_testkit::test_serial::lock();
    assert!(<Generation3dPlayApp as ArtifactEditor>::original_action_command_producer().is_none(), "the plugin hand-writes no producer");
    assert!(<EditorApp<Generation3dPlayApp> as ArtifactApp>::original_action_command_producer().is_some(), "the framework supplies the generic producer");
    let mut app = app().await;
    let (id, base, edited) = slider(&app);
    assert_ne!(base, edited, "the edit must move the value");
    let action_meta = meta("local");
    let args: semio_framework_value::DslValue = serde_json::json!({ "widgetIds": [id], "field": "value", "value": edited }).into();
    dispatch_fixture_action(&mut app.0, "patchFlowWidgets", Some(args), &action_meta, &mut fixture_identity(), app.1.cx(), MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("patchFlowWidgets is admitted through the producer");
    settle_registered_typed_operation(&mut app.0, action_meta.instance_id, MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut fixture_identity(), app.1.cx()).await.expect("the edit publishes");
    assert_eq!(slider(&app).1, edited, "the typed command the producer born applied the edit");
    settle_history_verb(&mut app.0, "undo", action_meta.instance_id, MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut fixture_identity(), app.1.cx()).await;
    assert_eq!(slider(&app).1, base, "undo restores the widget value");
}

#[semio_framework_async_macros::async_test]
async fn an_undeclared_action_is_refused_before_any_producer_is_born() {
    let _serial = semio_s_artifact_procedural_3d_testkit::test_serial::lock();
    let mut app = app().await;
    let refused = dispatch_fixture_action(&mut app.0, "noSuchGeneration3dAction", None, &meta("local"), &mut fixture_identity(), app.1.cx(), MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await;
    assert!(refused.is_err(), "an undeclared action never reaches a producer");
}
