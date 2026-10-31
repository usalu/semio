//! 🪧️ Koordinator pane smoke: the bundled CAD demo loads, one `AddNode` applies, undo restores the loaded snapshot and the app closes through its exact retained close. See [`super`].

use super::*;

/// 📐️ LAW: the Koordinator pane loads its bundled demo model, adds a node, undoes it and closes with the store drop witness satisfied.
#[semio_framework_async_macros::async_test]
async fn koordinator_pane_loads_applies_undoes_and_closes() {
    let mut app = new_app().await;
    let command = CadCommand::AddNode(add_node::AddNode { kind: "solid".into() });
    let (loaded, applied) = semio_framework_plugin::artifact_app_laws::assert_pane_smoke(&mut app.0, semio_s_artifact_cad_cad_examples::PRIMARY_TEXT, command, |app| app.snapshot().expect("snapshot"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    assert_eq!(applied.nodes.len(), loaded.nodes.len() + 1);
}
