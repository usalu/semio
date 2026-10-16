//! 📦️ Forms artifact — binary document surface + laws (constitutional: pack). `pack::snapshot::ArtifactPack for
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
impl pack::snapshot::ArtifactPack for FormsSnapshot {
    
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        self.validate().map_err(|detail| PackError::Refusal(store::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "pack", offset: 0, detail }))?;
        let inner = store::pack_rt::encode_document(&pack::record_spec(), &pack::record(self).map_err(PackError::from)?, options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &pack::record_spec(), options)?;
        let snapshot = pack::reconstruct_record(&record).map_err(PackError::from)?;
        snapshot.validate().map_err(|detail| PackError::Refusal(store::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "pack", offset: 0, detail }))?;
        Ok(snapshot)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(pack::record_spec())
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for FormsSnapshot {
fn sqlite_snapshot_codec()->Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec>{Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())}
}
//#endregion 🔖️ArtifactPack

/// 📦️ Encodes a `FormsSnapshot` to its binary pack form.
pub fn encode(document: &FormsSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `FormsSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<FormsSnapshot, PackError> {
    <FormsSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
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

/// 🫴️ Declares this owner’s actual controlled native record factories.
impl pack::snapshot::ArtifactNativeSnapshot for FormsSnapshot {
 fn decode_native_snapshot(payload:pack::snapshot::NativeSnapshotInput<'_>,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{store::decode_native_snapshot_record(payload,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),pack::record_spec_producer(),|record,native|{let maximum=native.maximum_bytes();pack::reconstruct_record_controlled(record,native,maximum)},control)}
 fn encode_native_snapshot(&self,encoding:pack::snapshot::NativeSnapshotEncoding,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<store::io_schema::IoPayload,semio_framework_value::ValueError>{store::encode_native_snapshot_record(encoding,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),pack::record_spec_producer(),|native|pack::record_native_controlled(self,native),control)}
}
