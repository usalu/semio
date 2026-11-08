//! 🪧️ Statik pane smoke: the bundled concrete forest (the page default) loads through `setActiveExample`, one `AddNode` applies, undo, redo and a second undo restore it and the app closes, in en and de, through its exact retained close. See [`super`].

use crate::context::fem3d_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::{add_node, set_active_example};

/// 🏗️ LAW: the Statik pane loads its bundled concrete forest structure, adds a node, undoes, redoes and closes in en and de with the store drop witness satisfied.
#[semio_framework_async_macros::async_test]
async fn statik_pane_loads_applies_undoes_and_closes() {
    for locale in semio_plugin::artifact_app_laws::PANE_SMOKE_LOCALES {
        let mut app = fem3d_app();
        let mut view = semio_framework::ViewModel::new(locale, semio_ui_locale::Terminology::Native);
        view.focused_window_id = Some(String::from("fem3d-main"));
        let labels = semio_s_artifact_fem_3d_editor_terminology::editor::fem3d::terminology::fem3d_labels(&view);
        let language = semio_plugin::artifact_app_laws::PaneSmokeLanguage { body_key: semio_s_artifact_fem_3d_editor_panels::editor::fem3d::panels::artifact::BODY_KEY, catalog: vec![labels.nodes.as_str().into(), labels.elements.as_str().into()], view: view.for_panel() };
        let example = Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_fem_3d_io::examples::concrete_forest::ID.into() });
        let command = Fem3dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0, z: 3.0 });
        let (loaded, applied) = semio_plugin::artifact_app_laws::assert_pane_smoke_by_example(&mut app.0, &language, example, command, |app| app.snapshot().expect("snapshot"), semio_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
        assert!(!loaded.nodes.is_empty(), "the concrete forest carries its structure");
        assert_eq!(applied.nodes.len(), loaded.nodes.len() + 1);
    }
}

/// 🌐️ LAW: every Statik panel body named by the case data renders non-empty in each declared locale, and the locales render different text. Each locale renders on its own freshly mounted app because a mounted app retains its first stationary render tree.
#[semio_framework_async_macros::async_test]
async fn statik_pane_body_renders_in_every_locale() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("🧫️cases/🔣️.json")).expect("pane smoke case data");
    for body in cases["bodies"].as_array().expect("bodies") {
        let key = body.as_str().expect("body key");
        let mut rendered = Vec::new();
        for locale in cases["locales"].as_array().expect("locales") {
            let locale = match locale.as_str().expect("locale") {
                "en" => semio_ui_locale::Locale::En,
                "de" => semio_ui_locale::Locale::De,
                other => panic!("undeclared locale {other}"),
            };
            let mut app = fem3d_app();
            let mut view = semio_framework::ViewModel::new(locale, semio_ui_locale::Terminology::Native);
            view.focused_window_id = Some(String::from("fem3d-main"));
            let text = semio_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, key, &view.for_panel(), app.1.cx()).await.expect("render panel body");
            assert!(text.len() as u64 >= cases["minimumRenderedBytes"].as_u64().expect("minimum"), "{key} renders a non-empty body in {locale:?}");
            rendered.push(text);
        }
        if cases["localesMustDiffer"].as_bool() == Some(true) {
            assert_ne!(rendered[0], rendered[1], "{key} renders different text per locale");
        }
    }
}

/// 🧾️ LAW: the allocated `Effect::LoadDocument` that `setActiveExample` publishes is retired through the app-owned retirement authority one funded step at a time; every step fits its grant and the receipts add up exactly to the original payload capacities plus the cursor scaffold born on the way.
#[test]
fn statik_pane_load_document_effect_retires_with_exact_receipts() {
    use semio_framework_value::{retained_clone::RetainedCloneGrant, retirement::controlled::ControlledRetirement};
    let (mut pack, mut spr) = (Vec::<u8>::with_capacity(96), Vec::<u8>::with_capacity(40));
    pack.extend_from_slice(b"pack");
    spr.extend_from_slice(b"spr");
    let payload = pack.capacity() + spr.capacity();
    let effect = semio_framework::Effect::LoadDocument { pack, spr };
    assert!(semio_plugin::CompletionEffect::supported(&effect));
    let quoted = semio_plugin::CompletionEffect::original_birth(&effect).expect("known effect quotes its birth");
    let mut owner = ControlledRetirement::new(semio_plugin::CompletionEffect(effect)).unwrap_or_else(|(error, _)| panic!("allocated load-document admission refused: {error:?}"));
    let (mut born, mut released, mut steps) = (0usize, 0usize, 0usize);
    while !owner.terminal_is_empty() {
        steps += 1;
        assert!(steps < 10_000, "load-document retirement must terminate");
        let demand = owner.selected_demand(4096).expect("quoted step");
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        let progress = owner.step(grant).expect("funded step").progress();
        assert!(progress.fits(grant));
        born += progress.retained_capacity_bytes;
        released += progress.released_bytes;
    }
    assert!(born >= quoted, "the quoted first-step birth is part of the born scaffold");
    assert_eq!(released, payload + born);
}
