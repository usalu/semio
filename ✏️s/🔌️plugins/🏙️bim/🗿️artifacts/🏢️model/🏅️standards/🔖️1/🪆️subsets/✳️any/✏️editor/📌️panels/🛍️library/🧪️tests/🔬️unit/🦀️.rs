use super::*;
use semio_framework_plugin::ViewModel;
use semio_framework_ui_locale::{Locale, Terminology};

fn text(snapshot: &ModelSnapshot, locale: Locale) -> String {
    let view = ViewModel::new(locale, Terminology::Native);
    let node = render(snapshot, crate::editor::bim::terminology::bim_labels(&view), &TreeWindows::for_body(&view, BODY_KEY)).expect("the library renders");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("projects")
}

fn demo() -> ModelSnapshot {
    crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot()
}

#[semio_framework_async_macros::async_test]
async fn every_family_is_a_section_with_its_entries() {
    let rendered = text(&demo(), Locale::En);
    for expected in ["Materials", "Brick", "Mineral Wool", "Wall types", "Brick 300", "Slab types", "Roof types", "Column types", "Beam types", "Window types", "Door types"] {
        assert!(rendered.contains(expected), "the library shows '{expected}': {rendered}");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_sections_are_named_in_the_locale() {
    let german = text(&demo(), Locale::De);
    assert!(german.contains("Materialien") && german.contains("Wandtypen") && german.contains("Fenstertypen"), "{german}");
}

#[semio_framework_async_macros::async_test]
async fn add_rows_exist_exactly_for_the_families_with_a_create_mutation() {
    let rendered = text(&demo(), Locale::En);
    for family in ENTITIES.iter().filter(|row| row.library) {
        let label = format!("Add {}", (family.label)(&BimLabels::NATIVE_EN).as_str());
        assert_eq!(rendered.contains(&label), family.create.is_some(), "{label}");
    }
}

#[semio_framework_async_macros::async_test]
async fn an_empty_library_still_renders_every_family() {
    assert!(text(&ModelSnapshot::default(), Locale::En).contains("Wall types"));
}
