//! 📦️ FEM 3D artifact — binary document surface + laws (constitutional: pack).

use crate::Fem3dSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `Fem3dSnapshot` to its binary pack form.
pub fn encode(document: &Fem3dSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Fem3dSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Fem3dSnapshot, PackError> {
    <Fem3dSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-protocol-conformance/🦀️.rs"]
mod semio_protocol_conformance;
