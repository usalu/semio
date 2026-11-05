//! 🪧️ Bearbeiten pane smoke: the bundled process3d concrete forest (the page default) loads, one `AddStep` applies, undo restores the loaded snapshot and the app closes through its exact retained close.

use super::*;

/// 🔨️ LAW: the Bearbeiten pane loads its bundled concrete forest document, adds a step, undoes it and closes with the store drop witness satisfied.
#[semio_framework_async_macros::async_test]
async fn bearbeiten_pane_loads_applies_undoes_and_closes() {
    let mut app = app();
    let command = Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None });
    let (loaded, applied) = artifact_app_laws::assert_pane_smoke(&mut app.0, semio_s_artifact_process_process3d_examples::examples::concrete_forest::PRIMARY_TEXT, command, |app| app.snapshot().expect("snapshot"), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut artifact_app_laws::fixture_identity(), app.1.cx()).await;
    assert_eq!(loaded.stock_label, "Hexagonal Cut Concrete Forest Left");
    assert_ne!(applied.steps, loaded.steps);
}
