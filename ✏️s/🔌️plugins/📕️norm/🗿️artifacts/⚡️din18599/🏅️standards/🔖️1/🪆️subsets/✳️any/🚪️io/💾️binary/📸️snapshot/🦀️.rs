//! 📦️ DIN V 18599 app — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use store::PackError;

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &Din18599Snapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Din18599Snapshot, PackError> {
    <Din18599Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
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
use semio_s_artifact_norm_din18599_model::*;
use semio_s_artifact_norm_din18599_model::{Adjacency, Attachment, AutomationClass, BuildingCategory, CalculationMethod, CoolingSystem, DhwSystem, Din18599ClimateChild, ElementKind, EnvelopeElement, HeatingSystem, LightingSystem, MonthlyClimate, Renewables, ThermalZone, UseClass, VentilationSystem};
use framework_schema::ArtifactSchema;
use crate::text::snapshot::*;
/// 📦️ Decodes a [`Din18599Snapshot`] from the binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din18599_pack(bytes: &[u8]) -> Result<Din18599Snapshot, String> {
    <Din18599Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
/// 📦️ Encodes a [`Din18599Snapshot`] to its binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_din18599_pack(snapshot: &Din18599Snapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(snapshot)
}
}
pub use crate::binary::snapshot::native_snapshot_codec::*;
