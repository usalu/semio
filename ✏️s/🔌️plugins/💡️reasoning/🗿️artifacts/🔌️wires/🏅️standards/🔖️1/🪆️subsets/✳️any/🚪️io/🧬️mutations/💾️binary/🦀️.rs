//! ⚖️ Wires artifact — binary command protocol surface + laws (constitutional: spr, renamed from
//! protocol). The app-level `WiresCommand` binary command envelope (the old hand-derived enum this
//! module used to also host) is now REBUILT by `app_commands!` in `crate::editor::wires::component` — see
//! `crate::editor::wires::WiresCommand`'s doc there.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::mutations::WiresMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `WiresMutation` to its binary command form.
pub fn encode_op(operation: &WiresMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `WiresMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<WiresMutation, protocol::ProtocolError> {
    WiresMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
