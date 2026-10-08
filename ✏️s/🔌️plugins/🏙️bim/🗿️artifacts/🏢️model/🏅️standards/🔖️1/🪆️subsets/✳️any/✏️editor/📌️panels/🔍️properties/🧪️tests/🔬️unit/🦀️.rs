use super::*;
use semio_framework_ui_locale::Locale;

fn demo() -> (ModelSnapshot, ModelInference) {
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let inference = crate::editor::bim::inference::with_inference(None, &snapshot, Clone::clone);
    (snapshot, inference)
}

fn text(snapshot: &ModelSnapshot, inference: &ModelInference, elements: &[&str], library: &[&str], locale: Locale) -> String {
    let own = |ids: &[&str]| ids.iter().map(|id| id.to_string()).collect::<Vec<_>>();
    let view = semio_framework_plugin::ViewModel::new(locale, semio_framework_ui_locale::Terminology::Native);
    let node = render(snapshot, inference, &own(elements), &own(library), crate::editor::bim::terminology::bim_labels(&view)).expect("the properties panel renders");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("projects")
}

#[semio_framework_async_macros::async_test]
async fn the_subject_is_the_first_selected_kind_and_only_its_ids() {
    let (snapshot, _) = demo();
    let selection = ["w-south".to_string(), "st-ground".to_string(), "w-east".to_string()];
    let (row, ids) = subject(&snapshot, &selection, &[]).expect("a subject");
    assert_eq!((row.kind, ids), ("wall", vec!["w-south", "w-east"]));
    assert!(subject(&snapshot, &[], &[]).is_none());
    let library = ["wt-300".to_string()];
    assert_eq!(subject(&snapshot, &[], &library).map(|(row, _)| row.kind), Some("wall-type"));
}

#[semio_framework_async_macros::async_test]
async fn a_storey_shows_its_editable_parameters_and_its_inferred_elevations() {
    let (snapshot, inference) = demo();
    let rendered = text(&snapshot, &inference, &["st-first"], &[], Locale::En);
    for expected in ["Storey First", "Name", "Level", "Height", "Building", "Inferred", "Elevation", "3", "Top elevation", "5.8"] {
        assert!(rendered.contains(expected), "the storey panel shows '{expected}': {rendered}");
    }
}

#[semio_framework_async_macros::async_test]
async fn editable_rows_are_labelled_inputs_and_read_only_rows_are_plain_text() {
    let (snapshot, inference) = demo();
    let rendered = text(&snapshot, &inference, &["st-first"], &[], Locale::En);
    assert!(rendered.contains("bim-properties.height.input"), "the height is an input control: {rendered}");
    assert!(!rendered.contains("bim-properties.building.input"), "the building reference has no set mutation, so it is read-only text: {rendered}");
}

#[semio_framework_async_macros::async_test]
async fn a_wall_shows_its_axis_read_only_and_the_inferred_length_and_volume() {
    let (snapshot, inference) = demo();
    let rendered = text(&snapshot, &inference, &["w-south"], &[], Locale::En);
    for expected in ["Wall South", "Axis", "0, 0 → 8, 0", "Length", "Volume", "7.2"] {
        assert!(rendered.contains(expected), "the wall panel shows '{expected}': {rendered}");
    }
}

#[semio_framework_async_macros::async_test]
async fn several_walls_show_a_common_value_or_mixed() {
    let (snapshot, inference) = demo();
    let rendered = text(&snapshot, &inference, &["w-south", "w-east"], &[], Locale::En);
    assert!(rendered.contains("Wall × 2") && rendered.contains("Mixed"), "{rendered}");
}

#[semio_framework_async_macros::async_test]
async fn a_library_type_is_shown_when_no_element_is_selected_and_the_german_panel_is_german() {
    let (snapshot, inference) = demo();
    let rendered = text(&snapshot, &inference, &[], &["wt-300"], Locale::De);
    assert!(rendered.contains("Wandtyp Brick 300") && rendered.contains("Schichten"), "{rendered}");
}

#[semio_framework_async_macros::async_test]
async fn nothing_selected_summarises_the_model_per_kind() {
    let (snapshot, inference) = demo();
    let rendered = text(&snapshot, &inference, &[], &[], Locale::En);
    for expected in ["Nothing selected", "Walls", "4", "Storeys", "2"] {
        assert!(rendered.contains(expected), "the summary shows '{expected}': {rendered}");
    }
}
