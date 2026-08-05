//! ⚖️ VCS artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! The app's typed `VcsCommand` enum — which used to share the old `📡️protocol` crate with this codec —
//! is an APP concern, not an artifact one: it now lives in `🎛️apps/🌿️vcs/🦀️component.rs`, assembled from
//! the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

use crate::artifacts::vcs::op::VcsDemoOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `VcsDemoOperation` to its binary state-patch form.
pub fn encode_op(operation: &VcsDemoOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `VcsDemoOperation` from its binary state-patch form.
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
