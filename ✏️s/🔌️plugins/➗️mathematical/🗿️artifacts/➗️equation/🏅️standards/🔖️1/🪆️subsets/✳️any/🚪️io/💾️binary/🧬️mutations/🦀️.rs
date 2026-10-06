//! ⚖️ Equation artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`).
//!
//! `protocol::OpBinary for EquationMutation` is implemented directly in `crate::op`
//! (see that module's doc comment). This component only adds the thin artifact-facing
//! `encode_op`/`decode_op` wrappers plus the op text↔binary equivalence law and a whole-store round trip.
//!
//! The app's typed `EquationCommand` enum — which used to share the old `📡️protocol` crate with this codec —
//! is an APP concern, not an artifact one: it now lives in `✏️editor/🦀️.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::op::EquationMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `EquationMutation` to its binary command form.
pub fn encode_op(operation: &EquationMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `EquationMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<EquationMutation, protocol::ProtocolError> {
    EquationMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
