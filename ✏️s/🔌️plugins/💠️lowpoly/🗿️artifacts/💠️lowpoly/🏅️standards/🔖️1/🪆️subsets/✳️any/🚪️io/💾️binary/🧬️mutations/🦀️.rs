//! 💾️ Lowpoly binary operation codec and store round-trip laws.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::LowpolyMutation;
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

/// 💾️ Binary JSON operation payloads with the artifact's physical byte representation.
impl OpBinary for LowpolyMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::lowpoly_json_encode(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "lowpoly-mutation", offset: error.valid_up_to() as u64, detail: error.to_string() })?;
        semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::lowpoly_json_decode(text).map_err(|error| protocol::ProtocolError::Malformed { what: "lowpoly-mutation", offset: 0, detail: error.to_string() })
    }
}
