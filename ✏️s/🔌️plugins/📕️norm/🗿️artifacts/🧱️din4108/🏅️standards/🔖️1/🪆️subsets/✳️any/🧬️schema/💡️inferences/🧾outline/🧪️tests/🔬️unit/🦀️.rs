use super::*;

#[semio_framework_async_macros::async_test]
async fn outline_field_count_matches_section_outline_length() {
    let outline = Din4108Outline::compute(&Din4108Snapshot::default());
    assert_eq!(outline.field_count as usize, outline.section_outline.len());
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = Din4108Snapshot::default();
    assert_eq!(Din4108Outline::compute(&snapshot), Din4108Outline::compute(&snapshot));
}
