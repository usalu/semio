
use super::*;
use crate::CadPaneId;
use crate::editor::cad::config::CadConfig;
use crate::editor::cad::terminology::cad_labels;
use crate::editor::cad::testkit::*;
use crate::editor::cad::{CadPlayApp, CadPlayRuntime, make_object_for_typology};
use crate::standards::v1::subsets::any::io::geometry_import::CadPrimitiveSlot;
use crate::standards::v1::subsets::any::schema::inferences::{CAD_MODEL_DEFINITION_SHAPE, default_document, forest_play_scene};
use semio_framework_plugin::{ArtifactView, PluginApp, ViewModel};

#[semio_framework_async_macros::async_test]
async fn document_lists_nodes() {
    // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: pane object sections
    // render empty at this boundary now (documented gap, see `build_document_tree`'s own doc
    // comment) — `object_tree_item_shows_name_with_kind_as_secondary_label`/
    // `object_tree_item_includes_primitive_children` below cover the real (still-working)?
    // tree-item builder directly instead.
    let mut app = new_app().await;
    let node = app.render(CAD_PLAY_BODY_DOCUMENT, None, &ViewModel::default()).await.expect("render").root;
    let json = serde_json::to_string(&node).unwrap();
    assert!(json.contains("cad-node:"));
}

#[semio_framework_async_macros::async_test]
async fn object_tree_item_shows_name_with_kind_as_secondary_label() {
    let mut object = make_object_for_typology("building.building.beam", 0, CadPaneId::Shape);
    object.label = "U2".into();
    let labels = cad_labels(&CadConfig::default());
    let item = object_tree_item("shape", &object, labels).expect("object tree item");
    let semio_framework_plugin::Component::TreeItem(props) = &item.component else {
        panic!("expected tree item");
    };
    assert_eq!(props.label.0.as_str(), "U2");
    assert_eq!(props.description.as_ref().map(|text| text.as_str()), Some("Beam"));

    let de_config = CadConfig { locale: "de".into(), ..CadConfig::default() };
    let de_labels = cad_labels(&de_config);
    let de_item = object_tree_item("shape", &object, de_labels).expect("German object tree item");
    let semio_framework_plugin::Component::TreeItem(props) = &de_item.component else {
        panic!("expected German tree item");
    };
    assert_eq!(props.description.as_ref().map(|text| text.as_str()), Some("Träger"));
}

#[semio_framework_async_macros::async_test]
async fn object_tree_item_includes_primitive_children() {
    let mut object = make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape);
    object.primitives = vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: "solid-1".into(), kind: "solid".into() }];
    let labels = cad_labels(&CadConfig::default());
    let item = object_tree_item("shape", &object, labels).expect("object tree item");
    let json = serde_json::to_string(&item).unwrap();
    assert!(json.contains("cad-primitive:"));
}

#[semio_framework_async_macros::async_test]
async fn document_tree_selected_and_highlighted_ids_are_none_without_a_reference_selection() {
    // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): mesh object selection/hover is
    // framework-owned now, unreachable at this render boundary — only reference-overlay
    // selection/hover still resolves here (see `document_tree_selected_ids`'s doc comment).
    let scene = default_document();
    let runtime = CadPlayRuntime::default();
    assert_eq!(document_tree_selected_ids(&scene, &runtime).expect("selection assembly"), None);
    assert_eq!(document_tree_highlighted_ids(&scene, &runtime).expect("highlight assembly"), None);
}

#[semio_framework_async_macros::async_test]
async fn document_tree_selected_ids_resolves_reference_selection() {
    let scene = forest_play_scene();
    let runtime = CadPlayRuntime { selected_reference_model_definition_id: Some(CAD_MODEL_DEFINITION_SHAPE.into()), selected_reference_id: Some("ref-concrete-forest".into()), ..CadPlayRuntime::default() };
    let selected = document_tree_selected_ids(&scene, &runtime).expect("selection assembly").expect("selected");
    assert!(selected.iter().any(|id| id == "cad-reference:spatial.shape:ref-concrete-forest"));
}

#[semio_framework_async_macros::async_test]
async fn cad_labels_translate_document_tree_panes_in_german() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let config = CadConfig { locale: "de".into(), ..CadConfig::default() };
    let node = render_direct(&app, CAD_PLAY_BODY_DOCUMENT, &doc, &config).expect("CAD UI assembly");
    let json = serde_json::to_string(&node).unwrap();
    assert!(json.contains("\"Form\""));
    assert!(json.contains("Gebäude"));
    assert!(json.contains("Energie"));
    assert!(json.contains("Tragwerk Klassisch"));
    assert!(json.contains("Referenzen"));
    assert!(json.contains("\"Knoten\""));
    assert!(!json.contains("\"Shape\""));
    assert!(!json.contains("Struktur Klassisch"));
}
