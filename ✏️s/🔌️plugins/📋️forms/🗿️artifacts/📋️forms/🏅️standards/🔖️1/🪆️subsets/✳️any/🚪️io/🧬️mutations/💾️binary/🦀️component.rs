//! ⚖️ Forms artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! `protocol::OpBinary for FormMutation` is implemented directly in the shared `playbook` kernel crate;
//! see `🗿️artifacts/📋️forms/🦀️component.rs` for why. This component only adds the thin artifact-facing
//! `encode_op`/`decode_op` wrappers plus the op text↔binary equivalence law.
//!
//! The app's typed `FormsCommand` enum — which used to share the old `📡️protocol` crate with this codec —
//! is an APP concern, not an artifact one: it now lives in the `✏️editor` surface's own root
//! `🦀️component.rs`, assembled from
//! the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::forms::op::FormMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `FormMutation` to its binary state-patch form.
pub async fn encode_op(operation: &FormMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `FormMutation` from its binary state-patch form.
pub async fn decode_op(bytes: &[u8]) -> Result<FormMutation, protocol::ProtocolError> {
    FormMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let operation = FormMutation::ChangeFormTitle(crate::artifacts::forms::mutations::change_form_title::mutation::ChangeFormTitle { new_title: Some("Renamed".into()) });
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
//#endregion 🧪️Tests
