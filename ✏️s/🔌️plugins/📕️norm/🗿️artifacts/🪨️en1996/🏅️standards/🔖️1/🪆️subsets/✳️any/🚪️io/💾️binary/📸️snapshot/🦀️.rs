//! 📦️ EN 1996 app — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use store::PackError;

/// 📦️ Encodes a `En1996Snapshot` to its binary pack form.
pub fn encode(document: &En1996Snapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `En1996Snapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<En1996Snapshot, PackError> {
    <En1996Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod native_snapshot_codec {
use crate::binary::snapshot::*;
use crate::binary::snapshot::*;
use semio_s_artifact_norm_en1996_model::*;
use semio_s_plugin_norm_compliance::{AnnexChoice, DesignSituation};
use semio_s_artifact_norm_en1996_model::{MasonryClass, MasonryWall, MortarClass, MortarType, UnitGroup, UnitMaterial, WallLoadCase, WallType, ExposureClass};
use framework_schema::ArtifactSchema;
use crate::text::snapshot::*;
pub fn decode_en1996_pack(bytes: &[u8]) -> Result<En1996Snapshot, String> {
    <En1996Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
pub fn encode_en1996_pack(snapshot: &En1996Snapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(snapshot)
}
}
pub use crate::binary::snapshot::native_snapshot_codec::*;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(@binary En1996Snapshot, extension="en1996", envelope_id="norm.en1996", sqlite=crate::sqlite::snapshot::sqlite_codec);
