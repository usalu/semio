//! ⚖️ S Studio app — binary command protocol surface + laws (constitutional: protocol).
//!
//! 🕳️ Wraps `semio_framework_os::OsOperation` — see `space_op`'s doc comment for why this app owns no
//! document/operation type.

use semio_framework_os::OsOperation;
use protocol::OpBinary;

/// 📦 Encodes an `OsOperation` to its binary command form.
pub fn encode_op(operation: &OsOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes an `OsOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<OsOperation, protocol::ProtocolError> {
    OsOperation::decode_op(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = OsOperation::MoveWorkflowNode { node_id: "node-1".into(), x: 12.0, y: -8.0 };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
//#endregion 🧪Tests
