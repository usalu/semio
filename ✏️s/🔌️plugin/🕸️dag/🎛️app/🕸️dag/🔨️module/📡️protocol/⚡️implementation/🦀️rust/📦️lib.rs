//! ⚖️ DAG app — binary command protocol surface + laws (constitutional: protocol).
//!
//! `protocol::OpBinary for DagOperation` is implemented directly in the DAG kernel crate
//! (`infinite_board_port_directed_dag`); see `s/plugin/dag/app/op/rs/lib.rs` for why. This crate only
//! adds the thin app-facing `encode_op`/`decode_op` wrappers plus the op text↔binary equivalence law.

use dag_op::DagOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `DagOperation` to its binary command form.
pub fn encode_op(operation: &DagOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `DagOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<DagOperation, protocol::ProtocolError> {
    DagOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = DagOperation::SetNodes { nodes: Vec::new() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
//#endregion 🧪️Tests
