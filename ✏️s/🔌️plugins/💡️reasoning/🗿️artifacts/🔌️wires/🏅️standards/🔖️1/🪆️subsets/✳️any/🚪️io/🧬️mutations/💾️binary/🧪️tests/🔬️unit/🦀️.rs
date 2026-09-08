
use super::*;
use crate::WiresSnapshot;
use crate::mutations::create_node;

/// 🗄️ Local envelope/store alias for the whole-store tests below — mirrors the `pub type
/// MindmapWiresEnvelope`/`MindmapWiresStore` the pre-split `semio_s_mindmap` crate exported,
/// scoped here since this is the only sub-region that still needs it after the taxonomy split.
type MindmapWiresStore = store::ArtifactStore<WiresSnapshot, WiresMutation>;

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let node = dsl::to_dsl_value(&dsl::json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] })).expect("node serializes");
    let operation = create_node(node);
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn store_applies_node_add() {
    let mut store = MindmapWiresStore::new(store::create_document_envelope(crate::MINDMAP_WIRES_SCHEMA, "mindmap-wires", crate::empty_wires_snapshot(), None)).await.expect("valid artifact store fixture");
    let node = dsl::to_dsl_value(&dsl::json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] })).expect("node serializes");
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![create_node(node)], description: None }).await.expect("apply");
    assert_eq!(crate::wires_working_board(&store.snapshot().expect("snapshot")).get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trip_with_operation_applied() {
    let mut store = MindmapWiresStore::new(store::create_document_envelope(crate::MINDMAP_WIRES_SCHEMA, "mindmap-wires", crate::empty_wires_snapshot(), None)).await.expect("valid artifact store fixture");
    let node = dsl::to_dsl_value(&dsl::json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] })).expect("node serializes");
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![create_node(node)], description: None }).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
}

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `WiresMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this
/// file's existing pack round-trip law (same pattern as `dag`'s own
/// `command_envelope_round_trip_holds_for_an_applied_operation`). Uses `create-node`
/// deliberately, not a whole-document replace — a whole-snapshot variant is banned vocabulary
/// and no longer exists on `WiresMutation` (see `📓️taxonomy.md`).
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use protocol::{ArtifactId, Edit, SchemaId};

    let mut store = MindmapWiresStore::new(store::create_document_envelope(crate::MINDMAP_WIRES_SCHEMA, "mindmap-wires", crate::empty_wires_snapshot(), None)).await.expect("valid artifact store fixture");
    let node = dsl::to_dsl_value(&dsl::json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] })).expect("node serializes");
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![create_node(node)], description: None }).await.expect("apply");
    let edit: &Edit<WiresMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<WiresSnapshot, WiresMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}
//#endregion 🔖️CommandEnvelopeTests
