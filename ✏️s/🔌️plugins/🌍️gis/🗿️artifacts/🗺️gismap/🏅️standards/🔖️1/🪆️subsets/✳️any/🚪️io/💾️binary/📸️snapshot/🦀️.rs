//! 📦️ GIS map artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::GisMapSnapshot;
use store::PackError;

/// 📦️ Encodes a `GisMapSnapshot` to its binary pack form.
pub fn encode(document: &GisMapSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `GisMapSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<GisMapSnapshot, PackError> {
    <GisMapSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests

//#endregion 🧪️Tests



pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::io::binary::snapshot::owned_pack;
