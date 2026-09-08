//! ⚖️ FEM 2D artifact — binary operation protocol surface + laws (constitutional: spr).
//!
//! Renamed from the pre-migration `📡️protocol` crate: only `encode_op`/`decode_op` for `Fem2dMutation`
//! live here. The old crate's hand-rolled `Fem2dCommand` enum does NOT move here — it is rebuilt by
//! `app_commands!` in the app's `🦀️.rs` (see `crate::editor::fem2d::Fem2dCommand`).

use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use protocol::OpBinary;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `Fem2dMutation` to its binary command form.
pub fn encode_op(operation: &Fem2dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Fem2dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Fem2dMutation, protocol::ProtocolError> {
    Fem2dMutation::decode_op(bytes)
}

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-protocol-conformance/🦀️.rs"]
mod semio_protocol_conformance;
