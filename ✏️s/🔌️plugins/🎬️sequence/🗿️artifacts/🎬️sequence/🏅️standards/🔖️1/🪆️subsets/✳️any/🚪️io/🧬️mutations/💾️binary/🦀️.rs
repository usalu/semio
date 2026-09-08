//! ⚖️ Sequence artifact — state-patch-representation wire codec (constitutional: protocol). The
//! `OpText`/`OpBinary` impls for `SequenceMutation` are handcrafted in the sibling `📝️text` facet
//! (P6: derive no longer emits these traits); this facet only carries the normative binary
//! protocol doc-string and the encode/decode free-function wrappers + their round-trip tests.
//!
//! The app's typed `SequenceCommand` enum — which used to share the old `📡️protocol` crate with this
//! codec — is an APP concern, not an artifact one: it lives in `✏️editor/🦀️.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::mutations::SequenceMutation;
use protocol::OpBinary;

//#region 🔖️OpText
/// 📦️ Encodes a `SequenceMutation` to its binary state-patch form.
pub fn encode_op(mutation: &SequenceMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a `SequenceMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<SequenceMutation, protocol::ProtocolError> {
    SequenceMutation::decode_op(bytes)
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
