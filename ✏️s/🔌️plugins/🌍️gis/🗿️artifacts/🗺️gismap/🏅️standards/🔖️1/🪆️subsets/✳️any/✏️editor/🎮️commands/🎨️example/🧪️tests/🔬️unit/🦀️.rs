
use super::*;
use crate::editor::gis2d::Gis2dCommand;
use crate::editor::gis2d::testkit::{app, app_with_registry, dispatch};
use semio_framework_plugin::PluginApp;

#[semio_framework_async_macros::async_test]
async fn set_active_example_empty_then_reuse_round_trips_document() {
    let mut app = app().await;
    assert!(!app.snapshot().expect("projection").positions.is_empty());
    dispatch(&mut app, Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: String::new() })).await;
    assert!(app.snapshot().expect("projection").positions.is_empty());
    dispatch(&mut app, Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "reuse-map".into() })).await;
    assert!(!app.snapshot().expect("projection").positions.is_empty());
    app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).await.expect("undo");
    assert!(app.snapshot().expect("projection").positions.is_empty(), "undo returns to the empty document");
}

/// 🧬️ `setActiveExample` replaces document content with batched create/delete/replace-data
/// operations, so it MUST be declared as an Operation. Under the real registry the View/Shell →
/// emits-operations guard rejects a mis-declaration; this proves the corrected declaration lets
/// the document-replacing edit flow through without erroring.
#[semio_framework_async_macros::async_test]
async fn set_active_example_is_operation_under_registry_kind_discipline() {
    let definition = crate::editor::gis2d::create_gis2d_app();
    let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
    assert!(matches!(action.kind, semio_framework_plugin::ActionKind::Mutation), "loading an example emits document-mutating operations, so it is a Mutation");
    assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");

    let mut app = app_with_registry().await;
    let result = dispatch(&mut app, Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: String::new() })).await;
    assert!(!result.mutations.is_empty(), "clearing a non-empty example emits at least one delete operation per removed feature");
    assert!(app.snapshot().expect("projection").positions.is_empty(), "the empty example clears every position feature");
}
