use super::*;

#[test]
fn outline_field_count_matches_section_outline_length() {
    let outline = En1996Outline::compute(&En1996Snapshot::default());
    assert_eq!(outline.field_count as usize, outline.section_outline.len());
}

#[test]
fn outline_counts_walls() {
    let doc = En1996Snapshot::compliant_clay_wall();
    let o = En1996Outline::compute(&doc);
    assert_eq!(o.entry_count, 1);
    assert_eq!(o.field_count, 5);
}
