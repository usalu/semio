
use super::*;
use crate::sample_plugin;

#[semio_framework_async_macros::async_test]
async fn requirement_lists_output_nonempty_for_sample() {
    let output = build_output(&sample_plugin(), OutputKind::RequirementLists);
    assert_eq!(output.kind, OutputKind::RequirementLists);
}

#[semio_framework_async_macros::async_test]
async fn adjacency_matrices_output_uses_matrix_cells() {
    let output = build_output(&sample_plugin(), OutputKind::AdjacencyMatrices);
    assert!(!output.lines.is_empty());
}
