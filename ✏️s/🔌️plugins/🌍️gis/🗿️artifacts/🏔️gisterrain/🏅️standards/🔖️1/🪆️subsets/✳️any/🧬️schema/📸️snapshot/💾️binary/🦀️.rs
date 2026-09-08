//! 📦️ GIS terrain artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::GisTerrainSnapshot;
use store::PackError;

/// 📦️ Encodes a `GisTerrainSnapshot` to its binary pack form.
pub fn encode(document: &GisTerrainSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `GisTerrainSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<GisTerrainSnapshot, PackError> {
    <GisTerrainSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
