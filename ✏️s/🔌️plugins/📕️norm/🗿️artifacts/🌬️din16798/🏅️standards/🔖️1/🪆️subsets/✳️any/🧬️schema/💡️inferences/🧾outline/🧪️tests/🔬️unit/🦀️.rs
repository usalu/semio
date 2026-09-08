
use super::*;

#[semio_framework_async_macros::async_test]
async fn outline_field_count_matches_section_outline_length() {
    let outline = Din16798Outline::compute(&Din16798Snapshot::default());
    assert_eq!(outline.field_count as usize, outline.section_outline.len());
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = Din16798Snapshot::default();
    assert_eq!(Din16798Outline::compute(&snapshot), Din16798Outline::compute(&snapshot));
}
