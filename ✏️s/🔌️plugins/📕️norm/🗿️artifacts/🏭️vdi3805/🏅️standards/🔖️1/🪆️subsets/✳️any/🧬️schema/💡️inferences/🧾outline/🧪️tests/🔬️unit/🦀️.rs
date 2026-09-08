
use super::*;

#[semio_framework_async_macros::async_test]
async fn outline_field_count_matches_section_outline_length() {
    let outline = Vdi3805Outline::compute(&Vdi3805Snapshot::default());
    assert_eq!(outline.field_count as usize, outline.section_outline.len());
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = Vdi3805Snapshot::default();
    assert_eq!(Vdi3805Outline::compute(&snapshot), Vdi3805Outline::compute(&snapshot));
}
