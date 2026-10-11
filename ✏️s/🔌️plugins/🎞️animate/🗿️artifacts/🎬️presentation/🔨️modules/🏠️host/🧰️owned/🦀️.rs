//! 🏠️ Artifact document-store and publication authorities.

use crate::standards::v1::subsets::any::schema::empty_presentation_snapshot;
use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;
use crate::{PresentationSnapshot, PRESENTATION_DOCUMENT_SCHEMA};
use protocol::{Mutation as _, MutationDiff as _, OpBinary};
use store::ArtifactOwnedValueRetirementFactory as _;
use store::{create_document_envelope, ArtifactEnvelope, ArtifactStore};
pub type PresentationStore = ArtifactStore<PresentationSnapshot, PresentationMutation>;

/// 🔐️ Opens a Presentation store WITH its exact owner catalog installed. `ArtifactStore::new`
/// installs no catalog, and `reserve_edit_history_slot` then refuses every `Apply`
/// (`edit history insertion requires its exact mutation retirement factory`) — a bare
/// `PresentationStore::new` can be READ but never mutated, undone or closed. The app installs the
/// same catalog through `ArtifactEditor::build_document_store_owners`; every standalone store goes
/// through here instead. Mirrors `🕸️dag`'s `new_dag_store`.
pub async fn new_presentation_store(envelope: PresentationEnvelope, actor: protocol::ActorId) -> Result<OwnedPresentationStore, store::VcsError> {
    let mut store = PresentationStore::new(envelope, actor).await?;
    store.install_document_store_owners_exact(semio_framework_os_kernel::os_store::funded_bounded_artifact_store_owners::<PresentationSnapshot, PresentationMutation>().expect("funded bounded document owners")).unwrap_or_else(|(error, _)| panic!("bounded document owners install refused: {error}"));
    Ok(OwnedPresentationStore(store))
}

/// 🔚 A standalone Presentation store that retires itself: `ArtifactStore::drop` panics
/// `artifact store reached Drop without its exact terminal-empty shallow-shell witness` unless the
/// store walked its bounded close loop first, so the guard runs that loop on drop (skipped while
/// unwinding, where the original panic is the report worth keeping). Derefs to the bare store for
/// every read and dispatch.
pub struct OwnedPresentationStore(PresentationStore);

impl OwnedPresentationStore {
    /// 🔚 Walks the exact bounded owner close loop to the terminal-empty witness.
    pub fn close(&mut self) {
        self.0.close_owned_unscheduled().expect("Presentation document store closes through its exact bounded owners");
    }
}

impl std::ops::Deref for OwnedPresentationStore {
    type Target = PresentationStore;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OwnedPresentationStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for OwnedPresentationStore {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.close();
        }
    }
}

pub type PresentationEnvelope = ArtifactEnvelope<PresentationSnapshot, PresentationMutation>;

/// 📦️ Creates an empty typed VCS envelope for a presentation deck document.
pub fn create_presentation_envelope(id: &str) -> PresentationEnvelope {
    create_document_envelope(PRESENTATION_DOCUMENT_SCHEMA, id, empty_presentation_snapshot(), None)
}

const PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

/// 📦️ Installs Presentation's exact field catalog and nested owner retirement factories as
/// one indivisible app decode authority.
pub fn presentation_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<PresentationSnapshot, PresentationMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(PresentationEnvelopeOwnedFieldCatalog), std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<PresentationSnapshot>::default()), std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<PresentationMutation>::default()))
}

impl store::ArtifactEnvelopeOwnedFieldCatalog<PresentationSnapshot, PresentationMutation> for PresentationEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Result<Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<PresentationSnapshot, PresentationMutation>>, Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<PresentationSnapshot>>> {
        store::ArtifactEnvelopeFreshVcsAuthority::try_new(
            self.begin_snapshot(operation, generation, path),
            std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<PresentationSnapshot>::default()),
            std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<PresentationMutation>::default()),
            self.edit_history_decoder(),
        )
        .map(|authority| Box::new(authority) as Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<PresentationSnapshot, PresentationMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<PresentationSnapshot>> {
        Box::new(PresentationPackSnapshotAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<PresentationMutation>> {
        Box::new(PresentationRejectedNestedAuthority { terminal: false, code: "presentation-envelope.fresh-mutation-not-admitted" })
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(PresentationRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<PresentationMutation>>> {
        store::artifact_bounded_history_entry_decoder()
    }
}



#[expect(clippy::large_enum_variant, reason = "The active decoder keeps its fixed path and admitted hex authority inline without a second allocation at the state transition.")]
enum PresentationPackSnapshotState {
    AwaitToken,
    Decode(store::OwnedSchemaHexAuthority<PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES>),
    Ready,
    Published,
    Closing,
    Complete,
}

struct PresentationPackSnapshotAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    state: PresentationPackSnapshotState,
    value: std::mem::ManuallyDrop<Option<PresentationSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl PresentationPackSnapshotAuthority {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self { operation, generation, path, state: PresentationPackSnapshotState::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path, refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() }
    }

    fn owners_terminal_empty(&self) -> bool {
        matches!(self.state, PresentationPackSnapshotState::Published | PresentationPackSnapshotState::Complete) && self.value.is_none() && self.retirement.is_none()
    }

    /// 📏️ Quotes the next close turn: the active hex decoder, the started retirement's own frontier, or the retirement's start.
    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, store::OwnedSchemaDecodeDiagnostic> {
        if let Some(retirement) = self.retirement.as_ref() {
            return retirement.next_demand(body).map_err(|error| self.diagnostic_from(error));
        }
        Ok(semio_framework_value::RetirementDemand { depth: usize::from(!self.owners_terminal_empty()), ..Default::default() })
    }

    fn diagnostic_from(&self, error: semio_framework_value::ValueError) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { refusal_kind: error.kind, retained_progress: error.retained_progress(), ..self.diagnostic("presentation-envelope.snapshot-retirement-fault", 0) }
    }
}

impl store::ArtifactEnvelopeSnapshotFieldAuthority<PresentationSnapshot> for PresentationPackSnapshotAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path, refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() };
        if matches!(self.state, PresentationPackSnapshotState::AwaitToken) {
            if !terminal {
                return Err(diagnostic("presentation-envelope.snapshot-pack-must-be-scalar", token.start));
            }
            self.state = PresentationPackSnapshotState::Decode(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
        }
        let PresentationPackSnapshotState::Decode(authority) = &mut self.state else {
            return Err(diagnostic("presentation-envelope.snapshot-pack-token-replayed", token.start));
        };
        match authority.step(source, cx) {
            store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
            store::OwnedSchemaHexStep::Complete => {
                let bytes = authority.as_bytes().ok_or_else(|| diagnostic("presentation-envelope.snapshot-pack-missing", token.start))?;
                let value = <PresentationSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|_| diagnostic("presentation-envelope.snapshot-pack-malformed", token.start))?;
                assert!(authority.release(), "completed Presentation snapshot pack releases its inline bytes exactly once");
                *self.value = Some(value);
                self.state = PresentationPackSnapshotState::Ready;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            store::OwnedSchemaHexStep::Cancelled => Err(diagnostic("presentation-envelope.snapshot-pack-cancelled", token.start)),
            store::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
        }
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::ArtifactEnvelopeSnapshotFieldTarget<PresentationSnapshot>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, PresentationPackSnapshotState::Ready) {
            return Err(self.diagnostic("presentation-envelope.snapshot-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("presentation-envelope.snapshot-owner-missing", 0))?;
        target.publish_snapshot_reserved(reservation, value);
        self.state = PresentationPackSnapshotState::Published;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn maximum_close_byte_demand(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(maximum_copy_bytes).map(|demand| demand.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.depth)
    }

    fn close_step(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> Result<semio_framework_value::RetainedCloneStep, store::OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::{RetainedCloneProgress, RetainedCloneStep};
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        if let PresentationPackSnapshotState::Decode(authority) = &mut self.state {
            authority.cancel();
            self.state = PresentationPackSnapshotState::Closing;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                let factory = semio_framework_value::retirement::OwnedValueRetirementFactory::<PresentationSnapshot>::default();
                return match store::ArtifactOwnedValueRetirementFactory::retire_owned(&factory, value, grant) {
                    Ok((retirement, progress)) => {
                        *self.retirement = Some(retirement);
                        self.state = PresentationPackSnapshotState::Closing;
                        Ok(RetainedCloneStep::Progress(progress))
                    }
                    Err((error, value)) => {
                        *self.value = Some(value);
                        if matches!(error.kind, semio_framework_value::ValueRefusalKind::InvariantViolated | semio_framework_value::ValueRefusalKind::InvalidValue) {
                            Err(self.diagnostic_from(error))
                        } else {
                            Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()))
                        }
                    }
                };
            }
            self.state = PresentationPackSnapshotState::Complete;
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        let step = self.retirement.as_mut().expect("Presentation snapshot retirement remains retained").close_step(grant).map_err(|error| self.diagnostic_from(error))?;
        if self.retirement.as_ref().is_some_and(|retirement| retirement.terminal_is_empty()) {
            drop(self.retirement.take());
            self.state = PresentationPackSnapshotState::Complete;
            return Ok(RetainedCloneStep::Complete(step.progress()));
        }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners_terminal_empty()
    }
}

impl Drop for PresentationPackSnapshotAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owners_terminal_empty()), "Presentation pack snapshot authority reached Drop before publication or bounded retirement");
    }
}

struct PresentationRejectedNestedAuthority {
    terminal: bool,
    code: &'static str,
}

impl store::ArtifactEnvelopeMutationFieldAuthority<PresentationMutation> for PresentationRejectedNestedAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { offset: token.start, ..store::OwnedSchemaDecodeDiagnostic::before(self.code, store::OwnedSchemaPath::ROOT) })
    }

    fn publish_reserved(
        &mut self,
        _target: &mut dyn store::ArtifactEnvelopeMutationFieldTarget<PresentationMutation>,
        _reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic::before(self.code, store::OwnedSchemaPath::ROOT))
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(usize::from(!self.terminal))
    }

    fn close_step(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> Result<semio_framework_value::RetainedCloneStep, store::OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::{RetainedCloneProgress, RetainedCloneStep};
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

struct PresentationRejectedConflictAuthority {
    terminal: bool,
}

impl store::ArtifactEnvelopeSprConflictAuthority for PresentationRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { offset: token.start, ..store::OwnedSchemaDecodeDiagnostic::before("presentation-envelope.fresh-conflict-not-admitted", store::OwnedSchemaPath::ROOT) })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(usize::from(!self.terminal))
    }

    fn close_step(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> Result<semio_framework_value::RetainedCloneStep, store::OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::{RetainedCloneProgress, RetainedCloneStep};
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

/// 🎭️ Owner-local exact catalog for the Presentation fresh-envelope decode cohort.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct PresentationEnvelopeOwnedFieldCatalog;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
