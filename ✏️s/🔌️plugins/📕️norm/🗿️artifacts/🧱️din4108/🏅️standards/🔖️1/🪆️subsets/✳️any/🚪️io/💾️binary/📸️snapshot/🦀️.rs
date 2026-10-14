//! 📦️ DIN 4108 app — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use semio_s_artifact_norm_din4108_model::Din4108Snapshot;
use store::PackError;

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &Din4108Snapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Din4108Snapshot, PackError> {
    <Din4108Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
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
use semio_s_artifact_norm_din4108_model::*;
use semio_s_plugin_norm_compliance::ClimateZoneDe;
use semio_s_artifact_norm_din4108_model::{EnvelopeElement, LayerDocument, LayerSegment, ThermalBridge, ThermalZone, ZoneWindow};
use framework_schema::ArtifactSchema;
use crate::text::snapshot::*;
/// 📦️ Decodes a [`Din4108Snapshot`] from the binary `.pack.semio` envelope.
pub fn decode_din4108_pack(bytes: &[u8]) -> Result<Din4108Snapshot, String> {
    <Din4108Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
/// 📦️ Encodes a [`Din4108Snapshot`] to its binary `.pack.semio` envelope.
pub fn encode_din4108_pack(snapshot: &Din4108Snapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(snapshot)
}
}
pub use crate::binary::snapshot::native_snapshot_codec::*;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(@binary Din4108Snapshot, extension="din4108", envelope_id="norm.din4108", sqlite=crate::sqlite::snapshot::sqlite_codec);
