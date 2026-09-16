use super::*;
use crate::editor::cad::config::CadConfig;
use crate::editor::cad::terminology::cad_labels;
use crate::editor::cad::unit_tests::context::*;
use crate::editor::cad::{make_object_for_typology, CadPlayApp, CadPlayRuntime};
use crate::standards::v1::subsets::any::io::geometry_import::CadPrimitiveSlot;
use crate::editor::cad::forest_working_scene;
use crate::standards::v1::subsets::any::schema::inferences::{default_document, forest_play_scene, CAD_MODEL_DEFINITION_SHAPE};
use crate::CadPaneId;
use semio_framework_plugin::{ArtifactView, Locale, PluginApp, ViewModel};

#[semio_framework_async_macros::async_test]
async fn document_lists_nodes() {
    let app = CadPlayApp::default();
    let scene = forest_play_scene();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let node = render_direct(&app, CAD_PLAY_BODY_ARTIFACT, &doc, &CadConfig::default(), &ViewModel::default()).expect("CAD UI assembly");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: node }).expect("fixture projection");
    assert!(json.contains("cad-node:node-root"), "{json}");
    assert!(json.contains("Concrete Forest Left"));
}

/// 🪆️ Every pane's objects come from its composed child's local owner, row ids are the raw object
/// ids the `"cad"` domain (which the tree is bound to) marks selection by, and a pane past the
/// shared argument-arena page closes with a `+N` continuation row that accounts for every object
/// the page left out (`semio_framework_plugin::paged_panel_section`).
#[semio_framework_async_macros::async_test]
async fn document_lists_every_pane_object_bound_to_the_cad_domain() {
    let scene = forest_play_scene();
    let working = forest_working_scene();
    let tree = build_document_tree(&view(scene, CadPlayRuntime::default()), cad_labels(&ViewModel::default())).expect("document tree");
    let semio_framework_plugin::Component::Tree(props) = &tree.component else { panic!("panel tree") };
    assert_eq!(props.interaction_domain.as_ref().map(|domain| domain.as_str()), Some(CAD_INTERACTION_DOMAIN));
    let panes: [(&str, &[crate::standards::v1::subsets::any::io::geometry_import::CadObject]); 4] = [("shape", &working.objects), ("building", &working.building_objects), ("energy", &working.energy_objects), ("structure-classic", &working.structure_classic_objects)];
    let mut listed = 0;
    for (suffix, objects) in panes {
        let section_key = format!("cad-play-document.{suffix}");
        let section = tree.children.iter().find(|child| child.key.as_str() == section_key).unwrap_or_else(|| panic!("section {section_key}"));
        let rows: Vec<&str> = section.children.iter().map(|row| row.key.as_str()).collect();
        let placed = rows.iter().filter(|key| objects.iter().any(|object| object.id == **key)).count();
        let omitted = section.children.iter().find(|row| row.key.as_str() == format!("{section_key}.more")).map_or(0, |more| match &more.component {
            semio_framework_plugin::plugin_app_close_prelude::Component::TreeItem(props) => props.label.0.as_str().trim_start_matches('+').parse::<usize>().expect("continuation count"),
            _ => panic!("continuation row is a tree item"),
        });
        assert_eq!(placed + omitted, objects.len(), "pane {suffix}: rows {rows:?} must account for every object");
        assert!(rows.iter().all(|key| !key.starts_with("cad-object:")), "object rows use raw domain ids: {rows:?}");
        listed += placed;
    }
    assert!(listed > 0, "at least the first page lists objects");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: tree }).expect("fixture projection");
    assert!(json.contains("interactionSelect"), "rows pick through the framework domain");
}

#[semio_framework_async_macros::async_test]
async fn object_tree_item_shows_name_with_kind_as_secondary_label() {
    let mut object = make_object_for_typology("building.building.beam", 0, CadPaneId::Shape);
    object.label = "U2".into();
    let labels = cad_labels(&ViewModel::default());
    let item = object_tree_item("shape", &object, labels).expect("object tree item");
    let semio_framework_plugin::Component::TreeItem(props) = &item.component else {
        panic!("expected tree item");
    };
    assert_eq!(props.label.0.as_str(), "U2");
    assert_eq!(props.description.as_ref().map(|text| text.as_str()), Some("Beam"));

    let de_view = ViewModel { locale: Locale::De, ..ViewModel::default() };
    let de_labels = cad_labels(&de_view);
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
    let labels = cad_labels(&ViewModel::default());
    let item = object_tree_item("shape", &object, labels).expect("object tree item");
    assert_eq!(item.key.as_str(), object.id.as_str(), "object rows are keyed by the raw domain id");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: item }).expect("fixture projection");
    assert!(json.contains("cad-primitive:"));
}

#[semio_framework_async_macros::async_test]
async fn document_tree_selected_and_highlighted_ids_are_none_without_a_reference_selection() {
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
    let config = CadConfig::default();
    let view_state = ViewModel { locale: Locale::De, ..ViewModel::default() };
    let node = render_direct(&app, CAD_PLAY_BODY_ARTIFACT, &doc, &config, &view_state).expect("CAD UI assembly");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: node }).expect("fixture projection");
    assert!(json.contains("\"Form\""));
    assert!(json.contains("Gebäude"));
    assert!(json.contains("Energie"));
    assert!(json.contains("Tragwerk Klassisch"));
    assert!(json.contains("Referenzen"));
    assert!(json.contains("\"Knoten\""));
    assert!(!json.contains("\"Shape\""));
    assert!(!json.contains("Struktur Klassisch"));
}
