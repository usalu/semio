//! ⚖️ GIS terrain artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).
//!
//! 🧷️ `GisTerrainMutation` derives `dsl::DslEnum` directly (no foreign `CollectionMutation` in its
//! shape, unlike the map artifact), so this component is a pure pass-through over the derived codec.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::mutations::text::GisTerrainMutation;
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
