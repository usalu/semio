//! 📦️ VDI 3805 app — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::Vdi3805Snapshot;
use store::PackError;

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &Vdi3805Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Vdi3805Snapshot, PackError> {
    <Vdi3805Snapshot as store::ArtifactPack>::decode_pack(bytes)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ParametricGeometry, SecurityLimits};
use framework_schema::ArtifactSchema;
use std::collections::BTreeMap;
use crate::standards::v1::subsets::any::io::text::snapshot::*;
pub fn decode_vdi3805_pack(bytes: &[u8]) -> Result<Vdi3805Snapshot, String> { <Vdi3805Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| format!("{e:?}")) }
pub fn encode_vdi3805_pack(snapshot: &Vdi3805Snapshot) -> Vec<u8> { store::ArtifactPack::encode_pack(snapshot) }
}
pub use native_snapshot_codec::*;

crate::impl_norm_artifact_record!(@binary crate::Vdi3805Snapshot, extension="vdi3805", envelope_id="norm.vdi3805", sqlite=crate::standards::v1::subsets::any::io::sqlite::snapshot::sqlite_codec);
