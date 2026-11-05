//! 🪧️ Statik pane smoke: the bundled concrete forest (the page default) loads through `setActiveExample`, one `AddNode` applies, undo restores the loaded snapshot and the app closes through its exact retained close. See [`super`].

use crate::context::fem3d_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::{add_node, set_active_example};

/// 🏗️ LAW: the Statik pane loads its bundled concrete forest structure, adds a node, undoes it and closes with the store drop witness satisfied.
#[semio_framework_async_macros::async_test]
async fn statik_pane_loads_applies_undoes_and_closes() {
    let mut app = fem3d_app();
    let example = Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_fem_3d_io::examples::concrete_forest::ID.into() });
    let command = Fem3dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0, z: 3.0 });
    let (loaded, applied) = semio_plugin::artifact_app_laws::assert_pane_smoke_by_example(&mut app.0, example, command, |app| app.snapshot().expect("snapshot"), semio_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    assert!(!loaded.nodes.is_empty(), "the concrete forest carries its structure");
    assert_eq!(applied.nodes.len(), loaded.nodes.len() + 1);
}
