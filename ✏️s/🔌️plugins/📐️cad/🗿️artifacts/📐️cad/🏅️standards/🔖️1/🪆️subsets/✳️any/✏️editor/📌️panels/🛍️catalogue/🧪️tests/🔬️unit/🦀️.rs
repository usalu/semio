use super::*;
use crate::editor::cad::config::CadConfig;
use crate::editor::cad::testkit::*;
use crate::editor::cad::CadPlayApp;
use crate::standards::v1::subsets::any::schema::inferences::default_document;
use semio_framework_plugin::{ArtifactView, Locale, ViewModel};

#[semio_framework_async_macros::async_test]
async fn cad_labels_translate_catalogue_typologies_in_german() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let config = CadConfig::default();
    let view_state = ViewModel { locale: Locale::De, ..ViewModel::default() };
    let node = render_direct(&app, CAD_PLAY_BODY_CATALOGUE, &doc, &config, &view_state).expect("CAD UI assembly");
    let json = serde_json::to_string(&node).unwrap();
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
