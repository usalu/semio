//! 📇️ Owned field catalog of the Generation3d envelope: the exact field catalog, the decode owner bundle and the catalog trait impl. See [`super`].

use super::*;

//#region 🔖️TypedOwnedEnvelopeCatalog
/// 🎭️ Owner-local exact catalog for the Generation3d fresh-envelope decode cohort.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct Generation3dEnvelopeOwnedFieldCatalog;

/// 📦️ Installs Generation3d's exact field catalog and nested owner retirement factories as
/// one indivisible app decode authority.
pub fn generation3d_envelope_decode_owner_bundle() -> store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Generation3dSnapshot, Generation3dMutation> {
    store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(Generation3dEnvelopeOwnedFieldCatalog), std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation3dSnapshot>::default()), std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation3dMutation>::default()))
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeOwnedFieldCatalog<Generation3dSnapshot, Generation3dMutation> for Generation3dEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Result<Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<Generation3dSnapshot, Generation3dMutation>>, Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<Generation3dSnapshot>>> {
        store::os_store::owned_schema_decode::ArtifactEnvelopeFreshVcsAuthority::try_new(
            self.begin_snapshot(operation, generation, path),
            std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation3dSnapshot>::default()),
            std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation3dMutation>::default()),
            self.edit_history_decoder(),
        )
        .map(|authority| Box::new(authority) as Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<Generation3dSnapshot, Generation3dMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<Generation3dSnapshot>> {
        Box::new(Generation3dPackSnapshotAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority<Generation3dMutation>> {
        Box::new(Generation3dMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(Generation3dRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::os_store::owned_schema_decode::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<Generation3dMutation>>> {
        store::os_store::owned_schema_decode::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation3dMutation>::default()))
    }
}
//#endregion 🔖️TypedOwnedEnvelopeCatalog
