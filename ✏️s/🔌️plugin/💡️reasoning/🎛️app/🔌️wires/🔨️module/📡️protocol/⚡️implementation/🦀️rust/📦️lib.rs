//! ⚖️ Reasoning wires app — binary command protocol surface + laws (constitutional: protocol).

use reasoning_wires_op::MindmapWiresOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `MindmapWiresOperation` to its binary command form.
pub fn encode_op(operation: &MindmapWiresOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `MindmapWiresOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<MindmapWiresOperation, protocol::ProtocolError> {
    MindmapWiresOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use reasoning_wires::MindmapWiresDocument;
    use serde_json::json;

    /// 🗄️ Local envelope/store alias for the whole-store tests below — mirrors the `pub type
    /// MindmapWiresEnvelope`/`MindmapWiresStore` the pre-split `reasoning_mindmap` crate exported,
    /// scoped here since this is the only crate that still needs it after the constitutional split.
    type MindmapWiresStore = store::DocumentStore<MindmapWiresDocument, MindmapWiresOperation>;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let node = json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] });
        let operation = MindmapWiresOperation::AddNode { node };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn store_applies_node_add() {
        let mut store = MindmapWiresStore::new(store::create_document_envelope(
            reasoning_wires::MINDMAP_WIRES_SCHEMA,
            "mindmap-wires",
            reasoning_wires_engine::empty_mindmap_wires_document(),
            None,
        ));
        let node = json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] });
        store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![MindmapWiresOperation::AddNode { node }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").board_fixture.get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
    }

    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = MindmapWiresStore::new(store::create_document_envelope(
            reasoning_wires::MINDMAP_WIRES_SCHEMA,
            "mindmap-wires",
            reasoning_wires_engine::empty_mindmap_wires_document(),
            None,
        ));
        let node = json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] });
        store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![MindmapWiresOperation::AddNode { node }],
                description: None,
            })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
