//! 📦️ DIN 4108 app — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::Din4108Snapshot;
use store::PackError;

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &Din4108Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Din4108Snapshot, PackError> {
    <Din4108Snapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::document::ClimateZoneDe;
use crate::{EnvelopeElement, LayerDocument, LayerSegment, ThermalBridge, ThermalZone, ZoneWindow};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::any::io::text::snapshot::*;
/// 📦️ Decodes a [`Din4108Snapshot`] from the binary `.pack.semio` envelope.
pub fn decode_din4108_pack(bytes: &[u8]) -> Result<Din4108Snapshot, String> {
    <Din4108Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
/// 📦️ Encodes a [`Din4108Snapshot`] to its binary `.pack.semio` envelope.
pub fn encode_din4108_pack(snapshot: &Din4108Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
}
pub use native_snapshot_codec::*;

crate::impl_norm_artifact_record!(@binary crate::Din4108Snapshot, extension="din4108", envelope_id="norm.din4108", sqlite=crate::standards::v1::subsets::any::io::sqlite::snapshot::sqlite_codec);
