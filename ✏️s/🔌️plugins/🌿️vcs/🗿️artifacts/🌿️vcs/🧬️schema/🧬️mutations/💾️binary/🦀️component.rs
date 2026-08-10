//! ⚖️ VCS artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! The app's typed `VcsCommand` enum — which used to share the old `📡️protocol` crate with this codec —
//! is an APP concern, not an artifact one: it now lives in `🎛️apps/🌿️vcs/🦀️component.rs`, assembled from
//! the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::vcs::op::VcsDemoMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `VcsDemoMutation` to its binary state-patch form.
pub fn encode_op(operation: &VcsDemoMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `VcsDemoMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<VcsDemoMutation, protocol::ProtocolError> {
    VcsDemoMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = VcsDemoMutation::SetCounter { counter: 7 };
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
//#endregion 🧪️Tests
