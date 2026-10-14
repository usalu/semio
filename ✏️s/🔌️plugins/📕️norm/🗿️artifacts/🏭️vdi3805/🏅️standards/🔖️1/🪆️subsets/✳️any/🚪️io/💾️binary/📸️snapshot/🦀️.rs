//! 📦️ VDI 3805 app — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;
use store::PackError;

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &Vdi3805Snapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Vdi3805Snapshot, PackError> {
    <Vdi3805Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[allow(unused_imports)]
mod native_snapshot_codec {
use crate::binary::snapshot::*;
use crate::binary::snapshot::*;
use semio_s_artifact_norm_vdi3805_model::*;
use semio_s_artifact_norm_vdi3805_model::{CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ParametricGeometry, SecurityLimits};
use framework_schema::ArtifactSchema;
use std::collections::BTreeMap;
use crate::text::snapshot::*;
pub fn decode_vdi3805_pack(bytes: &[u8]) -> Result<Vdi3805Snapshot, String> { <Vdi3805Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|e| format!("{e:?}")) }
pub fn encode_vdi3805_pack(snapshot: &Vdi3805Snapshot) -> Vec<u8> { pack::snapshot::ArtifactPack::encode_pack(snapshot) }
}
pub use crate::binary::snapshot::native_snapshot_codec::*;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(@binary Vdi3805Snapshot, extension="vdi3805", envelope_id="norm.vdi3805", sqlite=crate::sqlite::snapshot::sqlite_codec);
