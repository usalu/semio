//! ⚖️ Mathematical app — binary command protocol surface + laws (constitutional: protocol).

use mathematical_op::MathOperation;
use protocol::OpBinary;

/// 📦 Encodes a `MathOperation` to its binary command form.
pub fn encode_op(operation: &MathOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes a `MathOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<MathOperation, protocol::ProtocolError> {
    MathOperation::decode_op(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use mathematical::{MathGraph, MathProjection};

    #[test]
    fn math_document_text_round_trips_through_store() {
        let initial = MathProjection::default();
        let envelope = store::create_document_envelope("semio.mathematical/v1", "math-demo", initial, None);
        let mut store = store::DocumentStore::new(envelope);
        let mut graph = MathGraph::default();
        graph.algorithm = "components".into();
        store
            .dispatch(store::DocumentCommand::Apply { operations: vec![MathOperation::SetGraph { graph }], description: None })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪Tests
