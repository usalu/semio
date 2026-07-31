//! ⚖️ Trinity Jack app — binary command protocol surface + laws (constitutional: protocol).

use protocol::OpBinary;
use trinity_jack_op::Operation;

/// 📦️ Encodes a Trinity graph `Operation` to its binary command form.
pub fn encode_op(operation: &Operation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a Trinity graph `Operation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Operation, protocol::ProtocolError> {
    Operation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rename_op_binary_round_trips_and_agrees_with_text() {
        let operation = Operation::Rename { id: "node-1".into(), name: "Renamed".into() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn nakagin_document_text_round_trips_store_with_applied_operation() {
        let envelope = store::create_document_envelope::<trinity_ram::GraphFixture, Operation>(
            trinity_ram::TRINITY_GRAPH_SCHEMA,
            "doc-text-test",
            trinity_jack_engine::empty_jack_document(),
            None,
        );
        let mut doc_store = store::DocumentStore::new(envelope);
        doc_store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![Operation::Rename { id: "node-1".into(), name: "Renamed".into() }],
                description: None,
            })
            .ok();
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
    }
}
//#endregion 🧪️Tests
