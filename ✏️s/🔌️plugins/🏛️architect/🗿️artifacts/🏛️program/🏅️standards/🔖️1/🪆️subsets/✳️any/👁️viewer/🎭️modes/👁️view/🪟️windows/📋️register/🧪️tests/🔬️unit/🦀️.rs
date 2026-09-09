use super::*;
use crate::{empty_plugin, sample_plugin};

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_table_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, ARCHITECT_VIEW_BODY_REGISTER);
    assert!(matches!(definition.surface_kind, SurfaceKind::Table));
}

#[semio_framework_async_macros::async_test]
async fn the_overview_lists_every_non_empty_register_with_its_counts() {
    let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(render(&sample_plugin()).expect("render"))).expect("retire viewer tree");
    assert!(json.contains("\"elements\""));
    assert!(json.contains("Total: 2"));
}

#[semio_framework_async_macros::async_test]
async fn an_empty_program_renders_the_placeholder() {
    let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(render(&empty_plugin()).expect("render"))).expect("retire viewer tree");
    assert!(json.contains("No entities in this program yet."));
}
