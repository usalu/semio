use super::*;
use crate::{empty_plugin, sample_plugin};

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_adjacency_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, ARCHITECT_BODY_ADJACENCY);
    assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
}

#[semio_framework_async_macros::async_test]
async fn the_matrix_renders_a_triangle_strip_with_element_labels() {
    let json = crate::editor::architect::unit_tests::context::project_render(render(&sample_plugin(), &config::ArchitectAdjacencyWindowConfig::default()));
    assert!(json.contains('▲'));
    assert!(json.contains("Reception"));
}

#[semio_framework_async_macros::async_test]
async fn an_empty_program_renders_the_placeholder() {
    let json = crate::editor::architect::unit_tests::context::project_render(render(&empty_plugin(), &config::ArchitectAdjacencyWindowConfig::default()));
    assert!(json.contains("Add program elements"));
}
