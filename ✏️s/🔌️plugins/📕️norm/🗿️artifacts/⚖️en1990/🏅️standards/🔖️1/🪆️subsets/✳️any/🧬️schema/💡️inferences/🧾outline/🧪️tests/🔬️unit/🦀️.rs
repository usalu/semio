use super::*;

#[semio_framework_async_macros::async_test]
async fn outline_field_count_matches_section_outline_length() {
    let outline = En1990Outline::compute(&En1990Snapshot::default());
    assert_eq!(outline.field_count as usize, outline.section_outline.len());
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = En1990Snapshot::default();
    assert_eq!(En1990Outline::compute(&snapshot), En1990Outline::compute(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn outline_counts_checks_from_norm_computation() {
    let outline = En1990Outline::compute(&En1990Snapshot::default());
    assert!(outline.check_count > 0);
    assert!(outline.pass_count <= outline.check_count, "pass count cannot exceed total checks");
    assert!(!outline.governing_clause.is_empty());
    assert!(outline.governing_utilization > 0.0);
}
