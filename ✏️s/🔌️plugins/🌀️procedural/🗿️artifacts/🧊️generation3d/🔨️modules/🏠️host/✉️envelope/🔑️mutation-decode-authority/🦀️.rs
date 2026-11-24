//! 🔑️ Mutation decode authority of the owned envelope catalog: the persisted hex bytes feed the typed Pack receiver archive replay uses, and the rejected conflict authority. See [`super`].

use super::*;
use protocol::{OpBinaryReceiving, OpBinaryReceivingCursor};
use semio_framework_job::{close_original_effect_step, receive_original_retained_work};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetirementDemand, ValueError, ValueRefusalKind};
use store::os_store::owned_schema_decode::{
    artifact_envelope_close_resume_with, artifact_envelope_close_settle, artifact_envelope_diagnostic_error, artifact_envelope_failure_diagnostic, ArtifactEnvelopeCloseExit, ArtifactEnvelopeClosePoll, ArtifactEnvelopeFieldDecodeStep,
    ArtifactEnvelopeFieldReservation, ArtifactEnvelopeMutationFieldAuthority, ArtifactEnvelopeMutationFieldTarget, OwnedSchemaDecodeDiagnostic, OwnedSchemaHexAuthority, OwnedSchemaHexStep, OwnedSchemaPath, OwnedSchemaRecordCursor, OwnedSchemaToken,
};

//#region 🔖️TypedOwnedEnvelopeCatalog
type MutationReceiver = <Generation3dMutation as OpBinaryReceiving>::Receiving;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Generation3dMutationDecodeState {
    AwaitToken,
    Hex,
    Receive,
    Ready,
    Published,
    Closing,
    Complete,
}

pub(super) struct Generation3dMutationDecodeAuthority {
    replacement: Option<semio_framework_job::ReplacementIdentity>,
    path: OwnedSchemaPath,
    pub(super) state: Generation3dMutationDecodeState,
    hex: Option<OwnedSchemaHexAuthority<GENERATION3D_OWNER_BYTES>>,
    reader: Option<MutationReceiver>,
    value: std::mem::ManuallyDrop<Option<Generation3dMutation>>,
    pub(super) retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}

impl Generation3dMutationDecodeAuthority {
    pub(super) fn new(replacement: Option<semio_framework_job::ReplacementIdentity>, path: OwnedSchemaPath) -> Self {
        Self { replacement, path, state: Generation3dMutationDecodeState::AwaitToken, hex: None, reader: None, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { offset, ..OwnedSchemaDecodeDiagnostic::before(code, self.path) }
    }

    fn backing_demand(&self, body: usize) -> Result<Option<RetirementDemand>, OwnedSchemaDecodeDiagnostic> {
        if let Some(reader) = self.reader.as_ref() {
            if reader.terminal_is_empty() {
                return Ok(Some(RetirementDemand { copy_bytes: size_of::<Option<MutationReceiver>>(), depth: 1, ..Default::default() }));
            }
            return reader.close_demands(body).map(Some).map_err(|error| self.diagnostic("generation3d-envelope.mutation-receiver-demand", 0).with_native(error));
        }
        Ok(self.hex.as_ref().map(|_| RetirementDemand { release_bytes: GENERATION3D_OWNER_BYTES, depth: 1, ..Default::default() }))
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        if let Some(demand) = self.backing_demand(body)? {
            return Ok(demand);
        }
        let native = |error| self.diagnostic("generation3d-envelope.mutation-close-demand", 0).with_native(error);
        if let Some(retirement) = self.retirement.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(retirement, body).map_err(native);
        }
        store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.value).map_err(native)
    }

    fn close_backing(&mut self, demand: RetirementDemand, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ArtifactEnvelopeCloseExit> {
        if let Some(reader) = self.reader.as_mut() {
            if reader.terminal_is_empty() {
                self.reader = None;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
            }
            let diagnostic = OwnedSchemaDecodeDiagnostic::before("generation3d-envelope.mutation-receiver-close", self.path);
            let step = artifact_envelope_close_resume_with(reader.close_step(effect), |failure| artifact_envelope_failure_diagnostic(diagnostic, &failure))?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(effect.grant(), step, reader.terminal_is_empty(), "original Generation3d mutation receiver").map_err(|error| diagnostic.with_native(error))?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(mut hex) = self.hex.take() {
            hex.cancel();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }

    fn receive(&mut self, offset: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        use ArtifactEnvelopeFieldDecodeStep::{FieldComplete, Pending};
        if self.value.is_some() {
            if let Some(demand) = self.backing_demand(cx.retained_grant().maximum_copy_bytes)? {
                let diagnostic = self.diagnostic("generation3d-envelope.mutation-receiver-close", offset);
                close_original_effect_step(cx, demand, |effect| {
                    let outcome = self.close_backing(demand, effect);
                    artifact_envelope_close_settle(effect, outcome).map_err(artifact_envelope_diagnostic_error)
                })
                .map_err(|error| diagnostic.with_progress(error.retained_progress()))?;
                return Ok(Pending);
            }
            self.state = Generation3dMutationDecodeState::Ready;
            return Ok(FieldComplete);
        }

        let diagnostic = self.diagnostic("generation3d-envelope.mutation-pack-receive", offset);
        let bytes = self.hex.as_ref().and_then(OwnedSchemaHexAuthority::as_bytes).ok_or(diagnostic)?;
        let grant = cx.retained_grant();
        let Some(reader) = self.reader.as_mut() else {
            let turn = Generation3dMutation::admit_receiving(bytes, (0, bytes.len()), grant);
            let result = turn.result.map(|admitted| {
                admitted.map_or(Default::default(), |(reader, progress)| {
                    self.reader = Some(reader);
                    progress
                })
            });
            receive_original_retained_work(cx, RetainedWorkOutcome { result: result.map(RetainedCloneStep::Progress), work: turn.work }).map_err(|error| diagnostic.with_progress(error.retained_progress()))?;
            return Ok(Pending);
        };

        if reader.is_ready() {
            let turn = reader.take_into(&mut self.value, grant);
            receive_original_retained_work(cx, RetainedWorkOutcome { result: turn.result.map(RetainedCloneStep::Progress), work: turn.work }).map_err(|error| diagnostic.with_progress(error.retained_progress()))?;
            return Ok(Pending);
        }

        let mut scope = None;
        let run = cx.with_native_receiving(&mut scope, |native, issued| {
            let before = native.actual_work();
            let turn = reader.step(bytes, native, issued.maximum_copy_bytes, issued);
            let performed = native.actual_work().checked_sub(before);
            if performed.is_none_or(|performed| turn.work > performed) {
                let progress = match &turn.result {
                    Ok(progress) => *progress,
                    Err(error) => error.retained_progress(),
                };
                return Err(ValueError::invariant("Generation3d mutation reported work outside its native receiving scope").with_retained_progress(progress));
            }
            turn.result.map(|_| ()).map_err(|error| {
                let refusal_kind = error.refusal().map_or(ValueRefusalKind::InvalidValue, |error| error.kind);
                ValueError::literal(refusal_kind, diagnostic.code).with_retained_progress(error.retained_progress())
            })
        });
        if scope.is_some() {
            cx.settle_receiving_scope().map_err(|error| diagnostic.with_native(error))?;
        }
        if run.map_err(|error| diagnostic.with_progress(error.retained_progress()))? {
            scope.ok_or(diagnostic)?.into_result().map_err(|error| diagnostic.with_native(error))?;
        }
        Ok(Pending)
    }
}

impl ArtifactEnvelopeMutationFieldAuthority<Generation3dMutation> for Generation3dMutationDecodeAuthority {
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.depth)
    }

    fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if cx.replacement_is_stale(self.replacement) || cx.is_cancelled() {
            return Err(self.diagnostic("generation3d-envelope.mutation-stale-authority", token.start));
        }
        if self.state == Generation3dMutationDecodeState::AwaitToken {
            if !terminal {
                return Err(self.diagnostic("generation3d-envelope.mutation-pack-must-be-scalar", token.start));
            }
            self.hex = Some(OwnedSchemaHexAuthority::try_new(self.replacement, token, self.path)?);
            self.state = Generation3dMutationDecodeState::Hex;
        }
        if self.state == Generation3dMutationDecodeState::Hex {
            return match self.hex.as_mut().expect("Generation3d mutation hex owner retained").step(source, cx) {
                OwnedSchemaHexStep::Pending => Ok(ArtifactEnvelopeFieldDecodeStep::Pending),
                OwnedSchemaHexStep::Complete => {
                    self.state = Generation3dMutationDecodeState::Receive;
                    Ok(ArtifactEnvelopeFieldDecodeStep::Pending)
                }
                OwnedSchemaHexStep::Cancelled => Err(self.diagnostic("generation3d-envelope.mutation-pack-cancelled", token.start)),
                OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
            };
        }
        if self.state != Generation3dMutationDecodeState::Receive {
            return Err(self.diagnostic("generation3d-envelope.mutation-token-replayed", token.start));
        }
        self.receive(token.start, cx)
    }

    fn publish_reserved(&mut self, target: &mut dyn ArtifactEnvelopeMutationFieldTarget<Generation3dMutation>, reservation: ArtifactEnvelopeFieldReservation, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if self.state != Generation3dMutationDecodeState::Ready {
            return Err(self.diagnostic("generation3d-envelope.mutation-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("generation3d-envelope.mutation-owner-missing", 0))?;
        target.publish_mutation_reserved(reservation, value);
        self.state = Generation3dMutationDecodeState::Published;
        Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> ArtifactEnvelopeClosePoll {
        let result = (|| -> Result<RetainedCloneStep, ArtifactEnvelopeCloseExit> {
            let grant = effect.grant();
            let empty = RetainedCloneProgress::default();
            if self.terminal_is_empty() {
                return Ok(RetainedCloneStep::Complete(empty));
            }
            let demand = self.close_demands(grant.maximum_copy_bytes)?;
            if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) {
                return Ok(RetainedCloneStep::Progress(empty));
            }
            self.state = Generation3dMutationDecodeState::Closing;
            if self.reader.is_some() || self.hex.is_some() {
                return self.close_backing(demand, effect);
            }
            let diagnostic = self.diagnostic("generation3d-envelope.mutation-retirement-fault", 0);
            if self.retirement.is_some() {
                let step = artifact_envelope_close_resume_with(store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant.maximum_copy_bytes, effect), |failure| artifact_envelope_failure_diagnostic(diagnostic, &failure))?;
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            if self.value.is_some() {
                let step = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, effect).result.map_err(|error| diagnostic.with_native(error))?;
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            self.state = Generation3dMutationDecodeState::Complete;
            Ok(RetainedCloneStep::Complete(empty))
        })();
        artifact_envelope_close_settle(effect, result)
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.state, Generation3dMutationDecodeState::Published | Generation3dMutationDecodeState::Complete) && self.hex.is_none() && self.reader.is_none() && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for Generation3dMutationDecodeAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "Generation3d mutation authority reached Drop before publication or terminal-empty close");
    }
}

pub(super) struct Generation3dRejectedConflictAuthority {
    pub(super) terminal: bool,
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeSprConflictAuthority for Generation3dRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Err(store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code: "generation3d-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::os_store::owned_schema_decode::OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(usize::from(!self.terminal))
    }

    fn close_step(&mut self, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> store::os_store::owned_schema_decode::ArtifactEnvelopeClosePoll {
        let result = (|| -> Result<semio_framework_value::retained_clone::RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        let grant = effect.grant();
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        if self.terminal {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    })();
        store::os_store::owned_schema_decode::artifact_envelope_close_settle(effect, result.map_err(Into::into))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}
//#endregion 🔖️TypedOwnedEnvelopeCatalog
