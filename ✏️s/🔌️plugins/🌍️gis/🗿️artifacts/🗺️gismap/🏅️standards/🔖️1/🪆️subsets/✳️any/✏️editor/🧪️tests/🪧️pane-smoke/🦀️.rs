//! 🪧️ Verfolgen pane smoke: the bundled GIS map demo loads through `setActiveExample`, one `AddFeature` applies, undo, redo and a second undo restore it and the app closes, in en and de, through its exact retained close.

use crate::unit_tests::context::app;
use semio_s_artifact_gis_gismap_gis2d::editor::gis2d::commands::example::set_active_example;
use semio_s_artifact_gis_gismap_gis2d::editor::gis2d::commands::features::add_feature;
use semio_s_artifact_gis_gismap_gis2d::editor::gis2d::Gis2dCommand;

/// 🌍️ LAW: the Verfolgen pane loads its bundled demo map, adds one position, undoes, redoes and closes in en and de with the store drop witness satisfied.
#[semio_framework_async_macros::async_test]
async fn verfolgen_pane_loads_applies_undoes_and_closes() {
    for locale in semio_plugin::artifact_app_laws::PANE_SMOKE_LOCALES {
        let mut app = app().await;
        let view = semio_plugin::ViewModel::new(locale, semio_ui_locale::Terminology::Native);
        let labels = semio_s_artifact_gis_gismap_gis2d::editor::gis2d::terminology::gis2d_labels(&view);
        let language = semio_plugin::artifact_app_laws::PaneSmokeLanguage { body_key: semio_s_artifact_gis_gismap_gis2d::editor::gis2d::panels::artifact::GIS2D_PLAY_BODY_ARTIFACT, catalog: vec![labels.layer_water.as_str().into(), labels.layer_roads.as_str().into()], view };
        let command = Gis2dCommand::AddFeature(add_feature::AddFeature { collection: "positions".into(), label: "Depot".into(), lon: 7.5, lat: 51.0, span: 0.01 });
        let example = Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_gis_gismap_gis2d::editor::gis2d::commands::example::DEFAULT_EXAMPLE_ID.into() });
        let (loaded, applied) = semio_plugin::artifact_app_laws::assert_pane_smoke_by_example(&mut app.0, &language, example, command, |app| app.snapshot().expect("projection"), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
        assert!(!loaded.positions.is_empty(), "the loaded demo carries its positions");
        assert_eq!(applied.positions.len(), loaded.positions.len() + 1);
    }
}

/// 🌐️ LAW: the Verfolgen map body renders from the loaded demo in every locale of `🧫️cases.json` and carries each case's expected strings.
#[semio_framework_async_macros::async_test]
async fn verfolgen_pane_body_renders_in_every_locale() {
    use semio_plugin::PluginApp as _;
    use semio_ui_locale::{Locale, Terminology};
    let spec: serde_json::Value = serde_json::from_str(include_str!("🧫️cases.json")).expect("case data parses");
    let body_key = spec["bodyKey"].as_str().expect("body key");
    let mut app = app().await;
    let booted = app.0.document_text().await.expect("booted document text");
    semio_plugin::artifact_app_laws::load_document_text(&mut app.0, &semio_os_kernel::os_store::text_format::ArtifactTextFiles { dsl: semio_s_artifact_gis_gismap_demo_asset::DEMO_DSL.into(), ops: booted.ops }, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("the demo loads");
    for case in spec["cases"].as_array().expect("cases") {
        let locale = match case["locale"].as_str().expect("locale") { "en" => Locale::En, "de" => Locale::De, other => panic!("unknown locale {other}") };
        let rendered = semio_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, body_key, &semio_plugin::ViewModel::new(locale, Terminology::Native), app.1.cx()).await.unwrap_or_else(|fault| panic!("render {body_key} {locale:?}: {fault:?}"));
        assert!(!rendered.is_empty());
        for needle in case["contains"].as_array().expect("needles") {
            let needle = needle.as_str().expect("needle");
            assert!(rendered.contains(needle), "{locale:?} body lacks {needle:?}: {rendered}");
        }
    }
    semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("exact retained close");
}
