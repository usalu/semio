//! 🏭️ Generation3d actions reach their typed commands through the mounted editor's declared producer.

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

#[path = "../🔬️app-fixture/🦀️.rs"]
mod app_fixture;
use app_fixture::{app, snapshot, Generation3dApp};
use semio_framework::manifest::{CommandAddress, CommandOwnerAddress};
use semio_framework_artifact_flow_flow::Widget;
use semio_plugin::artifact_app_laws::{dispatch_fixture_action, dispatch_fixture_command, fixture_identity, meta, settle_history_verb, settle_registered_typed_operation};
use semio_plugin::{ArtifactApp, ArtifactEditor, EditorApp, MountedOwnerPolicyV1};
use semio_s_artifact_procedural_3d_editor::Generation3dPlayApp;

/// 🎚️ The first input slider of the fixture document with the nearest other value it accepts.
fn slider(app: &Generation3dApp) -> (String, f64, f64) {
    snapshot(app).host_snapshot.widgets.iter().find_map(|widget| match widget {
        Widget::InputSlider { id, value, min, max, step, .. } => Some((id.clone(), *value, if *value + *step <= *max { *value + *step } else { (*value - *step).max(*min) })),
        _ => None,
    }).expect("the fixture document holds an input slider")
}

#[semio_framework_async_macros::async_test]
async fn widget_value_edit_and_undo_reach_the_command_through_the_editor_producer() {
    let _serial = semio_s_artifact_procedural_3d_testkit::test_serial::lock();
    assert!(<Generation3dPlayApp as ArtifactEditor>::original_action_command_producer().is_some(), "the editor declares the contributions wire ceiling");
    assert!(<EditorApp<Generation3dPlayApp> as ArtifactApp>::original_action_command_producer().is_some(), "the mounted editor exposes the declared producer");
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
    settle_history_verb(&mut app.0, "redo", action_meta.instance_id, MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut fixture_identity(), app.1.cx()).await;
    assert_eq!(slider(&app).1, edited, "redo reapplies the widget value");
    settle_history_verb(&mut app.0, "undo", action_meta.instance_id, MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut fixture_identity(), app.1.cx()).await;
    assert_eq!(slider(&app).1, base, "a second undo restores it again");
}

#[semio_framework_async_macros::async_test]
async fn an_undeclared_action_is_refused_before_any_producer_is_born() {
    let _serial = semio_s_artifact_procedural_3d_testkit::test_serial::lock();
    let mut app = app().await;
    let refused = dispatch_fixture_action(&mut app.0, "noSuchGeneration3dAction", None, &meta("local"), &mut fixture_identity(), app.1.cx(), MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await;
    assert!(refused.is_err(), "an undeclared action never reaches a producer");
}

/// 📦️ A whole-pack `setContributions` crossing past the ordinary 64 KiB runtime wire ceiling reaches its typed command through the declared producer, because the producer prices it at the route's declared ceiling instead of the ordinary one.
#[semio_framework_async_macros::async_test]
async fn a_pack_wider_than_the_ordinary_runtime_wire_reaches_its_command() {
    let _serial = semio_s_artifact_procedural_3d_testkit::test_serial::lock();
    let mut app = app().await;
    let action_meta = meta("local");
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("../🧫️fixtures/📦️wide-contributions.json")).expect("wide contribution fixture");
    let padding_count = fixture["paddingCount"].as_u64().expect("padding count") as usize;
    let json = fixture["padding"].as_str().expect("padding").repeat(padding_count)
        + fixture["json"].as_str().expect("contributions JSON");
    assert!(json.len() > semio_plugin::RUNTIME_COMMAND_WIRE_MAXIMUM);
    let oracle: serde_json::Value = serde_json::from_str(&json).expect("the padded contribution pack is valid JSON");
    assert_eq!(oracle, fixture["expected"]);
    let maximum_depth = fixture["browserMaximumDepth"].as_u64().expect("browser maximum depth") as usize;
    let cx = app.1.cx();
    let previous_depth = cx.retained_grant().maximum_depth;
    semio_framework_job::testkit::retained_grant_mut(cx).maximum_depth = maximum_depth;
    assert_eq!(cx.retained_grant().maximum_depth, maximum_depth);
    let args: semio_framework_value::DslValue = serde_json::json!({ "json": json, "page": 0, "pageCount": 1 }).into();
    let mut policy = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
    policy.preparation.maximum_depth = maximum_depth;
    policy.maintenance.maximum_depth = maximum_depth;
    policy.close.maximum_depth = maximum_depth;
    let address = CommandAddress {
        owner: CommandOwnerAddress::App {
            plugin_id: "procedural".into(),
            app_id: "s.procedural.generation3d@1/*#editor".into(),
        },
        command_id: "setContributions".into(),
    };
    dispatch_fixture_command(&mut app.0, &address, Some(args), None, &action_meta, &mut fixture_identity(), app.1.cx(), policy.close)
        .await
        .expect("the wide contribution pack is admitted");
    let settlement = settle_registered_typed_operation(&mut app.0, action_meta.instance_id, policy, &mut fixture_identity(), app.1.cx()).await;
    semio_framework_job::testkit::retained_grant_mut(app.1.cx()).maximum_depth = previous_depth;
    let receipt = settlement.expect("the wide contribution pack settles");
    assert_eq!(receipt.completions, fixture["expectedCompletions"].as_u64().expect("expected completions") as usize);
}
