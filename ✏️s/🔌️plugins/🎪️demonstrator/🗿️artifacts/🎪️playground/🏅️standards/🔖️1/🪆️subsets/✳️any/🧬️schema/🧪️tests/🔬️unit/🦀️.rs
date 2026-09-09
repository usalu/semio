use super::*;

#[test]
fn empty_snapshot_matches_schema() {
    let snapshot = empty_playground_snapshot();
    assert_eq!(snapshot.schema, crate::PLAYGROUND_DOCUMENT_SCHEMA);
}
