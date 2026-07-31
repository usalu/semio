//! ⚖️ Sourcing curate app — binary command protocol surface + laws (constitutional: protocol).

use protocol::OpBinary;
use sourcing_op::SourcingOperation;

/// 📦️ Encodes a `SourcingOperation` to its binary command form.
pub fn encode_op(operation: &SourcingOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `SourcingOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<SourcingOperation, protocol::ProtocolError> {
    SourcingOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use sourcing::CurateDocument;

    /// 🌱️ Mirrors `sourcing_engine`'s private test-only helper (see that crate's tests for why this
    /// tiny fixture-assembly helper is duplicated rather than shared across crates).
    fn sample_document() -> CurateDocument {
        CurateDocument { stock: sourcing_engine::sourcing_modules().iter().flat_map(|module| module.demo_kinds()).collect(), ..Default::default() }
    }

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = SourcingOperation::SetDocument { document: sourcing_engine::empty_document() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    //#region 🔖️DslAndOpTextStore
    #[test]
    fn curate_document_text_round_trips_through_a_vcs_store() {
        let envelope = store::create_document_envelope(sourcing::SOURCING_CURATE_SCHEMA, "sourcing-curate-test", sample_document(), None);
        let mut store = store::DocumentStore::new(envelope);
        let mut next = store.projection().expect("projection").clone();
        sourcing_engine::curate_delta(&mut next, "beam-glulam-gl24h", 3);
        store
            .dispatch(store::DocumentCommand::Apply { operations: vec![SourcingOperation::SetDocument { document: next }], description: None })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
    //#endregion 🔖️DslAndOpTextStore
}
//#endregion 🧪️Tests
