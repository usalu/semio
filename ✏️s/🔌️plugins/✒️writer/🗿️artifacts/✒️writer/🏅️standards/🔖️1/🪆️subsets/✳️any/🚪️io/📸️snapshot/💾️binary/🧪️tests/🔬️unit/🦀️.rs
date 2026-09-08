
use super::*;
use crate::schema;

/// ✍️ Hand-built representative document — used across the artifact's own component tests.
fn jack_snapshot() -> WriterSnapshot {
    crate::writer_snapshot_with_text("writer.document", "jack", "jack", "writer://jack", "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name")
}

#[semio_framework_async_macros::async_test]
async fn writer_projection_dsl_pack_equivalence() {
    let empty = schema::empty_writer_snapshot();
    store::os_store::test_support::assert_dsl_pack_equivalence(&empty);
    let bytes = encode(&empty);
    assert_eq!(decode(&bytes).expect("decode"), empty);

    let jack = jack_snapshot();
    store::os_store::test_support::assert_dsl_pack_equivalence(&jack);
    let bytes = encode(&jack);
    assert_eq!(decode(&bytes).expect("decode"), jack);
}

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `WriterMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
/// existing dsl/pack round-trip law.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::op::WriterMutation;
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{ArtifactCommand, ArtifactStore, create_document_envelope};

    let mut store: ArtifactStore<WriterSnapshot, WriterMutation> = ArtifactStore::new(create_document_envelope("writer.document", "writer", schema::empty_writer_snapshot(), None)).await.expect("valid artifact store fixture");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![WriterMutation::EditText(schema::mutations::EditText { text: "hello".into() })], description: None }).await.expect("apply");
    let edit: &Edit<WriterMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<WriterSnapshot, WriterMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}
//#endregion 🔖️CommandEnvelopeTests
