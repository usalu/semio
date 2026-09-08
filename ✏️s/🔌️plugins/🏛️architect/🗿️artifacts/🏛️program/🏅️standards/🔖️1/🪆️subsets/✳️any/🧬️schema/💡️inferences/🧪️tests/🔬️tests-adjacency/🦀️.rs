
use super::*;
use crate::sample_plugin;

#[semio_framework_async_macros::async_test]
async fn sample_plugin_matrix_has_one_cell() {
    let program = sample_plugin();
    let matrix = adjacency_matrix(&program);
    assert_eq!(matrix.element_ids.len(), 2);
    let populated: usize = matrix.cells.iter().flat_map(|row| row.iter()).filter(|cell| cell.is_some()).count();
    assert_eq!(populated, 1);
}

#[semio_framework_async_macros::async_test]
async fn detects_distance_min_max_violation() {
    let mut program = sample_plugin();
    program.adjacencies[0].distance_min_m = Some(10.0);
    program.adjacencies[0].distance_max_m = Some(5.0);
    let conflicts = detect_adjacency_conflicts(&program);
    assert!(conflicts.iter().any(|c| c.message.contains("distance_min")));
}
