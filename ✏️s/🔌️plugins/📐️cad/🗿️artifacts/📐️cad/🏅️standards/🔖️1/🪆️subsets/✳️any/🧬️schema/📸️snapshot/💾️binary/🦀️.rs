//! 🎒️ CAD artifact — the binary document surface: `encode`/`decode` over the derive-generated
//! `store::ArtifactPack`, and the law that pack and dsl are two projections of the same `CadSnapshot`.

use crate::CadSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `CadSnapshot` to its binary pack form.
pub fn encode(document: &CadSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `CadSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<CadSnapshot, PackError> {
    <CadSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
