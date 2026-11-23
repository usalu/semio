//! ✉️ Retained fresh-envelope decode owners for FEM artifacts whose snapshot and operation travel as packs.
//! Included by path into each FEM editor crate (2D and 3D) and instantiated with [`fem_pack_envelope_decode_owner_bundle!`].

use ::semio_framework_job::{ReplacementIdentity, StepContext};
use ::semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits};
use ::semio_framework_value::{ErasedSnapshotRetirement, RetireOwned, RetainedAllocationEffect, RetirementDemand};
use ::semio_os_kernel::os_store::ArtifactPack;
use ::semio_os_kernel::os_store::owned_schema_decode::{
    ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES, ArtifactEnvelopeCloseExit, ArtifactEnvelopeClosePoll,
    ArtifactEnvelopeFieldDecodeStep, ArtifactEnvelopeFieldReservation, ArtifactEnvelopeMutationFieldAuthority,
    ArtifactEnvelopeMutationFieldTarget, ArtifactEnvelopeSnapshotFieldAuthority, ArtifactEnvelopeSnapshotFieldTarget,
    ArtifactEnvelopeSprConflictAuthority, OwnedSchemaDecodeDiagnostic, OwnedSchemaPath, OwnedSchemaRecordCursor,
    OwnedSchemaToken, artifact_envelope_close_resume_with, artifact_envelope_close_settle,
    artifact_envelope_failure_diagnostic, genesis_pack_admission::ArtifactGenesisPackCapture,
};
use ::semio_os_kernel::os_store::snapshot_read::{
    artifact_retirement_admit_owned, artifact_retirement_box_close_step, artifact_retirement_box_demands,
    artifact_retirement_owned_birth_demands,
};
use ::semio_os_kernel::OpBinary;
use std::mem::ManuallyDrop;

/// 📦️ One pack-encoded envelope field: captured in bounded turns, decoded once, retired through its exact owner.
pub struct PackField<T> {
    replacement: Option<ReplacementIdentity>,
    path: OwnedSchemaPath,
    capture: Option<ArtifactGenesisPackCapture>,
    value: ManuallyDrop<Option<T>>,
    retirement: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    terminal: bool,
}

impl<T: RetireOwned + Send + Sync + 'static> PackField<T> {
    pub fn new(replacement: Option<ReplacementIdentity>, path: OwnedSchemaPath) -> Self {
        Self { replacement, path, capture: None, value: ManuallyDrop::new(None), retirement: ManuallyDrop::new(None), terminal: false }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { offset, ..OwnedSchemaDecodeDiagnostic::before(code, self.path) }
    }

    fn accept(
        &mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut StepContext<'_>,
        decode: impl FnOnce(&[u8]) -> Result<T, OwnedSchemaDecodeDiagnostic>,
    ) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        let offset = token.start;
        let failure = self.diagnostic("fem-envelope.pack-effect", offset);
        if !terminal || self.terminal || self.value.is_some() || self.retirement.is_some() {
            return Err(self.diagnostic("fem-envelope.pack-incidence", offset));
        }
        if cx.is_cancelled() || cx.replacement_is_stale(self.replacement) {
            return Err(self.diagnostic("fem-envelope.pack-source-lost", offset));
        }
        if cx.should_yield() { return Ok(ArtifactEnvelopeFieldDecodeStep::Pending); }
        if self.capture.is_none() {
            let capacity_bytes = usize::try_from(token.end.saturating_sub(offset).saturating_sub(2) / 2)
                .map_err(|_| self.diagnostic("fem-envelope.pack-extent", offset))?;
            let demand = RetirementDemand { capacity_bytes, depth: 1, ..Default::default() };
            if !cx.admit_work_step(demand).map_err(|error| failure.with_native(error))? {
                return Ok(ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            let capture = ArtifactGenesisPackCapture::new(token, cx)?;
            let retained_capacity_bytes = capture.bytes.capacity();
            self.capture = Some(capture);
            cx.consume_retained(RetainedCloneProgress { retained_capacity_bytes, ..Default::default() })
                .map_err(|error| failure.with_native(error))?;
        }
        let capture = self.capture.as_mut().unwrap();
        if !capture.step(source, cx)? || cx.should_yield() {
            return Ok(ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        let released_bytes = capture.bytes.capacity();
        let demand = RetirementDemand { release_bytes: released_bytes, depth: 1, ..Default::default() };
        if !cx.admit_work_step(demand).map_err(|error| failure.with_native(error))? {
            return Ok(ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        *self.value = Some(decode(&capture.bytes)?);
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() })
            .map_err(|error| failure.with_native(error))?;
        self.capture = None;
        cx.consume_fuel(1);
        Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn take_value(&mut self) -> Result<T, OwnedSchemaDecodeDiagnostic> {
        let value = self.value.take().ok_or_else(|| self.diagnostic("fem-envelope.pack-owner-missing", 0))?;
        self.terminal = true;
        Ok(value)
    }

    fn demands(&self, body: usize) -> Result<RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        if let Some(capture) = &self.capture {
            return Ok(RetirementDemand { release_bytes: capture.bytes.capacity(), depth: 1, ..Default::default() });
        }
        let demand = match self.retirement.as_ref() {
            Some(owner) => artifact_retirement_box_demands(owner, body),
            None => artifact_retirement_owned_birth_demands(&self.value),
        };
        demand.map_err(|error| self.diagnostic("fem-envelope.pack-retirement-demand", 0).with_native(error))
    }

    fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ArtifactEnvelopeCloseExit> {
        let empty = RetainedCloneProgress::default();
        if self.terminal { return Ok(RetainedCloneStep::Complete(empty)); }
        let grant = effect.grant();
        let body = grant.maximum_copy_bytes;
        let diagnostic = self.diagnostic("fem-envelope.pack-retirement", 0);
        let demand = self.demands(body)?;
        if !retained_clone_grant_admits(grant, demand) || effect.remaining_work_units().map_err(|error| diagnostic.with_native(error))? == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if self.capture.take().is_some() {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
        }
        let progress = if self.retirement.is_some() {
            artifact_envelope_close_resume_with(artifact_retirement_box_close_step(&mut self.retirement, body, effect), |failure| artifact_envelope_failure_diagnostic(diagnostic, &failure))?.progress()
        } else if self.value.is_some() {
            artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, effect).result.map_err(|error| diagnostic.with_native(error))?.progress()
        } else {
            RetainedCloneProgress { copied_items: 1, ..empty }
        };
        self.terminal = self.value.is_none() && self.retirement.is_none();
        Ok(if self.terminal { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }
}

impl<T> Drop for PackField<T> {
    fn drop(&mut self) {
        assert!(self.terminal || std::thread::panicking(), "FEM pack owner must publish or retire before Drop");
    }
}

macro_rules! pack_field_authority {
    () => {
        fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { Ok(self.demands(0)?.copy_bytes) }
        fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic> { Ok(self.demands(body)?.capacity_bytes) }
        fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { Ok(self.demands(0)?.release_bytes) }
        fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { Ok(self.demands(0)?.depth) }
        fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> ArtifactEnvelopeClosePoll {
            let step = self.close(effect);
            artifact_envelope_close_settle(effect, step)
        }
        fn terminal_is_empty(&self) -> bool { self.terminal }
    };
}

impl<P: ArtifactPack + RetireOwned + Send + Sync + 'static> ArtifactEnvelopeSnapshotFieldAuthority<P> for PackField<P> {
    fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        let invalid = self.diagnostic("fem-envelope.pack-malformed", token.start);
        self.accept(token, terminal, source, cx, |bytes| P::decode_pack(bytes).map_err(|_| invalid))
    }

    fn publish_reserved(&mut self, target: &mut dyn ArtifactEnvelopeSnapshotFieldTarget<P>, reservation: ArtifactEnvelopeFieldReservation, _cx: &mut StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        target.publish_snapshot_reserved(reservation, self.take_value()?);
        Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    pack_field_authority!();

    fn maximum_close_byte_demand(&self) -> usize { ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES }
    fn maximum_retained_close_bytes(&self) -> usize { ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES }
}

impl<M: OpBinary + RetireOwned + Send + Sync + 'static> ArtifactEnvelopeMutationFieldAuthority<M> for PackField<M> {
    fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        let invalid = self.diagnostic("fem-envelope.pack-malformed", token.start);
        self.accept(token, terminal, source, cx, |bytes| M::decode_op(bytes).map_err(|_| invalid))
    }

    fn publish_reserved(&mut self, target: &mut dyn ArtifactEnvelopeMutationFieldTarget<M>, reservation: ArtifactEnvelopeFieldReservation, _cx: &mut StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        target.publish_mutation_reserved(reservation, self.take_value()?);
        Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    pack_field_authority!();
}

/// 🚫️ A fresh envelope never carries a conflict path: this issuer refuses it.
pub struct FreshConflictRefusal {
    pub path: OwnedSchemaPath,
}

impl ArtifactEnvelopeSprConflictAuthority for FreshConflictRefusal {
    fn accept_token(&mut self, token: OwnedSchemaToken, _terminal: bool, _source: &OwnedSchemaRecordCursor, _cx: &mut StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        Err(OwnedSchemaDecodeDiagnostic { offset: token.start, ..OwnedSchemaDecodeDiagnostic::before("fem-envelope.fresh-conflict-not-admitted", self.path) })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { Ok(0) }
    fn next_close_capacity_byte_demand(&self, _body: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { Ok(0) }
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { Ok(0) }

    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> ArtifactEnvelopeClosePoll {
        artifact_envelope_close_settle(effect, Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())))
    }

    fn terminal_is_empty(&self) -> bool { true }
}

/// ✉️ Defines `$bundle() -> ArtifactEnvelopeDecodeOwnerBundle<$snapshot, $mutation>`, the exact owner catalog of one FEM artifact.
macro_rules! fem_pack_envelope_decode_owner_bundle {
    ($bundle:ident, $fields:ident, $snapshot:ty, $mutation:ty) => {
        #[derive(::semio_framework_value::FactoryPayloadRetirement)]
        struct FemFieldCatalog;

        impl ::semio_os_kernel::os_store::owned_schema_decode::ArtifactEnvelopeOwnedFieldCatalog<$snapshot, $mutation> for FemFieldCatalog {
            fn begin_vcs(&self, replacement: Option<::semio_framework_job::ReplacementIdentity>, path: ::semio_os_kernel::os_store::owned_schema_decode::OwnedSchemaPath) -> Result<Box<dyn ::semio_os_kernel::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<$snapshot, $mutation>>, Box<dyn ::semio_os_kernel::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<$snapshot>>> {
                ::semio_os_kernel::os_store::owned_schema_decode::ArtifactEnvelopeFreshVcsAuthority::try_new(
                    self.begin_snapshot(replacement, path),
                    ::std::sync::Arc::new(::semio_framework_value::OwnedValueRetirementFactory::<$snapshot>::default()),
                    ::std::sync::Arc::new(::semio_framework_value::OwnedValueRetirementFactory::<$mutation>::default()),
                    self.edit_history_decoder(),
                ).map(|owner| Box::new(owner) as Box<dyn ::semio_os_kernel::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<$snapshot, $mutation>>)
            }

            fn maximum_vcs_close_byte_demand(&self) -> usize { ::semio_os_kernel::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES }
            fn maximum_retained_vcs_close_bytes(&self) -> usize { ::semio_os_kernel::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES }

            fn begin_snapshot(&self, replacement: Option<::semio_framework_job::ReplacementIdentity>, path: ::semio_os_kernel::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn ::semio_os_kernel::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<$snapshot>> {
                Box::new($fields::PackField::<$snapshot>::new(replacement, path))
            }

            fn begin_mutation(&self, replacement: Option<::semio_framework_job::ReplacementIdentity>, path: ::semio_os_kernel::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn ::semio_os_kernel::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority<$mutation>> {
                Box::new($fields::PackField::<$mutation>::new(replacement, path))
            }

            fn begin_spr_conflict(&self, _replacement: Option<::semio_framework_job::ReplacementIdentity>, path: ::semio_os_kernel::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn ::semio_os_kernel::os_store::owned_schema_decode::ArtifactEnvelopeSprConflictAuthority> {
                Box::new($fields::FreshConflictRefusal { path })
            }

            fn edit_history_decoder(&self) -> ::std::sync::Arc<dyn ::semio_os_kernel::os_store::owned_schema_decode::ArtifactOwnedHistoryEntryDecoder<::semio_os_kernel::Edit<$mutation>>> {
                ::semio_os_kernel::os_store::owned_schema_decode::artifact_owned_spr_edit_history_decoder(::std::sync::Arc::new(Self), ::std::sync::Arc::new(::semio_framework_value::OwnedValueRetirementFactory::<$mutation>::default()))
            }
        }

        /// ✉️ Supplies the retained fresh-envelope schema owner and exact native retirement factories.
        pub fn $bundle() -> ::semio_os_kernel::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<$snapshot, $mutation> {
            ::semio_os_kernel::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle::new(
                ::std::sync::Arc::new(FemFieldCatalog),
                ::std::sync::Arc::new(::semio_framework_value::OwnedValueRetirementFactory::<$snapshot>::default()),
                ::std::sync::Arc::new(::semio_framework_value::OwnedValueRetirementFactory::<$mutation>::default()),
            )
        }
    };
}
