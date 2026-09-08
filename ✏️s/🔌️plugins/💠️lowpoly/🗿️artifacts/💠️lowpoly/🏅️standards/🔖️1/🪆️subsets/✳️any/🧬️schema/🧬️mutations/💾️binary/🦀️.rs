//! ⚖️ Lowpoly artifact — binary command wire codec (constitutional: spr).
//!
//! `protocol::OpBinary for LowpolyMutation` is implemented directly in `../📝️text/🦀️.rs`
//! (JSON-body encoding, see that file's doc comment). This component only adds the thin
//! artifact-facing `encode_op`/`decode_op` wrappers plus the op text/binary equivalence law and a
//! whole-store round trip.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::mutations::text::LowpolyMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `LowpolyMutation` to its binary command form.
pub fn encode_op(operation: &LowpolyMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `LowpolyMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<LowpolyMutation, protocol::ProtocolError> {
    LowpolyMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
