
use crate::SequenceCamera;
use crate::editor::sequence::SequenceCommand;
use crate::editor::sequence::testkit::{dispatch, new_app, new_app_with_registry_wired, select_steps};
use semio_framework_plugin::{PluginApp, ViewModel};

use super::set_viewport::SetViewport;

/// 🎥️ `SetViewport` is config-only — it must never emit a `SequenceMutation` (no VCS edit, no
/// undo entry) and instead write straight into the config store.
#[semio_framework_async_macros::async_test]
async fn set_viewport_writes_config_not_operations() {
    let mut app = new_app().await;
    let result = app.dispatch_typed(SequenceCommand::SetViewport(SetViewport { camera: SequenceCamera { x: 5.0, y: 6.0, zoom: 2.0 } }), &semio_framework_plugin::testkit::meta("local")).await.expect("viewport pan/zoom");
    assert!(result.mutations.is_empty(), "setViewport must not emit a VCS operation");
    let node = app.render(crate::editor::sequence::modes::edit::windows::main::SEQUENCE_PLAY_BODY_MAIN, None, &ViewModel::default()).await.expect("render");
    let semio_framework_plugin::Component::Surface(props) = &node.root.component else { panic!("semantic graph") };
    let scene: semio_framework_plugin::NodeGraphScene = semio_framework_ui_scene::decode(props).expect("packed graph");
    assert_eq!(scene.viewport.expect("camera viewport").zoom, 2.0);
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(node).expect("retire viewport graph");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking is now the framework's
/// injected `interactionSelect` verb against the "steps" domain — requires a registry-wired app
/// (see `select_steps`'s own doc comment) so `NodeGraphEdit::apply` (not the macro-dispatched
/// `handle`, which always sees an empty selection) reads the live selection.
#[semio_framework_async_macros::async_test]
async fn node_graph_edit_delete_selection_clears_selection() {
    let mut app = new_app_with_registry_wired().await;
    select_steps(&mut app, &["step-1"]).await;
    dispatch(&mut app, SequenceCommand::NodeGraphEdit(super::node_graph_edit::NodeGraphEdit { operations_json: "[{\"operation\":\"deleteSelection\"}]".into() })).await;
    assert!(!app.snapshot().expect("projection").to_fixture().steps.iter().any(|step| step.id == "step-1"));
}
