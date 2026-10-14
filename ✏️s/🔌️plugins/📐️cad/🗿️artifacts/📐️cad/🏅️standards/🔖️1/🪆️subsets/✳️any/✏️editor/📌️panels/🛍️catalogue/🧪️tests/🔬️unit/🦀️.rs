use semio_s_artifact_cad_cad_panels::catalogue::*;
use semio_s_artifact_cad_cad_config::CadConfig;
use semio_s_artifact_cad_cad_editor::unit_tests::context::*;
use semio_s_artifact_cad_cad_editor::CadPlayApp;
use semio_s_artifact_cad_cad_inferences::default_document;
use semio_framework_plugin::ArtifactView;
use semio_framework_ui_locale::Locale;
use semio_framework_plugin::ViewModel;

#[semio_framework_async_macros::async_test]
async fn cad_labels_translate_catalogue_typologies_in_german() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let config = CadConfig::default();
    let view_state = ViewModel { locale: Locale::De, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let node = render_direct(&app, CAD_PLAY_BODY_CATALOGUE, &doc, &config, &view_state).expect("CAD UI assembly");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(node)).expect("fixture projection");
    assert!(json.contains("Typologien"));
    assert!(json.contains("Quader"));
    assert!(json.contains("Platte"));
    assert!(json.contains("Stütze"));
    assert!(json.contains("Träger"));
    assert!(json.contains("Wand"));
    assert!(json.contains("Außenwand"));
    assert!(!json.contains("\"Slab\""));
    assert!(!json.contains("\"Balken\""));
}
