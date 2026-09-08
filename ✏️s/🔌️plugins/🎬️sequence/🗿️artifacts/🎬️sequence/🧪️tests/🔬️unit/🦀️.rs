
use super::*;

#[semio_framework_async_macros::async_test]
async fn default_snapshot_has_steps() {
    assert_eq!(default_snapshot().to_fixture().steps.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn step_content_round_trips_through_the_composed_child_snapshot() {
    let fixture = default_snapshot().to_fixture();
    let content = sequence_content_snapshot_from_working(&fixture.steps, &fixture.edges);
    let (steps, edges) = working_from_sequence_content_snapshot(&content);
    assert_eq!(steps, fixture.steps);
    assert_eq!(edges, fixture.edges);
}

#[semio_framework_async_macros::async_test]
async fn fixture_projection_rejects_a_wire_only_parent_instead_of_defaulting_empty() {
    let snapshot = default_snapshot();
    assert!(!snapshot.try_to_fixture().expect("owned scene projects").steps.is_empty());
    let bytes = <SequenceSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
    let decoded = <SequenceSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("parent wire decodes");
    assert_eq!(decoded.try_to_fixture(), Err(store::ArtifactChildMaterializationError::Absent));
}

#[semio_framework_async_macros::async_test]
async fn artifact_kind_keeps_the_media_schema_consistent_with_the_store_schema() {
    assert_eq!(artifact_kind().schema, "sequence.sequence");
    assert_eq!(SEQUENCE_DOCUMENT_SCHEMA, "sequence.sequence");
}
