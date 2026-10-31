//! 🪧️ Statik pane smoke: the bundled demo loads through `setActiveExample`, one `AddNode` applies, undo restores the loaded snapshot and the app closes through its exact retained close. See [`super`].

use super::*;
use crate::context::fem3d_app;

/// 🏗️ LAW: the Statik pane loads its bundled demo structure, adds a node, undoes it and closes with the store drop witness satisfied.
#[semio_framework_async_macros::async_test]
async fn statik_pane_loads_applies_undoes_and_closes() {
    let mut app = fem3d_app();
    let example = Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_fem_3d_io::examples::demo::ID.into() });
    let command = Fem3dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0, z: 3.0 });
    let (loaded, applied) = semio_framework_plugin::artifact_app_laws::assert_pane_smoke_by_example(&mut app.0, example, command, |app| app.snapshot().expect("snapshot"), semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    assert!(!loaded.solids.is_empty(), "the demo carries its meshed slab");
    assert_eq!(applied.nodes.len(), loaded.nodes.len() + 1);
}
