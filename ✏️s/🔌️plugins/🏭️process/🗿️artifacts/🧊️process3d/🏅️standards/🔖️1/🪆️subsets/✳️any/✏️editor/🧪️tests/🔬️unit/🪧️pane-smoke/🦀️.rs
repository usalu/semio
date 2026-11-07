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

/// 🌐️ LAW: the Bearbeiten pane's main window body renders the loaded concrete forest in English and German without a diagnostic.
#[semio_framework_async_macros::async_test]
async fn bearbeiten_pane_renders_the_loaded_forest_in_en_and_de() {
    let mut app = app();
    let booted = app.0.document_text().await.expect("booted document text");
    artifact_app_laws::load_document_text(&mut app.0, &semio_os_kernel::os_store::text_format::ArtifactTextFiles { dsl: semio_s_artifact_process_process3d_examples::examples::concrete_forest::PRIMARY_TEXT.into(), ops: booted.ops }, &mut artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("the bundled example loads");
    let body_key = format!("{}:{}", PROCESS_3D_PLAY_BODY_MAIN, workpiece::PROCESS_3D_PLAY_WINDOW_MAIN);
    for locale in [semio_ui_locale::Locale::En, semio_ui_locale::Locale::De] {
        let view = semio_plugin::ViewModel::new(locale, semio_ui_locale::Terminology::Native);
        let rendered = super::context::render_with_view(&mut app, &body_key, &view);
        assert!(rendered.contains("processed") && !rendered.contains("Unknown body"), "{locale:?}: {rendered}");
    }
}
