//! 🔡️ Owned field decode of the GIS map envelope: the pack decoders and the macro-generated snapshot and mutation field decode authorities. See [`super`].

use super::*;

fn decode_gis_map_snapshot_pack(bytes: &[u8]) -> Result<GisMapSnapshot, ()> {
    <GisMapSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|_| ())
}

fn decode_gis_map_mutation_pack(bytes: &[u8]) -> Result<GisMapMutation, ()> {
    GisMapMutation::decode_op(bytes).map_err(|_| ())
}

macro_rules! gis_map_owned_field_close_capacity {
    (@demands, $value:ty) => {
        fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.copy_bytes) }

        fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { self.close_demands(maximum_copy_bytes).map(|demand| demand.capacity_bytes) }

        fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.release_bytes) }

        fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.depth) }
    };
    (ArtifactEnvelopeSnapshotFieldAuthority, $value:ty) => {
        gis_map_owned_field_close_capacity!(@demands, $value);

        fn maximum_close_byte_demand(&self) -> usize {
            store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
        }

        fn maximum_retained_close_bytes(&self) -> usize {
            store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
        }
    };
    (ArtifactEnvelopeMutationFieldAuthority, $value:ty) => {
        gis_map_owned_field_close_capacity!(@demands, $value);
    };
}

macro_rules! gis_map_owned_field_authority {
    ($state:ident, $authority:ident, $value:ty, $authority_trait:ident, $target_trait:ident, $publish:ident, $decode:path, $kind:literal) => {
        #[expect(clippy::large_enum_variant, reason = "The active decoder keeps its fixed path and admitted hex authority inline without a second allocation at the state transition.")]
        enum $state {
            AwaitToken,
            Decode(store::os_store::owned_schema_decode::OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>),
            Ready,
            Published,
            Closing,
            Complete,
        }

        pub(super) struct $authority {
            operation: semio_framework_job::OperationId,
            generation: semio_framework_job::Generation,
            path: store::os_store::owned_schema_decode::OwnedSchemaPath,
            state: $state,
            value: std::mem::ManuallyDrop<Option<$value>>,
            retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
        }

        impl $authority {
            pub(super) fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Self {
                Self { operation, generation, path, state: $state::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
            }

            fn diagnostic(&self, code: &'static str, offset: u64) -> store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic {
                store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { offset, ..store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic::before(code, self.path) }
            }

            fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                if matches!(self.state, $state::Decode(_)) {
                    return Ok(semio_framework_value::RetirementDemand { release_bytes: GIS_MAP_OWNED_FIELD_BYTES, depth: 1, ..Default::default() });
                }
                let demand = match self.retirement.as_ref() {
                    Some(owner) => store::os_store::snapshot_read::artifact_retirement_box_demands(owner, maximum_copy_bytes),
                    None => store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.value),
                };
                demand.map_err(|error| self.diagnostic(concat!("gis-map-envelope.", $kind, "-retirement-demand-fault"), 0).with_native(error))
            }
        }

        impl store::$authority_trait<$value> for $authority {
            fn accept_token(
                &mut self,
                token: store::os_store::owned_schema_decode::OwnedSchemaToken,
                terminal: bool,
                source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
                cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                let path = self.path;
                let diagnostic = |code: &'static str, offset| store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { offset, ..store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic::before(code, path) };
                if matches!(self.state, $state::AwaitToken) {
                    if !terminal {
                        return Err(diagnostic(concat!("gis-map-envelope.", $kind, "-pack-must-be-scalar"), token.start));
                    }
                    self.state = $state::Decode(store::os_store::owned_schema_decode::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
                }
                let $state::Decode(authority) = &mut self.state else {
                    return Err(diagnostic(concat!("gis-map-envelope.", $kind, "-pack-token-replayed"), token.start));
                };
                match authority.step(source, cx) {
                    store::os_store::owned_schema_decode::OwnedSchemaHexStep::Pending => Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending),
                    store::os_store::owned_schema_decode::OwnedSchemaHexStep::Complete => {
                        let bytes = authority.as_bytes().ok_or_else(|| diagnostic(concat!("gis-map-envelope.", $kind, "-pack-missing"), token.start))?;
                        let value = $decode(bytes).map_err(|_| diagnostic(concat!("gis-map-envelope.", $kind, "-pack-malformed"), token.start))?;
                        if !authority.release() {
                            return Err(diagnostic(concat!("gis-map-envelope.", $kind, "-pack-release-duplicate"), token.start));
                        }
                        *self.value = Some(value);
                        self.state = $state::Ready;
                        Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
                    }
                    store::os_store::owned_schema_decode::OwnedSchemaHexStep::Cancelled => Err(diagnostic(concat!("gis-map-envelope.", $kind, "-pack-cancelled"), token.start)),
                    store::os_store::owned_schema_decode::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
                }
            }

            fn publish_reserved(
                &mut self,
                target: &mut dyn store::$target_trait<$value>,
                reservation: store::os_store::owned_schema_decode::ArtifactEnvelopeFieldReservation,
                _cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                if !matches!(self.state, $state::Ready) {
                    return Err(self.diagnostic(concat!("gis-map-envelope.", $kind, "-pack-not-ready"), 0));
                }
                let value = self.value.take().ok_or_else(|| self.diagnostic(concat!("gis-map-envelope.", $kind, "-owner-missing"), 0))?;
                target.$publish(reservation, value);
                self.state = $state::Published;
                Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }

            gis_map_owned_field_close_capacity!($authority_trait, $value);

            fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                let empty = RetainedCloneProgress::default();
                if self.terminal_is_empty() {
                    return Ok(RetainedCloneStep::Complete(empty));
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(empty));
                }
                let demand = self.close_demands(grant.maximum_copy_bytes)?;
                if grant.maximum_depth < demand.depth {
                    return Err(self.diagnostic(concat!("gis-map-envelope.", $kind, "-retirement-depth"), 0));
                }
                if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
                    return Ok(RetainedCloneStep::Progress(empty));
                }
                if let $state::Decode(authority) = &mut self.state {
                    authority.cancel();
                    self.state = $state::Closing;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: GIS_MAP_OWNED_FIELD_BYTES, ..empty }));
                }
                let step = if self.retirement.is_some() {
                    store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant)
                } else if self.value.is_some() {
                    store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant)
                } else {
                    Ok(RetainedCloneStep::Complete(empty))
                }
                .map_err(|error| self.diagnostic(concat!("gis-map-envelope.", $kind, "-retirement-fault"), 0).with_native(error))?;
                self.state = $state::Closing;
                if self.value.is_none() && self.retirement.is_none() {
                    self.state = $state::Complete;
                    return Ok(RetainedCloneStep::Complete(step.progress()));
                }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }

            fn terminal_is_empty(&self) -> bool {
                matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none()
            }
        }

        impl Drop for $authority {
            fn drop(&mut self) {
                assert!(
                    (matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none()) || std::thread::panicking(),
                    concat!("GIS ", $kind, " decode reached Drop before publication or bounded retirement")
                );
            }
        }
    };
}

gis_map_owned_field_authority!(
    GisMapSnapshotDecodeState,
    GisMapSnapshotDecodeAuthority,
    GisMapSnapshot,
    ArtifactEnvelopeSnapshotFieldAuthority,
    ArtifactEnvelopeSnapshotFieldTarget,
    publish_snapshot_reserved,
    decode_gis_map_snapshot_pack,
    "snapshot"
);

gis_map_owned_field_authority!(
    GisMapMutationDecodeState,
    GisMapMutationDecodeAuthority,
    GisMapMutation,
    ArtifactEnvelopeMutationFieldAuthority,
    ArtifactEnvelopeMutationFieldTarget,
    publish_mutation_reserved,
    decode_gis_map_mutation_pack,
    "mutation"
);
