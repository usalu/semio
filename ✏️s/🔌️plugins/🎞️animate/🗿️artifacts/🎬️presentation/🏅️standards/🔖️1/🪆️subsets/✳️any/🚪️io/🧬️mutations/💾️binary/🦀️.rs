//! 📡️ Animate presentation artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`).
//!
//! Also hosts the `PresentationEnvelope`/`PresentationStore` type aliases and the VCS envelope helpers — both need
//! `PresentationMutation` (from `crate::artifacts::presentation::op`) alongside `PresentationSnapshot` (from the artifact's
//! own component file), so this is the natural home for them.
//!
//! The app's typed `PresentationCommand` enum — which used to share the old `📡️protocol` crate with this
//! codec — is an APP concern, not an artifact one: it now lives in `✏️editor/🦀️.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`. Its
//! WASM bridge moved to `✏️editor/🌉️wasm/🦀️component.rs`.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::presentation::schema::mutations::PresentationMutation;
use crate::artifacts::presentation::schema::{empty_presentation_snapshot, PresentationError};
use crate::artifacts::presentation::{PresentationSnapshot, PRESENTATION_DOCUMENT_SCHEMA};
use protocol::{Mutation as _, MutationDiff as _, OpBinary};
use store::{create_document_envelope, ArtifactEnvelope, ArtifactStore};

//#region 🧬️OwnedEnvelopeCatalog
const PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

struct PresentationFreshSnapshotRetirement {
    value: std::mem::ManuallyDrop<Option<PresentationSnapshot>>,
}

impl store::ErasedSnapshotRetirement for PresentationFreshSnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 || maximum_bytes < PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(value) = self.value.take() {
            drop(value);
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none()
    }
}

impl Drop for PresentationFreshSnapshotRetirement {
    fn drop(&mut self) {
        assert!(self.value.is_none(), "Presentation fresh snapshot retirement reached Drop before its <=4096-byte admitted root was released");
    }
}

struct PresentationFreshSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<PresentationSnapshot> for PresentationFreshSnapshotRetirementFactory {
    fn retire_owned(&self, value: PresentationSnapshot) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(PresentationFreshSnapshotRetirement { value: std::mem::ManuallyDrop::new(Some(value)) })
    }
}

struct PresentationUnexpectedMutationRetirement {
    value: std::mem::ManuallyDrop<Option<PresentationMutation>>,
}

impl store::ErasedSnapshotRetirement for PresentationUnexpectedMutationRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if self.value.is_none() {
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        if maximum_items == 0 || maximum_bytes < store::ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        drop(self.value.take());
        Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES })
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none()
    }
}

impl Drop for PresentationUnexpectedMutationRetirement {
    fn drop(&mut self) {
        assert!(self.value.is_none(), "fresh Presentation mutation retirement fail-closed with an impossible populated-history owner");
    }
}

struct PresentationUnexpectedMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<PresentationMutation> for PresentationUnexpectedMutationRetirementFactory {
    fn retire_owned(&self, value: PresentationMutation) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(PresentationUnexpectedMutationRetirement { value: std::mem::ManuallyDrop::new(Some(value)) })
    }
}

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
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
    }

    fn owners_terminal_empty(&self) -> bool {
        matches!(self.state, PresentationPackSnapshotState::Published | PresentationPackSnapshotState::Complete) && self.value.is_none() && self.retirement.is_none()
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
        let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path };
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

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let diagnostic = |code: &'static str| store::OwnedSchemaDecodeDiagnostic { code, offset: 0, line: 0, column: 0, path };
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let PresentationPackSnapshotState::Decode(authority) = &mut self.state {
            authority.cancel();
            self.state = PresentationPackSnapshotState::Closing;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&PresentationFreshSnapshotRetirementFactory, value));
                self.state = PresentationPackSnapshotState::Closing;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.state = PresentationPackSnapshotState::Complete;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let retirement = self.retirement.as_mut().expect("Presentation snapshot retirement remains retained");
        match retirement.close_step(maximum_items, maximum_bytes).map_err(|_| diagnostic("presentation-envelope.snapshot-retirement-fault"))? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                self.state = PresentationPackSnapshotState::Complete;
                Ok(store::SnapshotRetirementStep::Complete)
            }
            store::SnapshotRetirementStep::Complete => Err(diagnostic("presentation-envelope.snapshot-retirement-false-terminal")),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners_terminal_empty()
    }
}

impl Drop for PresentationPackSnapshotAuthority {
    fn drop(&mut self) {
        assert!(self.owners_terminal_empty(), "Presentation pack snapshot authority reached Drop before publication or bounded retirement");
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
        Err(store::OwnedSchemaDecodeDiagnostic { code: self.code, offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })
    }

    fn publish_reserved(
        &mut self,
        _target: &mut dyn store::ArtifactEnvelopeMutationFieldTarget<PresentationMutation>,
        _reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: self.code, offset: 0, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
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
        Err(store::OwnedSchemaDecodeDiagnostic { code: "presentation-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

/// 🎭️ Owner-local exact catalog for the Presentation fresh-envelope decode cohort.
pub struct PresentationEnvelopeOwnedFieldCatalog;

/// 📦️ Installs Presentation's exact field catalog and nested owner retirement factories as
/// one indivisible app decode authority.
pub fn presentation_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<PresentationSnapshot, PresentationMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(PresentationEnvelopeOwnedFieldCatalog), std::sync::Arc::new(PresentationFreshSnapshotRetirementFactory), std::sync::Arc::new(PresentationUnexpectedMutationRetirementFactory))
}

impl store::ArtifactEnvelopeOwnedFieldCatalog<PresentationSnapshot, PresentationMutation> for PresentationEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<PresentationSnapshot, PresentationMutation>> {
        Box::new(store::ArtifactEnvelopeFreshVcsAuthority::new(
            self.begin_snapshot(operation, generation, path),
            std::sync::Arc::new(PresentationFreshSnapshotRetirementFactory),
            std::sync::Arc::new(PresentationUnexpectedMutationRetirementFactory),
            self.edit_history_decoder(),
        ))
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

struct PresentationProjectionCompletionState {
    value: std::mem::ManuallyDrop<Option<PresentationSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl Drop for PresentationProjectionCompletionState {
    fn drop(&mut self) {
        assert!(self.value.is_none() && self.retirement.is_none(), "Presentation projection completion reached Drop before its exact typed result was consumed or retired");
    }
}

/// 🎯️ Nonblocking publication target for one completed Presentation snapshot owner.
pub trait PresentationProjectionAdoptionTarget {
    fn try_adopt(&mut self, value: PresentationSnapshot) -> Result<(), PresentationSnapshot>;
}

/// 🎫️ Pollable exact-once typed result retained outside the worker job.
pub struct PresentationProjectionCompletion {
    state: std::sync::Mutex<PresentationProjectionCompletionState>,
}

impl PresentationProjectionCompletion {
    fn new() -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self { state: std::sync::Mutex::new(PresentationProjectionCompletionState { value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }) })
    }

    /// 📤️ Atomically transfers the exact completed owner or retains it on backpressure.
    pub fn try_publish_to(&self, target: &mut dyn PresentationProjectionAdoptionTarget) -> Result<bool, ()> {
        let mut state = self.state.try_lock().map_err(|_| ())?;
        let Some(value) = state.value.take() else { return Ok(false) };
        match target.try_adopt(value) {
            Ok(()) => Ok(true),
            Err(value) => {
                *state.value = Some(value);
                Ok(false)
            }
        }
    }

    fn close_step(&self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        let mut state = match self.state.try_lock() {
            Ok(state) => state,
            Err(_) => return Ok(store::SnapshotRetirementStep::Blocked),
        };
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if state.retirement.is_none() {
            if let Some(value) = state.value.take() {
                *state.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&PresentationFreshSnapshotRetirementFactory, value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let retirement = state.retirement.as_mut().expect("Presentation projection retirement remains retained");
        match retirement.close_step(maximum_items, maximum_bytes)? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(state.retirement.take());
                Ok(store::SnapshotRetirementStep::Complete)
            }
            store::SnapshotRetirementStep::Complete => Err("Presentation projection retirement reported Complete without its terminal-empty witness".into()),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.state.try_lock().is_ok_and(|state| state.value.is_none() && state.retirement.is_none())
    }
}

struct PresentationProjectionTarget<'a> {
    envelope: &'a mut Option<PresentationEnvelope>,
}

impl store::ArtifactEnvelopeCompletedRecordTarget<PresentationSnapshot, PresentationMutation> for PresentationProjectionTarget<'_> {
    fn try_adopt_completed(&mut self, envelope: PresentationEnvelope) -> Result<(), PresentationEnvelope> {
        if self.envelope.is_some() {
            return Err(envelope);
        }
        *self.envelope = Some(envelope);
        Ok(())
    }
}

enum PresentationEnvelopeMaterializeState {
    Decode,
    Publish,
    Materialize,
    RetireEnvelopeComplete,
    RetireEnvelopeCancelled,
    RetireEnvelopeFault,
    CloseCompleted,
    Complete,
    Cancelled,
    Fault,
}

/// 🛠️ One concrete persistent production caller for the 12-field Presentation fresh-envelope catalog.
/// The shared WorkerJobSession drives this job one step at a time; no caller-facing method loops.
pub struct PresentationEnvelopeMaterializeJob {
    decode: Option<store::ArtifactEnvelopeDecodeAuthority<PresentationSnapshot, PresentationMutation>>,
    field_registry: std::sync::Arc<store::ArtifactEnvelopeFieldDecoderRegistry<PresentationSnapshot, PresentationMutation>>,
    field_retirement: Option<store::ArtifactEnvelopeReturnedFieldDecoder<PresentationSnapshot, PresentationMutation>>,
    completed_registry: std::sync::Arc<store::ArtifactEnvelopeCompletedRecordRegistry<PresentationSnapshot, PresentationMutation>>,
    completed_retirement: Option<Box<dyn store::ArtifactEnvelopeCompletedRecord<PresentationSnapshot, PresentationMutation>>>,
    decode_completion: std::sync::Arc<store::ArtifactEnvelopeDecodeCompletion>,
    projection: std::sync::Arc<PresentationProjectionCompletion>,
    materialize_envelope: std::mem::ManuallyDrop<Option<PresentationEnvelope>>,
    materialize_snapshot: std::mem::ManuallyDrop<Option<PresentationSnapshot>>,
    materialize_snapshot_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    materialize_envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    materialize_edit: usize,
    materialize_mutation: usize,
    state: PresentationEnvelopeMaterializeState,
    fault_code: Option<&'static [u8]>,
    fault_writer: std::mem::ManuallyDrop<Option<semio_framework_job::RetainedJobPayloadWriter>>,
    fault_cursor: usize,
    fault_payload: std::mem::ManuallyDrop<Option<semio_framework_job::RetainedJobPayload>>,
    retained_nested_outcome: std::mem::ManuallyDrop<Option<semio_framework_job::StepOutcome>>,
    closing: bool,
}

impl PresentationEnvelopeMaterializeJob {
    fn pump_field_return(&mut self) -> Result<bool, store::OwnedSchemaDecodeDiagnostic> {
        if let Some(retirement) = self.field_retirement.as_mut() {
            let step = store::ErasedSnapshotRetirement::close_step(retirement, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|_| store::OwnedSchemaDecodeDiagnostic {
                code: "presentation-envelope.field-return-fault",
                offset: 0,
                line: 0,
                column: 0,
                path: store::OwnedSchemaPath::ROOT,
            })?;
            if step == store::SnapshotRetirementStep::Complete {
                if !store::ErasedSnapshotRetirement::terminal_is_empty(retirement) {
                    return Err(store::OwnedSchemaDecodeDiagnostic { code: "presentation-envelope.field-return-false-terminal", offset: 0, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT });
                }
                drop(self.field_retirement.take());
            }
            return Ok(true);
        }
        let Some(ticket) = self.field_registry.next_returned_ticket() else { return Ok(false) };
        match self.field_registry.take_returned_ticket(ticket) {
            Ok(retirement) => {
                self.field_retirement = Some(retirement);
                Ok(true)
            }
            Err(store::ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => Ok(true),
            Err(_) => Err(store::OwnedSchemaDecodeDiagnostic { code: "presentation-envelope.field-return-stale", offset: 0, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT }),
        }
    }

    fn begin_completed_close(&mut self) -> Result<(), store::OwnedSchemaDecodeDiagnostic> {
        let Some(ticket) = self.decode_completion.ticket() else {
            self.state = PresentationEnvelopeMaterializeState::Cancelled;
            return Ok(());
        };
        self.completed_registry.try_request_close(ticket).map_err(|_| store::OwnedSchemaDecodeDiagnostic { code: "presentation-envelope.completed-close-request", offset: 0, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })?;
        match self.completed_registry.try_detach(ticket) {
            Ok(owner) => {
                self.completed_retirement = Some(owner);
                self.state = PresentationEnvelopeMaterializeState::CloseCompleted;
                Ok(())
            }
            Err(store::ArtifactEnvelopeCompletedRecordFault::Contended) => Ok(()),
            Err(_) => Err(store::OwnedSchemaDecodeDiagnostic { code: "presentation-envelope.completed-close-stale", offset: 0, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT }),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.decode.is_none()
            && self.field_retirement.is_none()
            && self.field_registry.terminal_is_empty()
            && self.completed_retirement.is_none()
            && self.completed_registry.terminal_is_empty()
            && self.materialize_envelope.is_none()
            && self.materialize_snapshot.is_none()
            && self.materialize_snapshot_retirement.is_none()
            && self.materialize_envelope_retirement.is_none()
            && self.fault_payload.is_none()
            && self.retained_nested_outcome.is_none()
            && self.fault_writer.as_ref().is_none_or(semio_framework_job::RetainedJobPayloadWriter::terminal_is_empty)
            && matches!(self.state, PresentationEnvelopeMaterializeState::Complete | PresentationEnvelopeMaterializeState::Cancelled | PresentationEnvelopeMaterializeState::Fault)
    }

    fn begin_materialize_retirement(&mut self, state: PresentationEnvelopeMaterializeState) {
        if let Some(snapshot) = self.materialize_snapshot.take() {
            *self.materialize_snapshot_retirement = Some(PresentationFreshSnapshotRetirementFactory.retire_owned(snapshot));
        }
        if let Some(envelope) = self.materialize_envelope.take() {
            *self.materialize_envelope_retirement = Some(presentation_envelope_decode_owner_bundle().retire_envelope(envelope));
        }
        self.state = state;
    }

    fn pump_materialize_retirement(&mut self) -> Result<bool, String> {
        if let Some(retirement) = self.materialize_snapshot_retirement.as_mut() {
            return match retirement.close_step(1, PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.materialize_snapshot_retirement.take());
                    Ok(false)
                }
                store::SnapshotRetirementStep::Complete => Err("Presentation materialized snapshot retirement completed without its terminal witness".into()),
                _ => Ok(false),
            };
        }
        if let Some(retirement) = self.materialize_envelope_retirement.as_mut() {
            return match retirement.close_step(1, store::ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.materialize_envelope_retirement.take());
                    Ok(true)
                }
                store::SnapshotRetirementStep::Complete => Err("Presentation materialized envelope retirement completed without its terminal witness".into()),
                _ => Ok(false),
            };
        }
        Ok(true)
    }

    fn record_fault(&mut self, code: &'static [u8]) {
        if self.fault_code.is_none() {
            self.fault_code = Some(code);
        }
        self.state = PresentationEnvelopeMaterializeState::Fault;
    }

    fn fault_outcome(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if let Some(detail) = self.fault_payload.take() {
            return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail });
        }
        let detail = self.fault_code.unwrap_or(b"presentation-envelope.materialize-fault");
        let Some(writer) = self.fault_writer.as_mut() else { return semio_framework_job::StepOutcome::Yield };
        match writer.write_slice_page(cx, detail, &mut self.fault_cursor) {
            Ok(true) => {
                let writer = self.fault_writer.take().expect("Presentation fault writer remains owned until its admitted page is sealed");
                match writer.finish() {
                    Ok(detail) => semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail }),
                    Err(writer) => {
                        *self.fault_writer = Some(writer);
                        semio_framework_job::StepOutcome::Yield
                    }
                }
            }
            Ok(false) | Err(_) => semio_framework_job::StepOutcome::Yield,
        }
    }
}

impl semio_framework_job::InteractiveJob for PresentationEnvelopeMaterializeJob {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if let Err(diagnostic) = self.pump_field_return() {
            self.record_fault(diagnostic.code.as_bytes());
        }
        match self.state {
            PresentationEnvelopeMaterializeState::Decode => {
                let Some(decode) = self.decode.as_mut() else {
                    self.record_fault(b"presentation-envelope.decode-owner-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                match semio_framework_job::InteractiveJob::step(decode, cx) {
                    semio_framework_job::StepOutcome::Yield => semio_framework_job::StepOutcome::Yield,
                    outcome @ (semio_framework_job::StepOutcome::PreviewReady(_) | semio_framework_job::StepOutcome::CheckpointReady(_)) => {
                        *self.retained_nested_outcome = Some(outcome);
                        self.record_fault(b"presentation-envelope.unexpected-decode-output");
                        semio_framework_job::StepOutcome::Yield
                    }
                    semio_framework_job::StepOutcome::Complete(candidate) => {
                        if !candidate.state.terminal_is_empty() || !candidate.output.terminal_is_empty() {
                            *self.retained_nested_outcome = Some(semio_framework_job::StepOutcome::Complete(candidate));
                            self.record_fault(b"presentation-envelope.unexpected-decode-terminal-output");
                            return semio_framework_job::StepOutcome::Yield;
                        }
                        if !decode.terminal_is_empty() {
                            self.record_fault(b"presentation-envelope.decode-false-terminal");
                            return semio_framework_job::StepOutcome::Yield;
                        }
                        drop(self.decode.take());
                        self.state = PresentationEnvelopeMaterializeState::Publish;
                        semio_framework_job::StepOutcome::Yield
                    }
                    semio_framework_job::StepOutcome::Cancelled => {
                        if decode.terminal_is_empty() {
                            drop(self.decode.take());
                            self.state = PresentationEnvelopeMaterializeState::Cancelled;
                            semio_framework_job::StepOutcome::Cancelled
                        } else {
                            self.record_fault(b"presentation-envelope.cancel-false-terminal");
                            semio_framework_job::StepOutcome::Yield
                        }
                    }
                    semio_framework_job::StepOutcome::Fault(fault) => {
                        if decode.terminal_is_empty() {
                            drop(self.decode.take());
                        }
                        *self.fault_payload = Some(fault.detail);
                        self.state = PresentationEnvelopeMaterializeState::Fault;
                        semio_framework_job::StepOutcome::Yield
                    }
                }
            }
            PresentationEnvelopeMaterializeState::Publish => {
                if cx.is_cancelled() {
                    if let Err(diagnostic) = self.begin_completed_close() {
                        self.record_fault(diagnostic.code.as_bytes());
                    }
                    return semio_framework_job::StepOutcome::Yield;
                }
                let Some(ticket) = self.decode_completion.ticket() else { return semio_framework_job::StepOutcome::Yield };
                let mut target = PresentationProjectionTarget { envelope: &mut self.materialize_envelope };
                match self.completed_registry.try_publish_to(ticket, &mut target) {
                    Ok(true) => {
                        self.state = PresentationEnvelopeMaterializeState::Materialize;
                        semio_framework_job::StepOutcome::Yield
                    }
                    Ok(false) | Err(store::ArtifactEnvelopeCompletedRecordFault::Contended) => semio_framework_job::StepOutcome::Yield,
                    Err(_) => {
                        self.record_fault(b"presentation-envelope.completed-publication-stale");
                        semio_framework_job::StepOutcome::Yield
                    }
                }
            }
            PresentationEnvelopeMaterializeState::Materialize => {
                if cx.is_cancelled() {
                    self.begin_materialize_retirement(PresentationEnvelopeMaterializeState::RetireEnvelopeCancelled);
                    return semio_framework_job::StepOutcome::Yield;
                }
                if self.materialize_snapshot_retirement.is_some() {
                    match self.pump_materialize_retirement() {
                        Ok(_) => return semio_framework_job::StepOutcome::Yield,
                        Err(_) => {
                            self.record_fault(b"presentation-envelope.materialize-retirement");
                            self.begin_materialize_retirement(PresentationEnvelopeMaterializeState::RetireEnvelopeFault);
                            return semio_framework_job::StepOutcome::Yield;
                        }
                    }
                }
                let Some(envelope) = self.materialize_envelope.as_ref() else {
                    self.record_fault(b"presentation-envelope.materialize-owner-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if self.materialize_snapshot.is_none() {
                    *self.materialize_snapshot = Some(envelope.vcs.initial_snapshot.clone());
                    cx.consume_fuel(PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES as u64);
                    return semio_framework_job::StepOutcome::Yield;
                }
                if let Some(edit) = envelope.vcs.edits.get(self.materialize_edit) {
                    if let Some(mutation) = edit.forwards.get(self.materialize_mutation) {
                        let current = self.materialize_snapshot.as_ref().expect("materialized snapshot authority was established");
                        let (diff, messages) = mutation.diff(current).into_parts();
                        if messages.iter().any(|message| message.level == protocol::Severity::Fatal) {
                            self.record_fault(b"presentation-envelope.materialize-fatal-mutation");
                            self.begin_materialize_retirement(PresentationEnvelopeMaterializeState::RetireEnvelopeFault);
                            return semio_framework_job::StepOutcome::Yield;
                        }
                        match diff.apply(current) {
                            Ok(next) => {
                                let previous = self.materialize_snapshot.take().expect("materialized snapshot remains owned");
                                *self.materialize_snapshot = Some(next);
                                *self.materialize_snapshot_retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&PresentationFreshSnapshotRetirementFactory, previous));
                                self.materialize_mutation += 1;
                                cx.consume_fuel(PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES as u64);
                                return semio_framework_job::StepOutcome::Yield;
                            }
                            Err(_) => {
                                self.record_fault(b"presentation-envelope.materialize-apply");
                                self.begin_materialize_retirement(PresentationEnvelopeMaterializeState::RetireEnvelopeFault);
                                return semio_framework_job::StepOutcome::Yield;
                            }
                        }
                    }
                    self.materialize_edit += 1;
                    self.materialize_mutation = 0;
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                let mut projection = match self.projection.state.try_lock() {
                    Ok(projection) => projection,
                    Err(_) => return semio_framework_job::StepOutcome::Yield,
                };
                if projection.value.is_some() {
                    return semio_framework_job::StepOutcome::Yield;
                }
                let snapshot = self.materialize_snapshot.take().expect("completed Presentation materialization owns its exact snapshot");
                *projection.value = Some(snapshot);
                drop(projection);
                self.begin_materialize_retirement(PresentationEnvelopeMaterializeState::RetireEnvelopeComplete);
                semio_framework_job::StepOutcome::Yield
            }
            PresentationEnvelopeMaterializeState::RetireEnvelopeComplete | PresentationEnvelopeMaterializeState::RetireEnvelopeCancelled | PresentationEnvelopeMaterializeState::RetireEnvelopeFault => match self.pump_materialize_retirement() {
                Ok(false) => semio_framework_job::StepOutcome::Yield,
                Ok(true) => match self.state {
                    PresentationEnvelopeMaterializeState::RetireEnvelopeComplete => {
                        self.state = PresentationEnvelopeMaterializeState::Complete;
                        semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                        })
                    }
                    PresentationEnvelopeMaterializeState::RetireEnvelopeCancelled => {
                        self.state = PresentationEnvelopeMaterializeState::Cancelled;
                        semio_framework_job::StepOutcome::Cancelled
                    }
                    PresentationEnvelopeMaterializeState::RetireEnvelopeFault => {
                        self.state = PresentationEnvelopeMaterializeState::Fault;
                        semio_framework_job::StepOutcome::Yield
                    }
                    _ => unreachable!("retained materialize retirement state was matched above"),
                },
                Err(_) => {
                    self.record_fault(b"presentation-envelope.materialize-retirement");
                    semio_framework_job::StepOutcome::Yield
                }
            },
            PresentationEnvelopeMaterializeState::CloseCompleted => {
                let Some(owner) = self.completed_retirement.as_mut() else {
                    self.state = PresentationEnvelopeMaterializeState::Cancelled;
                    return semio_framework_job::StepOutcome::Cancelled;
                };
                match owner.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
                    Ok(store::SnapshotRetirementStep::Complete) if owner.terminal_is_empty() => {
                        drop(self.completed_retirement.take());
                        self.state = PresentationEnvelopeMaterializeState::Cancelled;
                        semio_framework_job::StepOutcome::Cancelled
                    }
                    Ok(_) => semio_framework_job::StepOutcome::Yield,
                    Err(_) => {
                        self.record_fault(b"presentation-envelope.completed-retirement");
                        semio_framework_job::StepOutcome::Yield
                    }
                }
            }
            PresentationEnvelopeMaterializeState::Complete => semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
            }),
            PresentationEnvelopeMaterializeState::Cancelled => semio_framework_job::StepOutcome::Cancelled,
            PresentationEnvelopeMaterializeState::Fault => self.fault_outcome(cx),
        }
    }

    fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        if let Some(decode) = self.decode.as_mut() {
            semio_framework_job::InteractiveJob::begin_close(decode);
        }
        if let Some(writer) = self.fault_writer.as_mut() {
            writer.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        self.begin_close();
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some(decode) = self.decode.as_mut() {
            match semio_framework_job::InteractiveJob::close_step(decode, maximum_items, maximum_bytes) {
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::InteractiveJobCloseStep::Blocked => return semio_framework_job::InteractiveJobCloseStep::Blocked,
                semio_framework_job::InteractiveJobCloseStep::Complete if !decode.terminal_is_empty() => return semio_framework_job::InteractiveJobCloseStep::Blocked,
                semio_framework_job::InteractiveJobCloseStep::Complete => {
                    drop(self.decode.take());
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
            }
        }
        if let Some(retirement) = self.field_retirement.as_mut() {
            return match store::ErasedSnapshotRetirement::close_step(retirement, maximum_items, maximum_bytes) {
                Ok(store::SnapshotRetirementStep::Pending { released_items, released_bytes }) => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                Ok(store::SnapshotRetirementStep::Blocked) => semio_framework_job::InteractiveJobCloseStep::Blocked,
                Ok(store::SnapshotRetirementStep::Complete) if !store::ErasedSnapshotRetirement::terminal_is_empty(retirement) => semio_framework_job::InteractiveJobCloseStep::Blocked,
                Ok(store::SnapshotRetirementStep::Complete) => {
                    drop(self.field_retirement.take());
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
                Err(_) => semio_framework_job::InteractiveJobCloseStep::Blocked,
            };
        }
        if let Some(ticket) = self.field_registry.next_returned_ticket() {
            return match self.field_registry.take_returned_ticket(ticket) {
                Ok(retirement) => {
                    self.field_retirement = Some(retirement);
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }
                }
                Err(store::ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => semio_framework_job::InteractiveJobCloseStep::Blocked,
                Err(_) => semio_framework_job::InteractiveJobCloseStep::Blocked,
            };
        }
        if !self.field_registry.terminal_is_empty() {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if let Some(retirement) = self.completed_retirement.as_mut() {
            return match retirement.close_step(maximum_items, maximum_bytes) {
                Ok(store::SnapshotRetirementStep::Pending { released_items, released_bytes }) => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                Ok(store::SnapshotRetirementStep::Blocked) => semio_framework_job::InteractiveJobCloseStep::Blocked,
                Ok(store::SnapshotRetirementStep::Complete) if !retirement.terminal_is_empty() => semio_framework_job::InteractiveJobCloseStep::Blocked,
                Ok(store::SnapshotRetirementStep::Complete) => {
                    drop(self.completed_retirement.take());
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
                Err(_) => semio_framework_job::InteractiveJobCloseStep::Blocked,
            };
        }
        if !self.completed_registry.terminal_is_empty() {
            let Some(ticket) = self.decode_completion.ticket() else { return semio_framework_job::InteractiveJobCloseStep::Blocked };
            match self.completed_registry.try_request_close(ticket) {
                Ok(()) | Err(store::ArtifactEnvelopeCompletedRecordFault::Contended) => {}
                Err(_) if self.completed_registry.terminal_is_empty() => {}
                Err(_) => return semio_framework_job::InteractiveJobCloseStep::Blocked,
            }
            return match self.completed_registry.try_detach(ticket) {
                Ok(retirement) => {
                    self.completed_retirement = Some(retirement);
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }
                }
                Err(store::ArtifactEnvelopeCompletedRecordFault::Contended) => semio_framework_job::InteractiveJobCloseStep::Blocked,
                Err(_) if self.completed_registry.terminal_is_empty() => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 },
                Err(_) => semio_framework_job::InteractiveJobCloseStep::Blocked,
            };
        }
        if self.materialize_snapshot_retirement.is_none() {
            if let Some(snapshot) = self.materialize_snapshot.take() {
                *self.materialize_snapshot_retirement = Some(PresentationFreshSnapshotRetirementFactory.retire_owned(snapshot));
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
        }
        if let Some(retirement) = self.materialize_snapshot_retirement.as_mut() {
            return match retirement.close_step(maximum_items, maximum_bytes) {
                Ok(store::SnapshotRetirementStep::Pending { released_items, released_bytes }) => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                Ok(store::SnapshotRetirementStep::Blocked) => semio_framework_job::InteractiveJobCloseStep::Blocked,
                Ok(store::SnapshotRetirementStep::Complete) if !retirement.terminal_is_empty() => semio_framework_job::InteractiveJobCloseStep::Blocked,
                Ok(store::SnapshotRetirementStep::Complete) => {
                    drop(self.materialize_snapshot_retirement.take());
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
                Err(_) => semio_framework_job::InteractiveJobCloseStep::Blocked,
            };
        }
        if self.materialize_envelope_retirement.is_none() {
            if let Some(envelope) = self.materialize_envelope.take() {
                *self.materialize_envelope_retirement = Some(presentation_envelope_decode_owner_bundle().retire_envelope(envelope));
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
        }
        if let Some(retirement) = self.materialize_envelope_retirement.as_mut() {
            return match retirement.close_step(maximum_items, maximum_bytes) {
                Ok(store::SnapshotRetirementStep::Pending { released_items, released_bytes }) => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                Ok(store::SnapshotRetirementStep::Blocked) => semio_framework_job::InteractiveJobCloseStep::Blocked,
                Ok(store::SnapshotRetirementStep::Complete) if !retirement.terminal_is_empty() => semio_framework_job::InteractiveJobCloseStep::Blocked,
                Ok(store::SnapshotRetirementStep::Complete) => {
                    drop(self.materialize_envelope_retirement.take());
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
                Err(_) => semio_framework_job::InteractiveJobCloseStep::Blocked,
            };
        }
        if let Some(payload) = self.fault_payload.as_mut() {
            return match payload.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => {
                    drop(self.fault_payload.take());
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
            };
        }
        if let Some(outcome) = self.retained_nested_outcome.as_mut() {
            return match outcome.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => {
                    drop(self.retained_nested_outcome.take());
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
            };
        }
        if let Some(writer) = self.fault_writer.as_mut() {
            return match writer.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => {
                    drop(self.fault_writer.take());
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
            };
        }
        self.state = PresentationEnvelopeMaterializeState::Cancelled;
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        PresentationEnvelopeMaterializeJob::terminal_is_empty(self)
    }
}

impl Drop for PresentationEnvelopeMaterializeJob {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Presentation envelope materialize job reached Drop before every decode/completed owner was terminal empty");
    }
}

/// 🏗️ Builds the representative retained caller from already-admitted, sealed fixed pages.
fn begin_materialize_presentation_projection(
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    pages: store::OwnedSchemaDecodePages,
) -> Result<(PresentationEnvelopeMaterializeJob, std::sync::Arc<PresentationProjectionCompletion>), store::OwnedSchemaDecodePages> {
    let record = store::artifact_envelope_decode_record(operation, generation, pages)?;
    let field_registry = store::ArtifactEnvelopeFieldDecoderRegistry::new();
    let completed_registry = store::ArtifactEnvelopeCompletedRecordRegistry::new();
    let decode_completion = store::ArtifactEnvelopeDecodeCompletion::new();
    let projection = PresentationProjectionCompletion::new();
    let fields = Box::new(store::ArtifactEnvelopeFreshFieldDecoder::new(
        operation,
        generation,
        std::sync::Arc::new(PresentationEnvelopeOwnedFieldCatalog),
        std::sync::Arc::new(PresentationFreshSnapshotRetirementFactory),
        std::sync::Arc::new(PresentationUnexpectedMutationRetirementFactory),
        std::sync::Arc::clone(&completed_registry),
        std::sync::Arc::clone(&decode_completion),
    ));
    let decode = match store::ArtifactEnvelopeDecodeAuthority::try_new(record, &field_registry, fields) {
        Ok(decode) => decode,
        Err(_) => unreachable!("a fresh private field registry admits its first exact decoder owner"),
    };
    Ok((
        PresentationEnvelopeMaterializeJob {
            decode: Some(decode),
            field_registry,
            field_retirement: None,
            completed_registry,
            completed_retirement: None,
            decode_completion,
            projection: std::sync::Arc::clone(&projection),
            materialize_envelope: std::mem::ManuallyDrop::new(None),
            materialize_snapshot: std::mem::ManuallyDrop::new(None),
            materialize_snapshot_retirement: std::mem::ManuallyDrop::new(None),
            materialize_envelope_retirement: std::mem::ManuallyDrop::new(None),
            materialize_edit: 0,
            materialize_mutation: 0,
            state: PresentationEnvelopeMaterializeState::Decode,
            fault_code: None,
            fault_writer: std::mem::ManuallyDrop::new(Some(semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::Fault))),
            fault_cursor: 0,
            fault_payload: std::mem::ManuallyDrop::new(None),
            retained_nested_outcome: std::mem::ManuallyDrop::new(None),
            closing: false,
        },
        projection,
    ))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresentationEnvelopeMaterializeHandleStep {
    Pending,
    Progress,
    Ready,
    Cancelled,
    Fault,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PresentationEnvelopeMaterializeHandleState {
    Active,
    RetiringComplete,
    RetiringCancelled,
    WorkerComplete,
    WorkerCancelled,
    WorkerFault,
    Complete,
}

/// 🎛️ App-retained worker handle for one Presentation envelope materialization. Every call
/// submits or observes at most one shared-pool turn; close retains the exact job and result owner.
pub struct PresentationEnvelopeMaterializeHandle {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    cancel: semio_framework_job::CancelToken,
    session: std::mem::ManuallyDrop<Option<semio_framework_job::WorkerJobSession<PresentationEnvelopeMaterializeJob>>>,
    rejected: std::mem::ManuallyDrop<Option<semio_framework_job::WorkerJobSessionAdmissionRejected<PresentationEnvelopeMaterializeJob>>>,
    pending: Option<semio_framework_job::WorkerJobTicket>,
    retained_outcome: std::mem::ManuallyDrop<Option<semio_framework_job::StepOutcome>>,
    completion: std::sync::Arc<PresentationProjectionCompletion>,
    fault: std::mem::ManuallyDrop<Option<semio_framework_job::RetainedJobPayload>>,
    fault_code: Option<&'static [u8]>,
    close_started: bool,
    state: PresentationEnvelopeMaterializeHandleState,
}

impl PresentationEnvelopeMaterializeHandle {
    pub fn operation(&self) -> semio_framework_job::OperationId {
        self.operation
    }

    pub fn generation(&self) -> semio_framework_job::Generation {
        self.generation
    }

    pub fn cancel_now(&self) {
        self.cancel.cancel_now();
    }

    pub fn fault(&self) -> Option<&[u8]> {
        self.fault.as_ref().and_then(semio_framework_job::RetainedJobPayload::single_page).or(self.fault_code)
    }

    fn adopt_worker_terminal(&mut self, mut owner: semio_framework_job::WorkerJobOutcome<PresentationEnvelopeMaterializeJob>) -> PresentationEnvelopeMaterializeHandleStep {
        let outcome = owner.take_outcome();
        match outcome {
            semio_framework_job::StepOutcome::Complete(candidate) if candidate.state.terminal_is_empty() && candidate.output.terminal_is_empty() => {
                owner.begin_close();
                self.close_started = true;
                self.state = PresentationEnvelopeMaterializeHandleState::RetiringComplete;
                PresentationEnvelopeMaterializeHandleStep::Progress
            }
            semio_framework_job::StepOutcome::Complete(candidate) => {
                *self.retained_outcome = Some(semio_framework_job::StepOutcome::Complete(candidate));
                owner.begin_close();
                self.close_started = true;
                self.fault_code = Some(b"presentation-envelope.unexpected-terminal-output");
                self.state = PresentationEnvelopeMaterializeHandleState::WorkerFault;
                PresentationEnvelopeMaterializeHandleStep::Fault
            }
            semio_framework_job::StepOutcome::Cancelled => {
                owner.begin_close();
                self.close_started = true;
                self.state = PresentationEnvelopeMaterializeHandleState::RetiringCancelled;
                PresentationEnvelopeMaterializeHandleStep::Progress
            }
            semio_framework_job::StepOutcome::Fault(fault) => {
                *self.fault = Some(fault.detail);
                owner.begin_close();
                self.close_started = true;
                self.state = PresentationEnvelopeMaterializeHandleState::WorkerFault;
                PresentationEnvelopeMaterializeHandleStep::Fault
            }
            _ => unreachable!("only a terminal worker outcome reaches job recovery"),
        }
    }

    fn retire_session_step(&mut self, completed: PresentationEnvelopeMaterializeHandleState) -> PresentationEnvelopeMaterializeHandleStep {
        let Some(session) = self.session.as_ref() else {
            self.state = completed;
            return match completed {
                PresentationEnvelopeMaterializeHandleState::WorkerComplete => PresentationEnvelopeMaterializeHandleStep::Ready,
                PresentationEnvelopeMaterializeHandleState::WorkerCancelled => PresentationEnvelopeMaterializeHandleStep::Cancelled,
                _ => PresentationEnvelopeMaterializeHandleStep::Fault,
            };
        };
        if session.terminal_is_empty() {
            drop(self.session.take());
            self.state = completed;
            return PresentationEnvelopeMaterializeHandleStep::Progress;
        }
        match session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES.max(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)) {
            semio_framework_job::WorkerJobCloseStep::Blocked => PresentationEnvelopeMaterializeHandleStep::Pending,
            semio_framework_job::WorkerJobCloseStep::Pending { .. } | semio_framework_job::WorkerJobCloseStep::Complete => PresentationEnvelopeMaterializeHandleStep::Progress,
        }
    }

    /// 🪜️ Advances at most one retained worker submission or observation. A stale live
    /// generation atomically requests cancellation before another turn can be submitted.
    pub fn maintenance_step(&mut self, pool: &semio_framework_job::WorkerPool, live_generation: semio_framework_job::Generation) -> PresentationEnvelopeMaterializeHandleStep {
        match self.state {
            PresentationEnvelopeMaterializeHandleState::RetiringComplete => return self.retire_session_step(PresentationEnvelopeMaterializeHandleState::WorkerComplete),
            PresentationEnvelopeMaterializeHandleState::RetiringCancelled => return self.retire_session_step(PresentationEnvelopeMaterializeHandleState::WorkerCancelled),
            PresentationEnvelopeMaterializeHandleState::WorkerComplete => return PresentationEnvelopeMaterializeHandleStep::Ready,
            PresentationEnvelopeMaterializeHandleState::WorkerCancelled => return PresentationEnvelopeMaterializeHandleStep::Cancelled,
            PresentationEnvelopeMaterializeHandleState::WorkerFault => return PresentationEnvelopeMaterializeHandleStep::Fault,
            PresentationEnvelopeMaterializeHandleState::Complete => return PresentationEnvelopeMaterializeHandleStep::Complete,
            PresentationEnvelopeMaterializeHandleState::Active => {}
        }
        if live_generation != self.generation {
            self.cancel.cancel_now();
        }
        let Some(session) = self.session.as_ref() else {
            self.fault_code = Some(b"presentation-envelope.worker-session-missing");
            self.state = PresentationEnvelopeMaterializeHandleState::WorkerFault;
            return PresentationEnvelopeMaterializeHandleStep::Fault;
        };
        match session.poll() {
            semio_framework_job::WorkerJobPoll::Submitted => return PresentationEnvelopeMaterializeHandleStep::Pending,
            semio_framework_job::WorkerJobPoll::Outcome => {
                let Some(ticket) = self.pending.take() else {
                    self.fault_code = Some(b"presentation-envelope.worker-ticket-missing");
                    let _ = session.begin_close();
                    self.close_started = true;
                    self.state = PresentationEnvelopeMaterializeHandleState::WorkerFault;
                    return PresentationEnvelopeMaterializeHandleStep::Fault;
                };
                let Ok(mut owner) = session.take_outcome(ticket) else {
                    self.pending = Some(ticket);
                    return PresentationEnvelopeMaterializeHandleStep::Pending;
                };
                let outcome = owner.take_outcome();
                if outcome.terminal_is_empty() && !outcome.is_terminal() {
                    drop(outcome);
                    return match owner.resume() {
                        Ok(()) => PresentationEnvelopeMaterializeHandleStep::Progress,
                        Err(owner) => {
                            owner.begin_close();
                            self.close_started = true;
                            self.fault_code = Some(b"presentation-envelope.worker-resume-rejected");
                            self.state = PresentationEnvelopeMaterializeHandleState::WorkerFault;
                            PresentationEnvelopeMaterializeHandleStep::Fault
                        }
                    };
                }
                *self.retained_outcome = Some(outcome);
                owner.begin_close();
                self.close_started = true;
                self.fault_code = Some(b"presentation-envelope.unexpected-nonterminal-output");
                self.state = PresentationEnvelopeMaterializeHandleState::WorkerFault;
                return PresentationEnvelopeMaterializeHandleStep::Fault;
            }
            semio_framework_job::WorkerJobPoll::Terminal => {
                self.pending = None;
                return match session.take_terminal() {
                    Ok(owner) => self.adopt_worker_terminal(owner),
                    Err(_) => PresentationEnvelopeMaterializeHandleStep::Pending,
                };
            }
            semio_framework_job::WorkerJobPoll::Rejected => {
                self.pending = None;
                let Ok(rejected) = session.take_rejected() else { return PresentationEnvelopeMaterializeHandleStep::Pending };
                return match rejected.kind() {
                    semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated => {
                        rejected.resume();
                        PresentationEnvelopeMaterializeHandleStep::Progress
                    }
                    semio_framework_async::WorkerSubmitErrorKind::Shutdown | semio_framework_async::WorkerSubmitErrorKind::Poisoned => {
                        rejected.begin_close();
                        self.close_started = true;
                        self.fault_code = Some(b"presentation-envelope.worker-pool-closed");
                        self.state = PresentationEnvelopeMaterializeHandleState::WorkerFault;
                        PresentationEnvelopeMaterializeHandleStep::Fault
                    }
                };
            }
            semio_framework_job::WorkerJobPoll::Closing => return PresentationEnvelopeMaterializeHandleStep::Pending,
            semio_framework_job::WorkerJobPoll::TerminalEmpty => {
                drop(self.session.take());
                self.fault_code = Some(b"presentation-envelope.worker-empty-without-terminal");
                self.state = PresentationEnvelopeMaterializeHandleState::WorkerFault;
                return PresentationEnvelopeMaterializeHandleStep::Fault;
            }
            semio_framework_job::WorkerJobPoll::CheckedOut => return PresentationEnvelopeMaterializeHandleStep::Pending,
            semio_framework_job::WorkerJobPoll::Idle => {}
        }
        match session.try_submit_step(pool, semio_framework_job::Lane::Interactive) {
            Ok(ticket) => {
                self.pending = Some(ticket);
                PresentationEnvelopeMaterializeHandleStep::Progress
            }
            Err(semio_framework_job::WorkerJobSubmitFault::Contention(_)) | Err(semio_framework_job::WorkerJobSubmitFault::Pool(_)) | Err(semio_framework_job::WorkerJobSubmitFault::SequenceExhausted) => PresentationEnvelopeMaterializeHandleStep::Pending,
        }
    }

    /// 📤️ Publishes the exact ready snapshot once. Backpressure leaves it in this handle.
    pub fn try_publish_to(&mut self, target: &mut dyn PresentationProjectionAdoptionTarget) -> Result<bool, ()> {
        if self.state != PresentationEnvelopeMaterializeHandleState::WorkerComplete {
            return Ok(false);
        }
        let published = self.completion.try_publish_to(target)?;
        if published && self.completion.terminal_is_empty() {
            self.state = PresentationEnvelopeMaterializeHandleState::Complete;
        }
        Ok(published)
    }

    /// 🧹️ Cancels and cursor-retires the exact worker/result owner without a run loop.
    pub fn close_step(&mut self, pool: &semio_framework_job::WorkerPool, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.state == PresentationEnvelopeMaterializeHandleState::Complete {
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        self.cancel.cancel_now();
        if !self.close_started {
            if let Some(rejected) = self.rejected.as_mut() {
                rejected.begin_close();
            } else if let Some(session) = self.session.as_ref() {
                let _ = session.begin_close();
            }
            self.pending = None;
            self.close_started = true;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(outcome) = self.retained_outcome.as_mut() {
            return Ok(match outcome.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => store::SnapshotRetirementStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => {
                    drop(self.retained_outcome.take());
                    store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }
                }
            });
        }
        if let Some(fault) = self.fault.as_mut() {
            return Ok(match fault.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => store::SnapshotRetirementStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => {
                    drop(self.fault.take());
                    store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }
                }
            });
        }
        if !self.completion.terminal_is_empty() {
            return self.completion.close_step(maximum_items, maximum_bytes);
        }
        if let Some(rejected) = self.rejected.as_mut() {
            if rejected.terminal_is_empty() {
                drop(self.rejected.take());
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(match rejected.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => store::SnapshotRetirementStep::Pending { released_items, released_bytes },
                semio_framework_job::InteractiveJobCloseStep::Blocked => store::SnapshotRetirementStep::Blocked,
                semio_framework_job::InteractiveJobCloseStep::Complete => store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 },
            });
        }
        if let Some(session) = self.session.as_ref() {
            if session.terminal_is_empty() {
                drop(self.session.take());
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(match session.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::WorkerJobCloseStep::Pending { released_items, released_bytes } => store::SnapshotRetirementStep::Pending { released_items, released_bytes },
                semio_framework_job::WorkerJobCloseStep::Blocked => {
                    let _ = pool;
                    store::SnapshotRetirementStep::Blocked
                }
                semio_framework_job::WorkerJobCloseStep::Complete => store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 },
            });
        }
        self.fault_code = None;
        self.state = PresentationEnvelopeMaterializeHandleState::Complete;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state == PresentationEnvelopeMaterializeHandleState::Complete
            && self.session.is_none()
            && self.rejected.is_none()
            && self.pending.is_none()
            && self.retained_outcome.is_none()
            && self.completion.terminal_is_empty()
            && self.fault.is_none()
            && self.fault_code.is_none()
    }
}

impl Drop for PresentationEnvelopeMaterializeHandle {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Presentation envelope materialize handle reached Drop before worker, result, and fault owners were terminal empty");
    }
}

/// 📨️ Creates the sole app-retained Presentation materialization handle from sealed fixed pages.
pub fn submit_materialize_presentation_projection(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, pages: store::OwnedSchemaDecodePages) -> Result<PresentationEnvelopeMaterializeHandle, store::OwnedSchemaDecodePages> {
    let (job, completion) = begin_materialize_presentation_projection(operation, generation, pages)?;
    let cancel = semio_framework_job::root_cancel_token();
    let params = semio_framework_job::BatchJobParams {
        operation,
        generation,
        cancel: cancel.clone(),
        config: semio_framework_job::BatchDriveConfig { site: "presentation_envelope_materialize", stage: semio_framework_job::InteractiveStage::InteractiveStep, fuel_per_step: 64, step_budget_us: semio_framework_job::INTERACTIVE_LANE_WALL_US },
        now_us: semio_framework_job::default_now_us,
    };
    let (session, rejected, state, fault_code) = match semio_framework_job::WorkerJobSession::try_new(job, params) {
        Ok(session) => (Some(session), None, PresentationEnvelopeMaterializeHandleState::Active, None),
        Err(rejected) => (None, Some(rejected), PresentationEnvelopeMaterializeHandleState::WorkerFault, Some(b"presentation-envelope.worker-admission" as &'static [u8])),
    };
    Ok(PresentationEnvelopeMaterializeHandle {
        operation,
        generation,
        cancel,
        session: std::mem::ManuallyDrop::new(session),
        rejected: std::mem::ManuallyDrop::new(rejected),
        pending: None,
        retained_outcome: std::mem::ManuallyDrop::new(None),
        completion,
        fault: std::mem::ManuallyDrop::new(None),
        fault_code,
        close_started: false,
        state,
    })
}

pub const PRESENTATION_ENVELOPE_MATERIALIZE_CAPACITY: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresentationEnvelopeMaterializeRegistryFault {
    Capacity,
    Collision,
    Contended,
    Stale,
}

struct PresentationEnvelopeMaterializeSlot {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    occupied: bool,
    handle: std::mem::MaybeUninit<PresentationEnvelopeMaterializeHandle>,
}

/// 🗄️ Fixed app-owned maintenance registry for retained Presentation envelope callers.
pub struct PresentationEnvelopeMaterializeRegistry {
    slots: [PresentationEnvelopeMaterializeSlot; PRESENTATION_ENVELOPE_MATERIALIZE_CAPACITY],
    live: usize,
    occupied: u64,
    maintenance_cursor: usize,
}

impl PresentationEnvelopeMaterializeRegistry {
    pub fn new() -> Self {
        Self {
            slots: std::array::from_fn(|_| PresentationEnvelopeMaterializeSlot { operation: semio_framework_job::OperationId(0), generation: semio_framework_job::Generation(0), occupied: false, handle: std::mem::MaybeUninit::uninit() }),
            live: 0,
            occupied: 0,
            maintenance_cursor: 0,
        }
    }

    fn index(operation: semio_framework_job::OperationId) -> usize {
        operation.0 as usize % PRESENTATION_ENVELOPE_MATERIALIZE_CAPACITY
    }

    pub fn can_insert(&self, operation: semio_framework_job::OperationId) -> bool {
        !self.slots[Self::index(operation)].occupied
    }

    /// 📥️ Preflights the fixed slot before constructing any nested decode/job owner.
    pub fn try_submit(&mut self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, pages: store::OwnedSchemaDecodePages) -> Result<(), (PresentationEnvelopeMaterializeRegistryFault, store::OwnedSchemaDecodePages)> {
        let index = Self::index(operation);
        if self.slots[index].occupied {
            let fault = if self.slots[index].operation == operation { PresentationEnvelopeMaterializeRegistryFault::Collision } else { PresentationEnvelopeMaterializeRegistryFault::Capacity };
            return Err((fault, pages));
        }
        let handle = match submit_materialize_presentation_projection(operation, generation, pages) {
            Ok(handle) => handle,
            Err(pages) => return Err((PresentationEnvelopeMaterializeRegistryFault::Capacity, pages)),
        };
        let slot = &mut self.slots[index];
        slot.operation = operation;
        slot.generation = generation;
        slot.handle.write(handle);
        slot.occupied = true;
        self.live += 1;
        self.occupied |= 1u64 << index;
        Ok(())
    }

    fn get_mut(&mut self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<&mut PresentationEnvelopeMaterializeHandle, PresentationEnvelopeMaterializeRegistryFault> {
        let slot = &mut self.slots[Self::index(operation)];
        if !slot.occupied || slot.operation != operation || slot.generation != generation {
            return Err(PresentationEnvelopeMaterializeRegistryFault::Stale);
        }
        Ok(unsafe { slot.handle.assume_init_mut() })
    }

    pub fn maintenance_step(
        &mut self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        live_generation: semio_framework_job::Generation,
        pool: &semio_framework_job::WorkerPool,
    ) -> Result<PresentationEnvelopeMaterializeHandleStep, PresentationEnvelopeMaterializeRegistryFault> {
        Ok(self.get_mut(operation, generation)?.maintenance_step(pool, live_generation))
    }

    /// 🪜️ App maintenance advances one exact live caller in stable slot order.
    pub fn maintenance_next_step(&mut self, pool: &semio_framework_job::WorkerPool) -> Option<PresentationEnvelopeMaterializeHandleStep> {
        if self.occupied == 0 {
            self.maintenance_cursor = 0;
            return None;
        }
        let start = self.maintenance_cursor % PRESENTATION_ENVELOPE_MATERIALIZE_CAPACITY;
        let offset = self.occupied.rotate_right(start as u32).trailing_zeros() as usize;
        let index = (start + offset) % PRESENTATION_ENVELOPE_MATERIALIZE_CAPACITY;
        self.maintenance_cursor = (index + 1) % PRESENTATION_ENVELOPE_MATERIALIZE_CAPACITY;
        let slot = &mut self.slots[index];
        let generation = slot.generation;
        Some(unsafe { slot.handle.assume_init_mut() }.maintenance_step(pool, generation))
    }

    pub fn cancel(&mut self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<(), PresentationEnvelopeMaterializeRegistryFault> {
        self.get_mut(operation, generation)?.cancel_now();
        Ok(())
    }

    pub fn fault(&mut self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<Option<&[u8]>, PresentationEnvelopeMaterializeRegistryFault> {
        Ok(self.get_mut(operation, generation)?.fault())
    }

    pub fn try_publish_to(&mut self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, target: &mut dyn PresentationProjectionAdoptionTarget) -> Result<bool, PresentationEnvelopeMaterializeRegistryFault> {
        let published = self.get_mut(operation, generation)?.try_publish_to(target).map_err(|_| PresentationEnvelopeMaterializeRegistryFault::Contended)?;
        if published {
            self.reclaim_terminal(operation, generation)?;
        }
        Ok(published)
    }

    fn reclaim_terminal(&mut self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<(), PresentationEnvelopeMaterializeRegistryFault> {
        let index = Self::index(operation);
        let slot = &mut self.slots[index];
        if !slot.occupied || slot.operation != operation || slot.generation != generation {
            return Err(PresentationEnvelopeMaterializeRegistryFault::Stale);
        }
        let handle = unsafe { slot.handle.assume_init_ref() };
        if !handle.terminal_is_empty() {
            return Ok(());
        }
        let handle = unsafe { slot.handle.assume_init_read() };
        slot.occupied = false;
        self.live -= 1;
        self.occupied &= !(1u64 << index);
        drop(handle);
        Ok(())
    }

    pub fn close_step(
        &mut self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        pool: &semio_framework_job::WorkerPool,
        maximum_items: usize,
        maximum_bytes: usize,
    ) -> Result<store::SnapshotRetirementStep, String> {
        let step = self.get_mut(operation, generation).map_err(|_| "Presentation envelope materialize close received a stale operation/generation")?.close_step(pool, maximum_items, maximum_bytes)?;
        if step == store::SnapshotRetirementStep::Complete {
            self.reclaim_terminal(operation, generation).map_err(|_| "Presentation envelope terminal handle changed before exact registry removal")?;
        }
        Ok(step)
    }

    /// 🧹️ App close advances one retained caller and removes only its witnessed terminal shell.
    pub fn close_next_step(&mut self, pool: &semio_framework_job::WorkerPool, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if self.occupied == 0 {
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let index = self.occupied.trailing_zeros() as usize;
        let operation = self.slots[index].operation;
        let generation = self.slots[index].generation;
        let step = self.close_step(operation, generation, pool, maximum_items, maximum_bytes)?;
        if self.occupied == 0 {
            Ok(store::SnapshotRetirementStep::Complete)
        } else if step == store::SnapshotRetirementStep::Complete {
            Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
        } else {
            Ok(step)
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.live == 0 && self.occupied == 0 && self.slots.iter().all(|slot| !slot.occupied)
    }
}

impl Default for PresentationEnvelopeMaterializeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for PresentationEnvelopeMaterializeRegistry {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Presentation envelope materialize registry reached Drop before every retained caller was closed and reclaimed");
    }
}
//#endregion 🧬️OwnedEnvelopeCatalog

/// 📦️ Encodes a `PresentationMutation` to its binary state-patch form.
pub fn encode_op(operation: &PresentationMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `PresentationMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<PresentationMutation, protocol::ProtocolError> {
    PresentationMutation::decode_op(bytes)
}

//#region 🔖️Store
pub type PresentationEnvelope = ArtifactEnvelope<PresentationSnapshot, PresentationMutation>;
pub type PresentationStore = ArtifactStore<PresentationSnapshot, PresentationMutation>;
//#endregion 🔖️Store

//#region 🔖️VcsEnvelope
/// 📦️ Creates an empty typed VCS envelope for a presentation deck document.
pub fn create_presentation_envelope(id: &str) -> PresentationEnvelope {
    create_document_envelope(PRESENTATION_DOCUMENT_SCHEMA, id, empty_presentation_snapshot(), None)
}

//#endregion 🔖️VcsEnvelope

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::presentation::schema::mutations::PresentationMutation;
    use crate::artifacts::presentation::schema::mutations::{create_tile, replace_tiles};
    use store::{os_store::test_support, ArtifactCommand};

    struct PresentationProjectionFixtureTarget {
        value: Option<PresentationSnapshot>,
    }

    impl PresentationProjectionAdoptionTarget for PresentationProjectionFixtureTarget {
        fn try_adopt(&mut self, value: PresentationSnapshot) -> Result<(), PresentationSnapshot> {
            if self.value.is_some() {
                return Err(value);
            }
            self.value = Some(value);
            Ok(())
        }
    }

    struct PresentationProjectionBackpressureTarget {
        reject_once: bool,
        value: Option<PresentationSnapshot>,
    }

    impl PresentationProjectionAdoptionTarget for PresentationProjectionBackpressureTarget {
        fn try_adopt(&mut self, value: PresentationSnapshot) -> Result<(), PresentationSnapshot> {
            if std::mem::take(&mut self.reject_once) || self.value.is_some() {
                return Err(value);
            }
            self.value = Some(value);
            Ok(())
        }
    }

    fn presentation_envelope_test_pages(snapshot_hex: &str) -> store::OwnedSchemaDecodePages {
        let json = format!("{{\"schema\":\"{PRESENTATION_DOCUMENT_SCHEMA}\",\"id\":\"deck-1\",\"vcs\":{{\"initialSnapshot\":\"{snapshot_hex}\",\"edits\":[],\"changes\":[],\"checkpoints\":[],\"alternatives\":[]}},\"editMessages\":[],\"conflicts\":[]}}");
        presentation_envelope_json_test_pages(&json)
    }

    fn presentation_envelope_json_test_pages(json: &str) -> store::OwnedSchemaDecodePages {
        let chunks = json.as_bytes().chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).collect::<Vec<_>>();
        let mut pages = store::OwnedSchemaDecodePages::try_with_credits(store::OwnedSchemaDecodeCredits { maximum_pages: chunks.len(), maximum_bytes: json.len() }).expect("exact test page credits");
        for chunk in chunks {
            pages.admit_page(store::OwnedSchemaDecodePage::try_from_slice(chunk).expect("bounded test page")).unwrap_or_else(|_| panic!("pre-admitted page"));
        }
        pages.seal().expect("sealed test pages");
        pages
    }

    fn close_presentation_snapshot(value: PresentationSnapshot) {
        let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&PresentationFreshSnapshotRetirementFactory, value);
        assert!(matches!(retirement.close_step(1, PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES), Ok(store::SnapshotRetirementStep::Pending { .. })));
        assert_eq!(retirement.close_step(1, PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES).expect("fixture retirement"), store::SnapshotRetirementStep::Complete);
        assert!(retirement.terminal_is_empty());
        drop(retirement);
    }

    fn close_presentation_pages(mut pages: store::OwnedSchemaDecodePages) {
        while let Some(page) = pages.close_take_page() {
            drop(page);
        }
        assert!(pages.terminal_is_empty());
        drop(pages);
    }

    fn close_presentation_registry(registry: &mut PresentationEnvelopeMaterializeRegistry, pool: &semio_framework_job::WorkerPool) {
        for _ in 0..100_000 {
            if registry.close_next_step(pool, 1, PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES).expect("bounded registry close") == store::SnapshotRetirementStep::Complete {
                assert!(registry.terminal_is_empty());
                return;
            }
        }
        panic!("Presentation registry did not reach terminal empty within the fixed fixture ceiling");
    }

    #[semio_framework_async_macros::async_test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let operation = PresentationMutation::ReplaceTiles(replace_tiles::mutation::ReplaceTiles { new_tiles: Vec::new() });
        test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[semio_framework_async_macros::async_test]
    async fn envelope_helpers_round_trip() {
        let snapshot = empty_presentation_snapshot();
        let pack = <PresentationSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let hex = pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        let operation = semio_framework_job::OperationId(7001);
        let generation = semio_framework_job::Generation(3);
        let mut registry = PresentationEnvelopeMaterializeRegistry::new();
        registry.try_submit(operation, generation, presentation_envelope_test_pages(&hex)).unwrap_or_else(|_| panic!("sealed fixed-page caller"));
        let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::InteractiveNative, 1));
        let mut target = PresentationProjectionFixtureTarget { value: None };
        for _ in 0..10_000 {
            match registry.maintenance_step(operation, generation, generation, &pool).expect("exact live caller") {
                PresentationEnvelopeMaterializeHandleStep::Pending | PresentationEnvelopeMaterializeHandleStep::Progress => {}
                PresentationEnvelopeMaterializeHandleStep::Ready => {
                    assert!(registry.try_publish_to(operation, generation, &mut target).expect("completion lock is uncontended"));
                    break;
                }
                outcome => panic!("valid Presentation envelope caller produced {outcome:?}"),
            }
        }
        assert!(registry.terminal_is_empty());
        drop(registry);
        let deck = target.value.take().expect("typed projection published exactly once");
        assert_eq!(deck.schema, PRESENTATION_DOCUMENT_SCHEMA);
        assert!(crate::artifacts::presentation::presentation_working_scene(&deck).1.is_empty());
        close_presentation_snapshot(deck);
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_presentation_envelope_materializes_populated_history_in_order() {
        let snapshot = empty_presentation_snapshot();
        let pack = <PresentationSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let hex = pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        let mutation_value: serde_json::Value = dsl::ToValue::to_value(&PresentationMutation::ReplaceTiles(replace_tiles::mutation::ReplaceTiles { new_tiles: Vec::new() })).into();
        let mutation = mutation_value.to_string();
        let json = format!(
            "{{\"schema\":\"{PRESENTATION_DOCUMENT_SCHEMA}\",\"id\":\"deck-history\",\"vcs\":{{\"initialSnapshot\":\"{hex}\",\"edits\":[{{\"id\":\"edit-1\",\"forwards\":[{mutation}],\"inverse\":[],\"sequenceNumber\":1,\"startedAt\":\"1\"}}],\"changes\":[],\"checkpoints\":[],\"alternatives\":[]}},\"editMessages\":[],\"conflicts\":[]}}"
        );
        let operation = semio_framework_job::OperationId(7007);
        let generation = semio_framework_job::Generation(9);
        let mut registry = PresentationEnvelopeMaterializeRegistry::new();
        registry.try_submit(operation, generation, presentation_envelope_json_test_pages(&json)).unwrap_or_else(|_| panic!("populated retained caller was pre-admitted"));
        let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::InteractiveNative, 1));
        let mut target = PresentationProjectionFixtureTarget { value: None };
        for _ in 0..20_000 {
            match registry.maintenance_step(operation, generation, generation, &pool).expect("exact populated caller") {
                PresentationEnvelopeMaterializeHandleStep::Pending | PresentationEnvelopeMaterializeHandleStep::Progress => {}
                PresentationEnvelopeMaterializeHandleStep::Ready => {
                    assert!(registry.try_publish_to(operation, generation, &mut target).expect("populated output publication"));
                    break;
                }
                outcome => panic!("populated Presentation history produced {outcome:?}"),
            }
        }
        assert!(registry.terminal_is_empty());
        drop(registry);
        let deck = target.value.take().expect("populated history published exactly once");
        assert!(crate::artifacts::presentation::presentation_working_scene(&deck).1.is_empty());
        close_presentation_snapshot(deck);
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_presentation_envelope_caller_faults_and_zero_grant_closes_malformed_pack() {
        let operation = semio_framework_job::OperationId(7002);
        let generation = semio_framework_job::Generation(4);
        let mut registry = PresentationEnvelopeMaterializeRegistry::new();
        registry.try_submit(operation, generation, presentation_envelope_test_pages("00")).unwrap_or_else(|_| panic!("sealed malformed pages remain retained"));
        let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::InteractiveNative, 1));
        for _ in 0..10_000 {
            if registry.maintenance_step(operation, generation, generation, &pool).expect("exact live caller") == PresentationEnvelopeMaterializeHandleStep::Fault {
                break;
            }
        }
        assert!(registry.fault(operation, generation).expect("exact fault owner").is_some());
        assert_eq!(registry.close_step(operation, generation, &pool, 0, 0).expect("zero grant preserves the exact fault owner"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        close_presentation_registry(&mut registry, &pool);
        assert!(registry.terminal_is_empty());
        drop(registry);
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_presentation_envelope_caller_cancels_and_zero_grant_closes_without_output() {
        let pack = <PresentationSnapshot as store::ArtifactPack>::encode_pack(&empty_presentation_snapshot());
        let hex = pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        let operation = semio_framework_job::OperationId(7003);
        let generation = semio_framework_job::Generation(5);
        let mut registry = PresentationEnvelopeMaterializeRegistry::new();
        registry.try_submit(operation, generation, presentation_envelope_test_pages(&hex)).unwrap_or_else(|_| panic!("sealed fixed-page caller"));
        let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::InteractiveNative, 1));
        registry.cancel(operation, generation).expect("exact live caller");
        for _ in 0..10_000 {
            if registry.maintenance_step(operation, generation, generation, &pool).expect("exact live caller") == PresentationEnvelopeMaterializeHandleStep::Cancelled {
                break;
            }
        }
        assert_eq!(registry.close_step(operation, generation, &pool, 0, 0).expect("zero grant preserves the exact cancelled job"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        close_presentation_registry(&mut registry, &pool);
        assert!(registry.terminal_is_empty());
        drop(registry);
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_presentation_envelope_registry_preserves_collision_capacity_and_exact_rejected_pages() {
        let pack = <PresentationSnapshot as store::ArtifactPack>::encode_pack(&empty_presentation_snapshot());
        let hex = pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        let generation = semio_framework_job::Generation(9);
        let mut registry = PresentationEnvelopeMaterializeRegistry::new();
        for index in 0..PRESENTATION_ENVELOPE_MATERIALIZE_CAPACITY {
            registry.try_submit(semio_framework_job::OperationId(8_000 + index as u64), generation, presentation_envelope_test_pages(&hex)).unwrap_or_else(|_| panic!("every fixed registry slot admits exactly once"));
        }
        let collision = semio_framework_job::OperationId(8_000 + PRESENTATION_ENVELOPE_MATERIALIZE_CAPACITY as u64);
        let pages = presentation_envelope_test_pages(&hex);
        let rejected_page_count = pages.page_count();
        let rejected_byte_count = pages.byte_count();
        let (fault, pages) = registry.try_submit(collision, generation, pages).expect_err("capacity +1 returns the exact caller pages");
        assert_eq!(fault, PresentationEnvelopeMaterializeRegistryFault::Capacity);
        assert_eq!(pages.page_count(), rejected_page_count);
        assert_eq!(pages.byte_count(), rejected_byte_count);
        close_presentation_pages(pages);
        let duplicate = semio_framework_job::OperationId(8_000);
        let (fault, pages) = registry.try_submit(duplicate, generation, presentation_envelope_test_pages(&hex)).expect_err("duplicate operation never replaces its live owner");
        assert_eq!(fault, PresentationEnvelopeMaterializeRegistryFault::Collision);
        close_presentation_pages(pages);
        let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::InteractiveNative, 1));
        close_presentation_registry(&mut registry, &pool);
        drop(registry);
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_presentation_envelope_wrong_owner_abort_and_interrupted_close_preserve_live_slot() {
        let pack = <PresentationSnapshot as store::ArtifactPack>::encode_pack(&empty_presentation_snapshot());
        let hex = pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        let operation = semio_framework_job::OperationId(8_050);
        let generation = semio_framework_job::Generation(13);
        let wrong_generation = semio_framework_job::Generation(14);
        let wrong_operation = semio_framework_job::OperationId(operation.0 + PRESENTATION_ENVELOPE_MATERIALIZE_CAPACITY as u64);
        let mut registry = PresentationEnvelopeMaterializeRegistry::new();
        registry.try_submit(operation, generation, presentation_envelope_test_pages(&hex)).unwrap_or_else(|_| panic!("one exact retained caller is admitted"));
        let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::InteractiveNative, 1));

        assert_eq!(registry.cancel(operation, wrong_generation), Err(PresentationEnvelopeMaterializeRegistryFault::Stale));
        assert_eq!(registry.cancel(wrong_operation, generation), Err(PresentationEnvelopeMaterializeRegistryFault::Stale));
        assert!(registry.close_step(operation, wrong_generation, &pool, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).is_err());
        assert!(registry.fault(operation, generation).is_ok(), "wrong-owner probes preserve the exact live slot");

        registry.cancel(operation, generation).expect("the exact owner may still abort");
        assert_eq!(
            registry.close_step(operation, generation, &pool, 0, 0).expect("zero-grant interrupted close"),
            store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
        );
        close_presentation_registry(&mut registry, &pool);
        assert!(registry.terminal_is_empty());
        drop(registry);
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_presentation_envelope_publication_retries_backpressure_exactly_once() {
        let pack = <PresentationSnapshot as store::ArtifactPack>::encode_pack(&empty_presentation_snapshot());
        let hex = pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        let operation = semio_framework_job::OperationId(8_100);
        let generation = semio_framework_job::Generation(10);
        let mut registry = PresentationEnvelopeMaterializeRegistry::new();
        registry.try_submit(operation, generation, presentation_envelope_test_pages(&hex)).unwrap_or_else(|_| panic!("sealed fixed-page caller"));
        let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::InteractiveNative, 1));
        for _ in 0..10_000 {
            if registry.maintenance_step(operation, generation, generation, &pool).expect("exact live caller") == PresentationEnvelopeMaterializeHandleStep::Ready {
                break;
            }
        }
        let mut target = PresentationProjectionBackpressureTarget { reject_once: true, value: None };
        assert!(!registry.try_publish_to(operation, generation, &mut target).expect("first publication preserves backpressure owner"));
        assert_eq!(registry.maintenance_step(operation, generation, generation, &pool).expect("backpressured caller remains exact"), PresentationEnvelopeMaterializeHandleStep::Ready);
        assert!(registry.try_publish_to(operation, generation, &mut target).expect("second publication atomically succeeds"));
        assert!(registry.terminal_is_empty());
        close_presentation_snapshot(target.value.take().expect("one exact output publication"));
        drop(registry);
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_presentation_envelope_stale_generation_cancels_and_unpublished_output_closes() {
        let pack = <PresentationSnapshot as store::ArtifactPack>::encode_pack(&empty_presentation_snapshot());
        let hex = pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        let operation = semio_framework_job::OperationId(8_200);
        let generation = semio_framework_job::Generation(11);
        let mut registry = PresentationEnvelopeMaterializeRegistry::new();
        registry.try_submit(operation, generation, presentation_envelope_test_pages(&hex)).unwrap_or_else(|_| panic!("sealed fixed-page caller"));
        let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::InteractiveNative, 1));
        for _ in 0..10_000 {
            let step = registry.maintenance_step(operation, generation, semio_framework_job::Generation(12), &pool).expect("exact stale caller remains retained");
            if matches!(step, PresentationEnvelopeMaterializeHandleStep::Cancelled | PresentationEnvelopeMaterializeHandleStep::Fault) {
                break;
            }
        }
        close_presentation_registry(&mut registry, &pool);
        assert!(registry.terminal_is_empty());
        drop(registry);

        let operation = semio_framework_job::OperationId(8_201);
        let mut registry = PresentationEnvelopeMaterializeRegistry::new();
        registry.try_submit(operation, generation, presentation_envelope_test_pages(&hex)).unwrap_or_else(|_| panic!("second sealed fixed-page caller"));
        for _ in 0..10_000 {
            if registry.maintenance_step(operation, generation, generation, &pool).expect("exact live caller") == PresentationEnvelopeMaterializeHandleStep::Ready {
                break;
            }
        }
        close_presentation_registry(&mut registry, &pool);
        assert!(registry.terminal_is_empty());
        drop(registry);
    }

    #[semio_framework_async_macros::async_test]
    async fn presentation_deck_materializes() {
        let mut store = PresentationStore::new(create_document_envelope(PRESENTATION_DOCUMENT_SCHEMA, "animate-presentation", empty_presentation_snapshot(), None)).await.expect("valid artifact store fixture");
        store
            .dispatch(ArtifactCommand::Apply {
                mutations: vec![PresentationMutation::CreateTile(create_tile::mutation::CreateTile {
                    index: 0,
                    tile: crate::artifacts::presentation::FigureTileDraft { id: "t1".into(), name: "A".into(), crop: crate::artifacts::presentation::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } },
                })],
                description: None,
            })
            .await
            .expect("apply");
        assert_eq!(crate::artifacts::presentation::presentation_working_scene(&store.snapshot().await.expect("projection")).1.len(), 1);
    }

    //#region 🔖️DocumentTextTests
    #[semio_framework_async_macros::async_test]
    async fn document_text_round_trip_with_operation_applied() {
        let mut store = PresentationStore::new(create_document_envelope(PRESENTATION_DOCUMENT_SCHEMA, "animate-presentation", crate::artifacts::presentation::default_presentation_snapshot(), None)).await.expect("valid artifact store fixture");
        store
            .dispatch(ArtifactCommand::Apply {
                mutations: vec![PresentationMutation::CreateTile(create_tile::mutation::CreateTile {
                    index: 0,
                    tile: crate::artifacts::presentation::FigureTileDraft { id: "t1".into(), name: "A".into(), crop: crate::artifacts::presentation::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } },
                })],
                description: None,
            })
            .await
            .expect("apply");
        test_support::assert_document_text_round_trip(&store).await;
        test_support::assert_document_pack_round_trip(&store).await;
    }
    //#endregion 🔖️DocumentTextTests
}
//#endregion 🧪️Tests
