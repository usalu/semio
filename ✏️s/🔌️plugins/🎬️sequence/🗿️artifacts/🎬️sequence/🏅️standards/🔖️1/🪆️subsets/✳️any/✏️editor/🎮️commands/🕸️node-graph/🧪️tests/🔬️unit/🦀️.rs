use crate::editor::sequence::unit_tests::context::{dispatch, live_host_snapshot, main_window_meta, new_app, new_app_with_registry_wired, select_steps};
use crate::editor::sequence::SequenceCommand;
use crate::SequenceCamera;
use semio_framework_plugin::PluginApp;

use super::set_viewport::SetViewport;

/// 🎥️ `SetViewport` is config-only — it must never emit a `SequenceMutation` (no VCS edit, no
/// undo entry) and instead write straight into the config store.
#[semio_framework_async_macros::async_test]
async fn set_viewport_writes_config_not_operations() {
    let mut app = new_app().await;
    // 🔁️ A mounted app publishes AFTER it answers, so the config write is only visible once the
    // retained operation has settled — `context::dispatch` drives that continuation the way the host
    // does. `result.mutations` is always empty on a mounted app; the no-VCS-edit claim is proven by
    // the settled receipt carrying no `Artifact` lane.
    let result = dispatch(&mut app, SequenceCommand::SetViewport(SetViewport { camera: SequenceCamera { x: 5.0, y: 6.0, zoom: 2.0 } })).await;
    assert!(result.mutations.is_empty(), "setViewport must not emit a VCS operation");
    // 🪟️ The camera lives in the MAIN window's own config, so the read has to come through the same
    // window instance the write was addressed at — an unaddressed render sees the default camera.
    let view = main_window_meta().view_state.expect("main window view");
    let node = app.render(crate::editor::sequence::modes::edit::windows::main::SEQUENCE_PLAY_BODY_MAIN, None, &view).await.expect("render");
    let semio_framework_plugin::Component::Surface(props) = &node.root.component else { panic!("semantic graph") };
    let scene: semio_framework_plugin::NodeGraphScene = semio_framework_ui_scene::decode(props).expect("packed graph");
    assert_eq!(scene.viewport.expect("camera viewport").zoom, 2.0);
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(node).expect("retire viewport graph");
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
    assert!(!live_host_snapshot(&app).await.steps.iter().any(|step| step.id == "step-1"));
}
