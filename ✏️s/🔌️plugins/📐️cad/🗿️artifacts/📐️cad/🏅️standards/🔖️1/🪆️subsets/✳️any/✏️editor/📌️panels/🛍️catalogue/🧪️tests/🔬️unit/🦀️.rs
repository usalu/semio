semio_plugin::install_fixture_heap_witness!();
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
#[path = "../../../../🧪️tests/🧰️support/🦀️.rs"]
mod context;
use semio_s_artifact_cad_cad_panels::catalogue::*;
use semio_s_artifact_cad_cad_kit::{editor::cad::terminology::typology_label, editor::cad::terminology::CadLabels};
use semio_s_artifact_cad_cad_kit::{cad_action, cad_tree_item, ui_label, ui_value_map, ui_value_text, TYPOLOGY_CATALOG};
use semio_plugin::BuiltNode;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::PanelGroup;
use semio_plugin::PanelTabDefinition;
use semio_plugin::PanelTabKind;
use semio_plugin::PanelTreeBuilder;
use semio_plugin::TreeWindows;
use semio_plugin::UiAssemblyResult;
use semio_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID;
use semio_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL;
use semio_s_artifact_cad_cad_config::CadConfig;
use crate::context::*;
use semio_s_artifact_cad_cad_editor::CadPlayApp;
use semio_s_artifact_cad_cad_inferences::default_document;
use semio_plugin::ArtifactView;
use semio_ui_locale::Locale;
use semio_plugin::ViewModel;

#[semio_framework_async_macros::async_test]
async fn cad_labels_translate_catalogue_typologies_in_german() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let config = CadConfig::default();
    let view_state = ViewModel { locale: Locale::De, ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) };
    let node = render_direct(&app, CAD_PLAY_BODY_CATALOGUE, &doc, &config, &view_state).expect("CAD UI assembly");
    let json = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::ComponentTree::new(node)).expect("fixture projection");
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
