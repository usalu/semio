use super::*;

#[semio_framework_async_macros::async_test]
async fn outline_field_count_matches_section_outline_length() {
    let outline = En1992Outline::compute(&En1992Snapshot::default());
    assert_eq!(outline.field_count as usize, outline.section_outline.len());
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = En1992Snapshot::default();
    assert_eq!(En1992Outline::compute(&snapshot), En1992Outline::compute(&snapshot));
}
