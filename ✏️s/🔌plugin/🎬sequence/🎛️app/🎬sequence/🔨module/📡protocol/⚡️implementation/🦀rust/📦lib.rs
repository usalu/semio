//! ⚖️ Sequence app — binary command protocol surface + laws (constitutional: protocol).

use protocol::OpBinary;
use sequence_op::SequenceOperation;

/// 📦 Encodes a `SequenceOperation` to its binary command form.
pub fn encode_op(operation: &SequenceOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes a `SequenceOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<SequenceOperation, protocol::ProtocolError> {
    SequenceOperation::decode_op(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use sequence::default_fixture;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = SequenceOperation::StepsPatch { id: "step-1".into(), patch: sequence::SequenceStepPatch { x: Some(42.0), ..Default::default() } };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    /// 🧪 Whole-store round trip: applies an operation through a real `SequenceStore`, then proves
    /// the resulting envelope survives both the text and binary document-level protocols.
    #[test]
    fn sequence_document_text_round_trips_store_with_applied_operation() {
        let envelope = store::create_document_envelope::<sequence::SequenceFixture, SequenceOperation>(sequence::SEQUENCE_FIXTURE_SCHEMA, "sequence-text-test", default_fixture(), None);
        let mut doc_store = store::DocumentStore::new(envelope);
        doc_store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![SequenceOperation::StepsAdd {
                    index: 2,
                    item: sequence::SequenceStep { id: "step-7".into(), kind: "log.print".into(), params: sequence::StepParams::new(), x: 12.0, y: 24.0, slot: None, collapsed: false },
                }],
                description: None,
            })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
    }
}
//#endregion 🧪Tests
