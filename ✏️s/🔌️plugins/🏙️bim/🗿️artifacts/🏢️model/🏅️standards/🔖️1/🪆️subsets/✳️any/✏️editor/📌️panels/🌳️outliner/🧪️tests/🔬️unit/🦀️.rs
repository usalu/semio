use super::*;
use semio_framework_plugin::ViewModel;
use semio_framework_ui_locale::{Locale, Terminology};

fn demo() -> (ModelSnapshot, ModelInference) {
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let inference = crate::editor::bim::inference::with_inference(None, &snapshot, Clone::clone);
    (snapshot, inference)
}

fn text(snapshot: &ModelSnapshot, inference: &ModelInference, locale: Locale) -> String {
    let view = ViewModel::new(locale, Terminology::Native);
    let node = render(snapshot, inference, crate::editor::bim::terminology::bim_labels(&view), &TreeWindows::for_body(&view, BODY_KEY)).expect("the outliner renders");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("projects")
}

#[semio_framework_async_macros::async_test]
async fn the_tree_nests_site_building_storeys_and_the_element_groups() {
    let (snapshot, inference) = demo();
    let rendered = text(&snapshot, &inference, Locale::En);
    for expected in ["Demo House", "Plot", "House", "Ground", "First", "+0.00 m", "+3.00 m", "Walls", "South", "East", "8.00 m"] {
        assert!(rendered.contains(expected), "the outliner shows '{expected}': {rendered}");
    }
    let position = |needle: &str| rendered.find(needle).unwrap_or_else(|| panic!("{needle} in the tree"));
    assert!(position("Plot") < position("House") && position("House") < position("Ground") && position("Ground") < position("First"), "storeys follow the hierarchy in level order");
}

#[semio_framework_async_macros::async_test]
async fn the_labels_follow_the_locale() {
    let (snapshot, inference) = demo();
    let german = text(&snapshot, &inference, Locale::De);
    assert!(german.contains("Wände") && german.contains("Geschoss hinzufügen"), "{german}");
    assert!(!german.contains("Add Storey"));
}

#[semio_framework_async_macros::async_test]
async fn an_empty_model_offers_its_add_rows_and_says_it_is_empty() {
    let rendered = text(&ModelSnapshot::default(), &ModelInference::default(), Locale::En);
    assert!(rendered.contains("The model is empty") && rendered.contains("Add Site"), "{rendered}");
}

#[semio_framework_async_macros::async_test]
async fn a_huge_group_stays_closed_and_an_opened_one_materialises_only_the_requested_slice() {
    let (mut snapshot, inference) = demo();
    let template = snapshot.walls["w-south"].clone();
    for index in 0..600 {
        snapshot.walls.insert(format!("bulk-{index:04}"), crate::Wall { name: format!("Bulk {index:04}"), ..template.clone() });
    }
    assert_eq!(text(&snapshot, &inference, Locale::En).matches("Bulk ").count(), 0, "a group over the open limit is not built until the host opens it");
    let group = ["bim-outliner.model", "site-1", "bldg-1", "st-ground", "st-ground::wall"].join(semio_framework_ui_contract::TREE_WINDOW_PATH_SEPARATOR);
    let mut view = ViewModel::new(Locale::En, Terminology::Native);
    view.tree_windows = vec![semio_framework_plugin::TreeWindowRequest { body_key: BODY_KEY.into(), node_key: group, open: Some(true), offset: 100, rows: 20 }];
    let windows = TreeWindows::for_body(&view, BODY_KEY);
    let node = render(&snapshot, &inference, crate::editor::bim::terminology::bim_labels(&view), &windows).expect("renders");
    let rendered = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("projects");
    let materialised = rendered.matches("Bulk ").count();
    assert!((1..=60).contains(&materialised), "{materialised} of 600 rows were built for a 20-row request");
}
