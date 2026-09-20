use super::*;

#[semio_framework_async_macros::async_test]
async fn default_snapshot_has_steps() {
    let fixture = neural_engine::ColdOwner::new(default_snapshot());
    assert_eq!(fixture.to_host_snapshot().steps.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn step_content_round_trips_through_the_composed_child_snapshot() {
    let source = neural_engine::ColdOwner::new(default_snapshot());
    let fixture = neural_engine::ColdOwner::new(source.to_host_snapshot());
    let content = sequence_content_snapshot_from_working(&fixture.steps, &fixture.edges);
    let (steps, edges) = working_from_sequence_content_snapshot(&content);
    let steps = neural_engine::ColdOwner::new(steps);
    assert_eq!(*steps, fixture.steps);
    assert_eq!(edges, fixture.edges);
}

#[semio_framework_async_macros::async_test]
async fn fixture_projection_rejects_a_wire_only_parent_instead_of_defaulting_empty() {
    let snapshot = neural_engine::ColdOwner::new(default_snapshot());
    assert!(!snapshot.try_to_host_snapshot().expect("owned scene projects").steps.is_empty());
    let bytes = <SequenceSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
    let decoded = <SequenceSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("parent wire decodes");
    assert_eq!(decoded.try_to_host_snapshot(), Err(store::ArtifactChildMaterializationError::Absent));
}

#[semio_framework_async_macros::async_test]
async fn artifact_kind_keeps_the_media_schema_consistent_with_the_store_schema() {
    assert_eq!(artifact_kind().schema, "sequence.sequence");
    assert_eq!(SEQUENCE_DOCUMENT_SCHEMA, "sequence.sequence");
}
