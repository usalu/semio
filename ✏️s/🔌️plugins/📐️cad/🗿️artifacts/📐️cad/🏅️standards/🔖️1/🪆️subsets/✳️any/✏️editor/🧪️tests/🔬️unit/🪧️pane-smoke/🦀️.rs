//! 🪧️ Koordinator pane smoke: the bundled CAD demo loads through `setActiveExample`, one `AddNode` applies, undo, redo and a second undo restore it and the app closes, in every smoke language, through its exact retained close. See [`super`].

use super::*;

/// 📐️ LAW: the Koordinator pane loads its bundled demo model, adds a node, undoes, redoes and closes in en and de with the store drop witness satisfied and its catalog strings rendered.
#[semio_framework_async_macros::async_test]
async fn koordinator_pane_loads_applies_undoes_and_closes() {
    for locale in semio_plugin::artifact_app_laws::PANE_SMOKE_LOCALES {
        let mut app = new_app().await;
        let view = ViewModel::new(locale, semio_ui_locale::Terminology::Native);
        let labels = cad_labels(&view);
        let language = semio_plugin::artifact_app_laws::PaneSmokeLanguage { body_key: document::CAD_PLAY_BODY_ARTIFACT, catalog: vec![labels.pane_shape.as_str().into(), labels.nodes.as_str().into()], view };
        let command = CadCommand::AddNode(add_node::AddNode { kind: "solid".into() });
        let example = CadCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_cad_cad_examples::ID.into() });
        let (loaded, applied) = semio_plugin::artifact_app_laws::assert_pane_smoke_by_example(&mut app.0, &language, example, command, |app| app.snapshot().expect("snapshot"), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
        assert_eq!(applied.nodes.len(), loaded.nodes.len() + 1);
    }
}
