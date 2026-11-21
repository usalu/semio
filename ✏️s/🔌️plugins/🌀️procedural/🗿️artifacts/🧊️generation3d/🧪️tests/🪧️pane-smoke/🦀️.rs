//! 🪧️ Generator pane smoke: the bundled hexagonal mushroom column loads through the example command, one `AddGeneration` applies, undo, redo and a second undo restore it and the app closes, in en and de, through its exact retained close.

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;


#[path = "../🔬️app-fixture/🦀️.rs"]
mod app_fixture;
use app_fixture as context;
use semio_s_artifact_procedural_3d_editor::*;
use semio_s_artifact_procedural_3d_editor_commands::{add_generation, set_active_example};

#[semio_framework_async_macros::async_test]
async fn example_configuration_publishes_before_the_document_load() {
    let _serial = semio_s_artifact_procedural_3d_testkit::test_serial::lock();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎨️example-publication.json")).expect("example publication fixture");
    let command = Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample {
        example_id: fixture["exampleId"].as_str().expect("example id").into(),
    });
    let mut app = context::app().await;
    let _host_lifetime = semio_plugin::artifact_app_laws::arm_host_lifetime(app.1.cx());
    let receipt = context::dispatch(&mut app, command).await;
    let lanes: Vec<_> = receipt.lanes.iter().map(|lane| format!("{lane:?}")).collect();
    let mut observed = lanes.iter();
    for expected in fixture["lanes"].as_array().expect("ordered result lanes") {
        let expected = expected.as_str().expect("result lane");
        assert!(observed.any(|lane| lane == expected), "expected {expected} in {lanes:?}");
    }
    assert_eq!(receipt.completions as u64, fixture["completions"].as_u64().expect("completion count"));
    assert!(receipt.effects.iter().any(|effect| matches!(effect, semio_plugin::Effect::LoadDocument { .. })));
}

/// 🌀️ LAW: the Generator pane loads its bundled example, adds a generation, undoes, redoes and closes in en and de with the store drop witness satisfied.
#[semio_framework_async_macros::async_test]
async fn generator_pane_loads_applies_undoes_and_closes() {
    let _serial = semio_s_artifact_procedural_3d_testkit::test_serial::lock();
    for locale in semio_plugin::artifact_app_laws::PANE_SMOKE_LOCALES {
        let mut app = context::app().await;
        let view = semio_plugin::ViewModel::new(locale, semio_ui_locale::Terminology::Native);
        let labels = semio_s_artifact_procedural_3d_terminology::generation3d_labels(&view);
        let language = semio_plugin::artifact_app_laws::PaneSmokeLanguage { body_key: semio_s_artifact_procedural_3d_editor_panels::catalogue::GENERATION_3D_PLAY_BODY_CATALOGUE, catalog: vec![labels.widgets.as_str().into()], view };
        let example = Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "hexagonal-mushroom-column".into() });
        let command = Generation3dCommand::AddGeneration(add_generation::AddGeneration {});
        let (loaded, applied) = semio_plugin::artifact_app_laws::assert_pane_smoke_by_example(&mut app.0, &language, example, command, |app| { let read = context::snapshot(app); (read.generation.generations.len(), read.host_snapshot.widgets.len()) }, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
        assert!(loaded.1 > 0, "the loaded example carries its widgets");
        assert_eq!(applied.0, loaded.0 + 1);
    }
}
