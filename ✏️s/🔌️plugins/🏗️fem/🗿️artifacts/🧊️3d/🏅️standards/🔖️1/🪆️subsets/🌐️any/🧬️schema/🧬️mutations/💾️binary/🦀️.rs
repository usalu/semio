//! ⚖️ FEM 3D artifact — binary operation protocol surface + laws (constitutional: spr, renamed from the
//! old `📡️protocol` crate — the old crate's hand-rolled `Fem3dCommand` enum moved to `app_commands!` in
//! `crate::editor::fem3d`; only the `Fem3dMutation` codec pair survives here).

use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use protocol::OpBinary;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `Fem3dMutation` to its binary command form.
pub fn encode_op(operation: &Fem3dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Fem3dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Fem3dMutation, protocol::ProtocolError> {
    Fem3dMutation::decode_op(bytes)
}

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-protocol-conformance/🦀️.rs"]
mod semio_protocol_conformance;
