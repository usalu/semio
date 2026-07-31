//! ⚖️ VCS app — binary command protocol surface + laws (constitutional: protocol).

use protocol::OpBinary;
use vcs_op::VcsDemoOperation;

/// 📦️ Encodes a `VcsDemoOperation` to its binary command form.
pub fn encode_op(operation: &VcsDemoOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `VcsDemoOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<VcsDemoOperation, protocol::ProtocolError> {
    VcsDemoOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = VcsDemoOperation::SetCounter { counter: 7 };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
//#endregion 🧪️Tests
