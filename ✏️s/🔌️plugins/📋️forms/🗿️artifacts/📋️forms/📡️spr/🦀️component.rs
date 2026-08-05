//! ⚖️ Forms artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! `protocol::OpBinary for FormOperation` is implemented directly in the shared `playbook` kernel crate;
//! see `🗿️artifacts/📋️forms/🦀️component.rs` for why. This component only adds the thin artifact-facing
//! `encode_op`/`decode_op` wrappers plus the op text↔binary equivalence law.
//!
//! The app's typed `FormsCommand` enum — which used to share the old `📡️protocol` crate with this codec —
//! is an APP concern, not an artifact one: it now lives in `🎛️apps/📋️forms/🦀️component.rs`, assembled from
//! the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

use crate::artifacts::forms::op::FormOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `FormOperation` to its binary state-patch form.
pub fn encode_op(operation: &FormOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `FormOperation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<FormOperation, protocol::ProtocolError> {
    FormOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = FormOperation::UpdatePlaybook { title: Some("Renamed".into()) };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
//#endregion 🧪️Tests
