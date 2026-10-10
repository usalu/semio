use super::*;
use crate::editor::bim::entities::components::tests::{furnished, placed};
use protocol::Inference;
use semio_framework_plugin::ViewModel;
use semio_framework_ui_locale::{Locale, Terminology};

fn text(snapshot: &ModelSnapshot, library: &[&str], locale: Locale) -> String {
    let own: Vec<String> = library.iter().map(|id| id.to_string()).collect();
    let view = ViewModel::new(locale, Terminology::Native);
    let inference = ModelInference::infer(snapshot).expect("infers");
    let node = render(snapshot, &inference, crate::editor::bim::terminology::bim_labels(&view), &own, &TreeWindows::for_body(&view, BODY_KEY)).expect("the family browser renders");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("projects")
}

#[semio_framework_async_macros::async_test]
async fn the_placeable_families_are_listed_by_category_and_profiles_are_left_out() {
    let rendered = text(&furnished(), &[], Locale::En);
    for expected in ["Table", "Basin", "Broken", "Furniture", "Plumbing", "Generic", "bim-families.search.input"] {
        assert!(rendered.contains(expected), "the browser shows '{expected}': {rendered}");
    }
    assert!(!rendered.contains("HEA 200") && !rendered.contains("Pipe 114"), "profiles cannot be placed: {rendered}");
    assert!(rendered.find("Table").expect("table") < rendered.find("Basin").expect("basin"), "furniture comes before plumbing");
}

#[semio_framework_async_macros::async_test]
async fn the_categories_and_the_search_follow_the_language() {
    let german = text(&furnished(), &[], Locale::De);
    for expected in ["Möbel", "Sanitär", "Familien suchen"] {
        assert!(german.contains(expected), "'{expected}': {german}");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_selected_family_shows_its_measures_and_the_rows_that_place_or_open_it() {
    let rendered = text(&furnished(), &["fam-basin"], Locale::En);
    for expected in ["Selected family", "bim-families.detail.size", "0.500 × 0.400 × 0.200 m", "Place Basin", "Open Basin in the family editor", "bim-families.place.fam-basin", "bim-families.open.fam-basin", "Matches"] {
        assert!(rendered.contains(expected), "'{expected}': {rendered}");
    }
    let german = text(&furnished(), &["fam-basin"], Locale::De);
    assert!(german.contains("Basin platzieren") && german.contains("Basin im Familieneditor öffnen") && german.contains("Gewählte Familie"), "{german}");
    assert!(!text(&furnished(), &[], Locale::En).contains("Selected family"), "nothing selected, no details");
    assert!(!text(&furnished(), &["fam-hea"], Locale::En).contains("Selected family"), "a profile is no browser selection");
}

#[semio_framework_async_macros::async_test]
async fn a_model_without_a_placeable_family_says_so() {
    let mut bare = placed();
    bare.components.clear();
    bare.families.retain(|_, family| family.category == crate::FamilyCategory::Profile);
    assert!(text(&bare, &[], Locale::En).contains("The model has no family to place"));
    assert!(text(&bare, &[], Locale::De).contains("keine Familie zum Platzieren"));
}

#[semio_framework_async_macros::async_test]
async fn the_size_is_the_union_of_the_visible_solids_in_the_family_frame() {
    let snapshot = furnished();
    let inference = ModelInference::infer(&snapshot).expect("infers");
    let table = size_of(&inference.families["fam-table"]).expect("a size");
    assert!((table[0] - 1.61).abs() < 1e-9 && (table[1] - 0.81).abs() < 1e-9 && table[2] > 0.7, "{table:?}");
    assert_eq!(size_of(&FamilyValue::default()), None);
    assert_eq!(selected(&snapshot, &["fam-hea".to_string(), "fam-basin".to_string(), "x".to_string()]), vec!["fam-basin"]);
}
