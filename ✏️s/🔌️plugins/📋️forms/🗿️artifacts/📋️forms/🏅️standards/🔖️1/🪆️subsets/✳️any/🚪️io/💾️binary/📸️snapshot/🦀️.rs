//! 📦️ Forms artifact — binary document surface + laws (constitutional: pack). `store::ArtifactPack for
//! FormsSnapshot` encodes the derived `dsl::DslRecord` spec (composed `structure`/`results` child handles
//! included) and validates the document marker and child identities both ways. This component also owns
//! the thin `encode`/`decode` wrappers, the pack↔dsl equivalence law and the command-envelope law.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::FormsSnapshot;
use store::PackError;

//#region 🔖️ArtifactPack
impl store::ArtifactPack for FormsSnapshot {
    fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        self.validate().map_err(|detail| store::PackError::Refusal(store::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "pack", offset: 0, detail }))?;
        let inner = store::pack_rt::encode_document(&crate::standards::v1::subsets::any::io::binary::snapshot::pack::record_spec(), &crate::standards::v1::subsets::any::io::binary::snapshot::pack::record(self).map_err(store::PackError::from)?, options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &crate::standards::v1::subsets::any::io::binary::snapshot::pack::record_spec(), options)?;
        let snapshot = crate::standards::v1::subsets::any::io::binary::snapshot::pack::reconstruct_record(&record).map_err(store::PackError::from)?;
        snapshot.validate().map_err(|detail| store::PackError::Refusal(store::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "pack", offset: 0, detail }))?;
        Ok(snapshot)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(crate::standards::v1::subsets::any::io::binary::snapshot::pack::record_spec())
    }
}
//#endregion 🔖️ArtifactPack

/// 📦️ Encodes a `FormsSnapshot` to its binary pack form.
pub fn encode(document: &FormsSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `FormsSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<FormsSnapshot, PackError> {
    <FormsSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🔖️Store














//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests




#[path="📦️pack/🦀️.rs"]
pub(crate) mod pack;
