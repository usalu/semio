//! ⚖️ Terrain mutation encoding and original receiving share the authored state-patch record tags.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::GisTerrainMutation;
use protocol::OpBinary;

//#region 🔖️Codec
/// 📦️ Encodes a `GisTerrainMutation` to its binary command form.
pub fn encode_op(operation: &GisTerrainMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `GisTerrainMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<GisTerrainMutation, protocol::ProtocolError> {
    GisTerrainMutation::decode_op(bytes)
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
impl OpBinary for GisTerrainMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::tagged_value_binary::encode_op(COMPONENT_PROTOCOL_SEMIO, dsl::tagged_value_binary::VariantTag::Key, self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::tagged_value_binary::decode_op(COMPONENT_PROTOCOL_SEMIO, dsl::tagged_value_binary::VariantTag::Key, bytes)
    }
}

dsl::original_pack_value_operation_receiving!(GisTerrainMutation, tagged_key = COMPONENT_PROTOCOL_SEMIO);
