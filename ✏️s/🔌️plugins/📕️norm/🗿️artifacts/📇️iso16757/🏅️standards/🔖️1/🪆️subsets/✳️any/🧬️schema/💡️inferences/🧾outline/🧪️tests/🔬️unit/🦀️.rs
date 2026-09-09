use super::*;

#[semio_framework_async_macros::async_test]
async fn outline_field_count_matches_section_outline_length() {
    let outline = Iso16757Outline::compute(&Iso16757Snapshot::default());
    assert_eq!(outline.field_count as usize, outline.section_outline.len());
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = Iso16757Snapshot::default();
    assert_eq!(Iso16757Outline::compute(&snapshot), Iso16757Outline::compute(&snapshot));
}
