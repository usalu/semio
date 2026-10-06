//! 📦️ DIN V 18599 app — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::Din18599Snapshot;
use store::PackError;

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &Din18599Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Din18599Snapshot, PackError> {
    <Din18599Snapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{
    Adjacency, Attachment, AutomationClass, BuildingCategory, CalculationMethod, CoolingSystem, DhwSystem, Din18599ClimateChild, ElementKind, EnvelopeElement, HeatingSystem, LightingSystem, MonthlyClimate, Renewables, ThermalZone, UseClass, VentilationSystem,
};
use framework_schema::ArtifactSchema;

impl store::ArtifactPack for Din18599Snapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(sqlite::sqlite_codec())
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{
    Adjacency, Attachment, AutomationClass, BuildingCategory, CalculationMethod, CoolingSystem, DhwSystem, Din18599ClimateChild, ElementKind, EnvelopeElement, HeatingSystem, LightingSystem, MonthlyClimate, Renewables, ThermalZone, UseClass, VentilationSystem,
};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::any::io::text::snapshot::*;
/// 📦️ Decodes a [`Din18599Snapshot`] from the binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din18599_pack(bytes: &[u8]) -> Result<Din18599Snapshot, String> {
    <Din18599Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
/// 📦️ Encodes a [`Din18599Snapshot`] to its binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_din18599_pack(snapshot: &Din18599Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
}
pub use native_snapshot_codec::*;
