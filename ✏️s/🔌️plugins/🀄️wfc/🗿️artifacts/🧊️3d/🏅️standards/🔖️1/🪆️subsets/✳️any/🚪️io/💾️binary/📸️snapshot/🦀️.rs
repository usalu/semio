//! 📦️ `wfc3d` artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::snapshot::Wfc3dSnapshot;
use store::{ErasedSnapshotRetirement, PackError};

/// 📦️ Encodes a `Wfc3dSnapshot` to its binary pack form.
pub fn encode(document: &Wfc3dSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Wfc3dSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Wfc3dSnapshot, PackError> {
    <Wfc3dSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region ♻️Retirement
/// 📖️ Decodes into a LIVE projection and hands back the displaced document as a bounded retirement
/// admitted under the caller's own five-currency grant — the only decode entry point a mounted store
/// may call, because it never drops the old snapshot inline. A refused admission restores the exact
/// displaced document into `target` and drops only the freshly decoded, never-published one.
pub fn decode_into(target: &mut Wfc3dSnapshot, bytes: &[u8], grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, semio_framework_value::retained_clone::RetainedCloneProgress), PackError> {
    let next = decode(bytes)?;
    let displaced = std::mem::replace(target, next);
    match semio_framework_value::retirement::admit_owned_retirement(displaced, grant) {
        Ok(admitted) => Ok(admitted),
        Err((error, displaced)) => {
            drop(std::mem::replace(target, displaced));
            Err(PackError::from(error))
        }
    }
}
//#endregion ♻️Retirement

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `encode`/`decode` speak, named as the schema names the export.
pub type Wfc3dSnapshotBinary = Vec<u8>;
//#endregion 🚚️Carrier

mod native_codec {
use super::*;
use crate::schema::snapshot::{GraphRule, Slot3d, SlotEdge, Tile, TileMedia3d, Wfc3dSnapshot, WFC3D_DOCUMENT_SCHEMA};
use crate::standards::v1::subsets::any::io::text::snapshot::{Wfc3dSnapshotDsl,wfc3d_document_to_dsl,wfc3d_document_from_dsl};

impl store::ArtifactPack for Wfc3dSnapshotDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl store::ArtifactPack for Wfc3dSnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <Wfc3dSnapshotDsl as store::ArtifactPack>::encode_pack_with(&wfc3d_document_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <Wfc3dSnapshotDsl as store::ArtifactPack>::decode_pack_with(bytes, options)?;
        wfc3d_document_from_dsl(parsed).map_err(store::text_error_to_pack_error)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        <Wfc3dSnapshotDsl as store::ArtifactPack>::record_spec()
    }
}
}
