
use super::*;

#[semio_framework_async_macros::async_test]
async fn outline_field_count_matches_section_outline_length() {
    let outline = En1991Outline::compute(&En1991Snapshot::default());
    assert_eq!(outline.field_count as usize, outline.section_outline.len());
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = En1991Snapshot::default();
    assert_eq!(En1991Outline::compute(&snapshot), En1991Outline::compute(&snapshot));
}
