
use super::*;

#[semio_framework_async_macros::async_test]
async fn outline_field_count_matches_section_outline_length() {
    let outline = Din18599Outline::compute(&Din18599Snapshot::default());
    assert_eq!(outline.field_count as usize, outline.section_outline.len());
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = Din18599Snapshot::default();
    assert_eq!(Din18599Outline::compute(&snapshot), Din18599Outline::compute(&snapshot));
}
