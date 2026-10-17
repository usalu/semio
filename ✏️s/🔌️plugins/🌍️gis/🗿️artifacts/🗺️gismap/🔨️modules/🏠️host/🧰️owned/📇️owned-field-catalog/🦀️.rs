//! 📇️ Owned field catalog of the GIS map envelope: the exact field catalog, the decode owner bundle and the rejected conflict authority. See [`super`].

use super::*;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct GisMapEnvelopeOwnedFieldCatalog;

impl store::os_store::owned_schema_decode::ArtifactEnvelopeOwnedFieldCatalog<GisMapSnapshot, GisMapMutation> for GisMapEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Result<Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<GisMapSnapshot, GisMapMutation>>, Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<GisMapSnapshot>>> {
        store::os_store::owned_schema_decode::ArtifactEnvelopeFreshVcsAuthority::try_new(
            self.begin_snapshot(operation, generation, path),
            std::sync::Arc::new(GisMapSnapshotRetirementFactory),
            std::sync::Arc::new(GisMapMutationRetirementFactory),
            self.edit_history_decoder(),
        )
        .map(|authority| Box::new(authority) as Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<GisMapSnapshot, GisMapMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<GisMapSnapshot>> {
        Box::new(GisMapSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority<GisMapMutation>> {
        Box::new(GisMapMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(GisMapRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::os_store::owned_schema_decode::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<GisMapMutation>>> {
        store::os_store::owned_schema_decode::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(GisMapMutationRetirementFactory))
    }
}

pub fn gis_map_envelope_decode_owner_bundle() -> store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<GisMapSnapshot, GisMapMutation> {
    store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(GisMapEnvelopeOwnedFieldCatalog), std::sync::Arc::new(GisMapSnapshotRetirementFactory), std::sync::Arc::new(GisMapMutationRetirementFactory))
}

struct GisMapRejectedConflictAuthority {
    pub(super) terminal: bool,
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeSprConflictAuthority for GisMapRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Err(store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { offset: token.start, ..store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic::before("gis-map-envelope.fresh-conflict-not-admitted", store::os_store::owned_schema_decode::OwnedSchemaPath::ROOT) })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { Ok(0) }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { Ok(0) }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { Ok(0) }

    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { Ok(usize::from(!self.terminal)) }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if self.terminal {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}
