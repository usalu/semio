//! 🪞️ Generation2d binary pack codec — `ArtifactPack`/`ArtifactStoreSnapshot` for the snapshot and its DSL twin, owned by the crate that owns both.

use semio_os_kernel as store;
use crate::snapshot::Generation2dSnapshot;
use crate::dsl_mirror::Generation2dSnapshotDsl;
use crate::dsl_mirror::generation2d_document_to_dsl;
use crate::dsl_mirror::generation2d_document_from_dsl;

impl pack::snapshot::ArtifactPack for Generation2dSnapshotDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_os_kernel::os_store::serialization::ArtifactStoreSnapshot for Generation2dSnapshotDsl {
    fn reference_snapshot_codec()->Option<::semio_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}


}

impl pack::snapshot::ArtifactPack for Generation2dSnapshot {

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let document = generation2d_document_to_dsl(self);
        let inner = store::pack_rt::encode_document(&Generation2dSnapshotDsl::__dsl_spec(), &document.__dsl_to_record(), options)?;
        let mut bytes = Vec::with_capacity(4 + inner.len());
        bytes.extend_from_slice(b"P2D2");
        bytes.extend_from_slice(&inner);
        Ok(bytes)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if !bytes.starts_with(b"P2D2") {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "generation2d pack discriminator mismatch")));
        }
        let (record, _report) = store::pack_rt::decode_document(&bytes[4..], &Generation2dSnapshotDsl::__dsl_spec(), options)?;
        let parsed = Generation2dSnapshotDsl::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)?;
        generation2d_document_from_dsl(parsed).map_err(store::os_store::serialization::text_error_to_pack_error)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        <Generation2dSnapshotDsl as pack::snapshot::ArtifactPack>::record_spec()
    }
}
impl ::semio_os_kernel::os_store::serialization::ArtifactStoreSnapshot for Generation2dSnapshot {
    fn reference_snapshot_codec()->Option<::semio_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> { Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec()) }
}
