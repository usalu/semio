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

/// 📬️ An untagged host contribution is admitted and published through the mounted command owner.
#[semio_framework_async_macros::async_test]
async fn koordinator_plain_contributions_reach_the_host_lane() {
    let mut app = new_app().await;
    let args = semio_framework_value::DslValue::object([
        ("json".to_owned(), semio_framework_value::DslValue::String("[]".to_owned())),
        ("page".to_owned(), semio_framework_value::DslValue::Number(0_u64.into())),
        ("pageCount".to_owned(), semio_framework_value::DslValue::Number(1_u64.into())),
    ]);
    let result = semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, "setContributions", Some(args), &meta("local"), &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::artifact_app_laws::fixture_mounted_policy().close).await;
    close(&mut app);
    assert!(result.is_ok(), "plain host contribution must not require a press tag: {result:?}");
}
