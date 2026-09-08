//! ⚖️ VCS artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! The app's typed `VcsCommand` enum — which used to share the old `📡️protocol` crate with this codec —
//! is a SURFACE concern, not an artifact one: it now lives in the editor surface's
//! `✏️editor/🦀️.rs`, assembled from the `🎮️commands/*` payload modules by
//! `semio_framework_plugin::app_commands!`.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::op::VcsDemoMutation;
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
