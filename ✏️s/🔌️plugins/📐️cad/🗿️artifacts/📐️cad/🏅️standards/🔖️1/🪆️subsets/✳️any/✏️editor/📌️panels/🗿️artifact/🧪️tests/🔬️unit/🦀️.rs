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

/// 📄️ Paging law: a section longer than its row quota closes with a `+N` continuation that dispatches
/// `setPanelPage` for the NEXT page, and rendering with that cursor starts the section at the first
/// row the previous page left out. This is the navigation half of `paged_panel_section` the
/// 2026-09-16 CAD end-to-end report recorded as missing.
#[semio_framework_async_macros::async_test]
async fn set_panel_page_advances_the_structure_section() {
    let scene = forest_play_scene();
    let working = forest_working_scene();
    let section_id = "cad-play-document.structure-classic";
    let objects = &working.structure_classic_objects;
    assert!(objects.len() > CAD_SECTION_ROWS, "this law needs a pane past one page, found {}", objects.len());

    let labels = cad_labels(&ViewModel::default());
    let first = build_document_tree(&view(scene.clone(), CadPlayRuntime::default()), labels).expect("page 0 tree");
    let first_section = first.children.iter().find(|child| child.key.as_str() == section_id).expect("structure section");
    let first_rows: Vec<String> = first_section.children.iter().map(|row| row.key.as_str().to_string()).collect();
    assert!(first_rows.contains(&format!("{section_id}.more")), "page 0 closes with a continuation row: {first_rows:?}");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: first }).expect("page 0 projection");
    assert!(json.contains("setPanelPage"), "the continuation row must dispatch setPanelPage: {json}");

    let runtime = CadPlayRuntime { panel_pages: std::collections::BTreeMap::from([(section_id.to_string(), 1)]), ..CadPlayRuntime::default() };
    let second = build_document_tree(&view(scene, runtime), labels).expect("page 1 tree");
    let second_section = second.children.iter().find(|child| child.key.as_str() == section_id).expect("structure section");
    let second_rows: Vec<String> = second_section.children.iter().map(|row| row.key.as_str().to_string()).collect();
    assert_ne!(first_rows, second_rows, "page 1 must not repeat page 0");
    assert_eq!(second_rows.first().map(String::as_str), Some(objects[CAD_SECTION_ROWS].id.as_str()), "page 1 starts at the first row page 0 left out");
}

/// 📄️ A cursor past the section's own last page reads as that last page — a document that shrank
/// under a stale cursor still renders rows instead of an empty section.
#[semio_framework_async_macros::async_test]
async fn a_stale_panel_page_cursor_clamps_to_the_last_page() {
    let pages = std::collections::BTreeMap::from([("section".to_string(), 99u32)]);
    assert_eq!(section_page(&pages, "section", 0), 0, "an empty section is always page 0");
    assert_eq!(section_page(&pages, "section", CAD_SECTION_ROWS + 1), 1, "two pages of entries clamp a runaway cursor to page 1");
    assert_eq!(section_page(&BTreeMap::new(), "section", 100), 0, "an unpaged section starts at page 0");
}
