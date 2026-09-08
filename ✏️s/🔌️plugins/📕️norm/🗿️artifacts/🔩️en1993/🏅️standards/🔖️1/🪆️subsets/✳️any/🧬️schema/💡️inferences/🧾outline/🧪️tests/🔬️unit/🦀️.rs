
use super::*;

#[semio_framework_async_macros::async_test]
async fn outline_field_count_matches_section_outline_length() {
    let outline = En1993Outline::compute(&En1993Snapshot::default());
    assert_eq!(outline.field_count as usize, outline.section_outline.len());
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = En1993Snapshot::default();
    assert_eq!(En1993Outline::compute(&snapshot), En1993Outline::compute(&snapshot));
}
