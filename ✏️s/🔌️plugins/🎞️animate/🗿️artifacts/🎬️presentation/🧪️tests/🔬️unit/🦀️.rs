
use super::*;

#[test]
fn presentation_snapshot_schema_is_animate_presentation() {
    assert_eq!(default_presentation_snapshot().schema, PRESENTATION_DOCUMENT_SCHEMA);
}

#[test]
fn artifact_kind_matches_the_store_schema() {
    assert_eq!(artifact_kind().schema, PRESENTATION_DOCUMENT_SCHEMA);
    assert_eq!(artifact_kind().id, PRESENTATION_DOCUMENT_SCHEMA);
}
