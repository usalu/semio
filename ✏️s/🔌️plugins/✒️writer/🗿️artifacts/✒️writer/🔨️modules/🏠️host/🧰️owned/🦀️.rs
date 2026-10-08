//! 🏠️ Artifact document-store and publication authorities.

use crate::op::WriterMutation;
use crate::schema;
use crate::WriterSnapshot;
use protocol::{Mutation, OpBinary};
use store::ArtifactEnvelopeMutationFieldTarget;
pub fn writer_document_store_owners() -> store::DocumentStoreOwners<WriterSnapshot, WriterMutation> {
    store::DocumentStoreOwners::new(
        std::sync::Arc::new(WriterSnapshotRetirementFactory),
        std::sync::Arc::new(WriterSnapshotRetirementFactory),
        std::sync::Arc::new(WriterMutationRetirementFactory),
        Box::new(store::ArtifactStoreCursorDisposer::<WriterSnapshot, WriterMutation>::new()),
    )
}

pub type WriterDocumentStore = store::ArtifactStore<WriterSnapshot, WriterMutation>;

struct WriterStoreInitializationAuthority {
    actor: protocol::ActorId,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    envelope: std::mem::ManuallyDrop<Option<store::ArtifactEnvelope<WriterSnapshot, WriterMutation>>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<WriterSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<WriterSnapshot, WriterMutation>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    initial: std::mem::ManuallyDrop<Option<WriterSnapshot>>,
    edit_index: store::ArtifactStoreInitializationEditIndex,
    phase: WriterStoreInitializationPhase,
    resume_phase: Option<WriterStoreInitializationPhase>,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    terminal_handoff: bool,
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<WriterSnapshot, WriterMutation> for WriterStoreInitializationAuthority {
    fn next_close_byte_demand(&self) -> usize {
        self.active.as_ref().or(self.envelope_retirement.as_ref()).map_or(WRITER_ENVELOPE_FIELD_BYTES, |owner| owner.next_close_byte_demand())
    }

    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"writer-store.initializer-stale-authority");
        }
        if self.cancel_requested && !matches!(self.phase, WriterStoreInitializationPhase::RetireCancelled | WriterStoreInitializationPhase::Cancelled) {
            self.phase = WriterStoreInitializationPhase::RetireCancelled;
        }
        if let Err(error) = self.pump_active() {
            self.fault = Some(error.into_message().into_bytes());
            self.phase = WriterStoreInitializationPhase::RetireFault;
        } else if self.active.is_some() {
            return semio_framework_job::StepOutcome::Yield;
        }
        if !matches!(self.phase, WriterStoreInitializationPhase::RetireCancelled | WriterStoreInitializationPhase::RetireFault | WriterStoreInitializationPhase::Cancelled | WriterStoreInitializationPhase::Fault | WriterStoreInitializationPhase::Complete) {
            if let Some(runtime) = self.runtime.as_mut() {
                match runtime.settle_current_retirement_step(1, WRITER_ENVELOPE_FIELD_BYTES) {
                    Ok(store::SnapshotRetirementStep::Complete) => {}
                    Ok(_) => { cx.consume_fuel(1); return semio_framework_job::StepOutcome::Yield; }
                    Err(error) => { self.fault = Some(error.into_message().into_bytes()); self.phase = WriterStoreInitializationPhase::RetireFault; }
                }
            }
        }
        match self.phase {
            WriterStoreInitializationPhase::BindGenesis => {
                let envelope = self.envelope.as_ref().expect("retained initializer genesis");
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, envelope.vcs.genesis.share_snapshot(), envelope.vcs.genesis.digest(), self.actor.clone()));
                self.phase = WriterStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::ValidateEnvelope => {
                let Some(envelope) = self.envelope.as_ref() else {
                    self.fail(b"writer-store.initializer-envelope-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if envelope.schema != crate::WRITER_DOCUMENT_SCHEMA || envelope.id.is_empty() || envelope.id.len() > WRITER_ENVELOPE_FIELD_BYTES {
                    self.fail(b"writer-store.initializer-envelope-invalid");
                } else {
                    self.phase = WriterStoreInitializationPhase::ValidateEdit { index: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::ValidateEdit { index } => {
                let envelope = self.envelope.as_ref().expect("validated Writer envelope remains retained");
                match self.edit_index.admit(&envelope.vcs.edits, index, WRITER_ENVELOPE_FIELD_BYTES) {
                    store::ArtifactStoreInitializationEditAdmission::Complete => self.phase = WriterStoreInitializationPhase::BindGenesis,
                    store::ArtifactStoreInitializationEditAdmission::Admitted => self.phase = WriterStoreInitializationPhase::ValidateEdit { index: index + 1 },
                    store::ArtifactStoreInitializationEditAdmission::Oversized | store::ArtifactStoreInitializationEditAdmission::Duplicate => self.fail(b"writer-store.initializer-duplicate-or-hostile-edit"),
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::CloneInitial { field, offset } => {
                if field == 10 {
                    let initial = self.initial.take().expect("Writer initial snapshot was built one field at a time");
                    match self.runtime.as_mut().expect("retained initializer runtime").adopt_current_owned(initial, std::sync::Arc::new(WriterSnapshotRetirementFactory)) {
                        Ok(()) => self.phase = self.resume_phase.take().expect("retained mutation resume phase"),
                        Err(initial) => {
                            *self.active = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&WriterSnapshotRetirementFactory, initial));
                            self.fail(b"initializer-owned-workspace-adoption");
                        }
                    }
                    return semio_framework_job::StepOutcome::Yield;
                }
                let envelope = self.envelope.as_ref().expect("Writer envelope remains retained during initial clone");
                let value = Self::initial_field(envelope.vcs.genesis.snapshot(), field);
                // ✂️ One PAGE of one field per step. `WriterSnapshot::text` is the document's authored body
                // and has no length ceiling, so a fixed per-field ceiling would refuse every writer
                // document over one page (`writer-store.initializer-initial-field-too-large`) instead of
                // copying it — the initial target starts empty, so the pages simply accumulate.
                let page = writer_page_boundary(&value[offset..], WRITER_ENVELOPE_FIELD_BYTES);
                if page == 0 {
                    if offset < value.len() {
                        self.fail(b"writer-store.initializer-initial-field-unpageable");
                        return semio_framework_job::StepOutcome::Yield;
                    }
                    self.phase = WriterStoreInitializationPhase::CloneInitial { field: field + 1, offset: 0 };
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                let chunk = &value[offset..offset + page];
                Self::initial_field_mut(self.initial.as_mut().expect("Writer initial target remains retained"), field).push_str(chunk);
                self.phase = WriterStoreInitializationPhase::CloneInitial { field, offset: offset + page };
                cx.consume_fuel(page as u64);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("Writer envelope remains retained while causal history is seeded");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = WriterStoreInitializationPhase::FoldSupersessions { transition: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let runtime = self.runtime.as_mut().expect("Writer runtime remains retained while history is seeded");
                match lane {
                    0 => {
                        if let Err(error) = runtime.seed_mutation(protocol::MutationId(entry.id.clone())) {
                            self.fault = Some(error.into_bytes());
                            self.phase = WriterStoreInitializationPhase::RetireFault;
                        } else {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = WriterStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                    }
                    1 if index < entry.forwards.len() => {
                        let id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| entry.forwards[index].mutation_id()).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                        if let Err(error) = runtime.seed_edit_operation(&entry.id, id) {
                            self.fault = Some(error.into_bytes());
                            self.phase = WriterStoreInitializationPhase::RetireFault;
                        } else {
                            self.phase = WriterStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                        }
                    }
                    1 => self.phase = WriterStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp.clone());
                        self.phase = WriterStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = WriterStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::FoldSupersessions { transition } => {
                let envelope = self.envelope.as_ref().expect("Writer envelope remains retained while its supersessions fold");
                match self.runtime.as_mut().expect("Writer runtime remains retained while its supersessions fold").fold_supersession_step(envelope, transition) {
                    Ok(true) => self.phase = WriterStoreInitializationPhase::FoldSupersessions { transition: transition + 1 },
                    Ok(false) => self.phase = WriterStoreInitializationPhase::FindApplied { position: 0 },
                    Err(error) => {
                        self.fault = Some(error.into_bytes());
                        self.phase = WriterStoreInitializationPhase::RetireFault;
                    }
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::FindApplied { position } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
                    self.runtime.as_mut().expect("Writer runtime remains retained").set_current_checkpoint_id(checkpoint);
                    self.phase = WriterStoreInitializationPhase::FindRedo { position: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("Writer envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"writer-store.initializer-applied-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    self.phase = WriterStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                } else {
                    self.fail(b"writer-store.initializer-applied-edit-missing");
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let needs_workspace = {
                    let envelope = self.envelope.as_ref().expect("retained initializer envelope");
                    let runtime = self.runtime.as_ref().expect("retained initializer runtime");
                    envelope.vcs.edits.get(edit).and_then(|entry| runtime.effective_forward(entry, mutation, &envelope.schema)).is_some_and(|effective| effective.operation().is_some())
                };
                if needs_workspace && self.runtime.as_mut().expect("retained initializer runtime").current_mut().is_none() {
                    self.resume_phase = Some(self.phase);
                    let document = store::ArtifactChild::new(String::new(), semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } });
                    *self.initial = Some(WriterSnapshot { schema: String::new(), id: String::new(), language_id: String::new(), uri: String::new(), text: String::new(), document });
                    self.phase = WriterStoreInitializationPhase::CloneInitial { field: 0, offset: 0 };
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Writer applied edit remains retained");
                let envelope = self.envelope.as_ref().expect("Writer envelope remains retained while its forwards fold");
                let entry = envelope.vcs.edits.get(edit).expect("Writer applied edit remains retained");
                match self.runtime.as_mut().expect("Writer runtime remains retained while its forwards fold").fold_forward(entry, mutation, &envelope.schema, WRITER_ENVELOPE_FIELD_BYTES) {
                    Ok(store::ArtifactStoreInitializationForward::Folded { displaced, fuel }) => {
                        if let Some(previous) = displaced {
                            *self.active = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&WriterSnapshotRetirementFactory, previous));
                        }
                        self.phase = WriterStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(fuel as u64);
                    }
                    Ok(store::ArtifactStoreInitializationForward::Exhausted) => self.phase = WriterStoreInitializationPhase::CommitApplied { position, edit },
                    Err(_) => self.fail(b"writer-store.initializer-forward-encoding"),
                }
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::CommitApplied { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Writer applied edit remains retained");

                let runtime = self.runtime.as_mut().expect("Writer runtime remains retained");
                if let Err(error) = runtime.push_applied_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fault = Some(error.into_bytes());
                    self.phase = WriterStoreInitializationPhase::RetireFault;
                } else {

                    self.phase = WriterStoreInitializationPhase::FindApplied { position: position + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::FindRedo { position } => {
                let Some(id) = self.redo_id(position) else {
                    self.edit_index.clear();
                    self.phase = WriterStoreInitializationPhase::BuildCandidate;
                    return semio_framework_job::StepOutcome::Yield;
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("Writer envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"writer-store.initializer-redo-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    self.phase = WriterStoreInitializationPhase::CommitRedo { position, edit: scan };
                } else {
                    self.fail(b"writer-store.initializer-redo-edit-missing");
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::CommitRedo { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Writer redo edit remains retained");
                if let Err(error) = self.runtime.as_mut().expect("Writer runtime remains retained").push_redo_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fault = Some(error.into_bytes());
                    self.phase = WriterStoreInitializationPhase::RetireFault;
                } else {
                    self.phase = WriterStoreInitializationPhase::FindRedo { position: position + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::BuildCandidate => {
                let Some(candidate_generation) = self.generation.0.checked_add(1) else {
                    self.fail(b"writer-store.initializer-generation-exhausted");
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.take().expect("Writer envelope remains retained until atomic store construction");
                let runtime = self.runtime.take().expect("Writer runtime remains retained until atomic store construction");
                let candidate = store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, writer_document_store_owners());
                *self.candidate = Some(candidate);
                self.phase = WriterStoreInitializationPhase::Complete;
                semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                })
            }
            WriterStoreInitializationPhase::RetireCancelled | WriterStoreInitializationPhase::RetireFault => match self.pump_terminal_retirement() {
                Ok(false) => semio_framework_job::StepOutcome::Yield,
                Ok(true) => {
                    self.terminal_handoff = true;
                    if self.phase == WriterStoreInitializationPhase::RetireCancelled {
                        self.phase = WriterStoreInitializationPhase::Cancelled;
                        semio_framework_job::StepOutcome::Cancelled
                    } else {
                        self.phase = WriterStoreInitializationPhase::Fault;
                        let source = self.fault.take().unwrap_or_else(|| b"writer-store.initializer-fault".to_vec());
                        let detail = cx.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, &source).unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                        semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail })
                    }
                }
                Err(error) => {
                    self.fault = Some(error.into_message().into_bytes());
                    semio_framework_job::StepOutcome::Yield
                }
            },
            WriterStoreInitializationPhase::Complete => semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
            }),
            WriterStoreInitializationPhase::Cancelled => semio_framework_job::StepOutcome::Cancelled,
            WriterStoreInitializationPhase::Fault => {
                let source = self.fault.as_deref().unwrap_or(b"writer-store.initializer-fault");
                let detail = cx.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, source).unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail })
            }
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, WriterStoreInitializationPhase::Cancelled | WriterStoreInitializationPhase::Fault) {
            self.phase = WriterStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, semio_framework::Fault> {
        self.begin_close();
        if maximum_items == 0 || maximum_bytes < WRITER_ENVELOPE_FIELD_BYTES {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        match self.pump_terminal_retirement() {
            Ok(false) => Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }),
            Ok(true) => {
                self.terminal_handoff = true;
                Ok(semio_framework_plugin::PluginCloseStep::Complete)
            }
            Err(error) => Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("artifact-store.initializer-close"), format!("Writer initializer close failed: {error}"))),
        }
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<WriterSnapshot, WriterMutation>> {
        if self.phase != WriterStoreInitializationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}

const WRITER_ENVELOPE_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

/// ✂️ The largest `index <= limit` that is a UTF-8 char boundary of `text` — a writer body is
/// authored prose, so a fixed byte page will land mid-codepoint sooner or later and `String::truncate`
/// panics there. Pages are therefore at most `limit` bytes, never exactly.
fn writer_page_boundary(text: &str, limit: usize) -> usize {
    let mut index = limit.min(text.len());
    while index > 0 && !text.is_char_boundary(index) {
        index -= 1;
    }
    index
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct WriterSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<WriterSnapshot> for WriterSnapshotRetirementFactory {
    fn retire_owned(&self, value: WriterSnapshot) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(WriterSnapshotRetirement { value: std::mem::ManuallyDrop::new(Some(value)), phase: 0 })
    }
}

impl store::SnapshotRetirementFactory<WriterSnapshot> for WriterSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<WriterSnapshot>) -> usize { std::mem::size_of::<WriterSnapshotRootRetirement>() }

    fn retire(&self, snapshot: std::sync::Arc<WriterSnapshot>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(WriterSnapshotRootRetirement { owner: std::mem::ManuallyDrop::new(Some(snapshot)), retirement: std::mem::ManuallyDrop::new(None) })
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct WriterMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<WriterMutation> for WriterMutationRetirementFactory {
    fn retire_owned(&self, value: WriterMutation) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(WriterMutationRetirement { value: std::mem::ManuallyDrop::new(Some(value)), field_released: false })
    }
}

/// 🔐️ Opens a Writer store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` then refuses every `Apply`
/// (`edit history insertion requires its exact mutation retirement factory`) — a bare
/// `ArtifactStore::new` can be READ but never mutated, undone or closed. The app installs
/// [`writer_document_store_owners`] through `ArtifactEditor::build_document_store_owners`; every
/// standalone store goes through here instead. Mirrors `🕸️dag`'s `new_dag_store`.
pub async fn new_writer_store(envelope: WriterDocumentEnvelope, actor: protocol::ActorId) -> Result<OwnedWriterStore, store::VcsError> {
    let mut store = WriterDocumentStore::new(envelope, actor).await?;
    store.install_document_store_owners_exact(writer_document_store_owners());
    Ok(OwnedWriterStore(store))
}

/// 🔚 A standalone Writer store that retires itself: `ArtifactStore::drop` panics `artifact store
/// reached Drop without its exact terminal-empty shallow-shell witness` unless the store walked its
/// bounded close loop first, so the guard runs that loop on drop (skipped while unwinding, where the
/// original panic is the report worth keeping). Derefs to the bare store for every read and dispatch.
pub struct OwnedWriterStore(WriterDocumentStore);

impl OwnedWriterStore {
    /// 🔚 Walks the exact bounded owner close loop to the terminal-empty witness.
    pub fn close(&mut self) {
        while !self.0.close_owned_terminal_is_empty() {
            self.0.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Writer document store closes through its exact bounded owners");
        }
    }
}

impl std::ops::Deref for OwnedWriterStore {
    type Target = WriterDocumentStore;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OwnedWriterStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for OwnedWriterStore {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.close();
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WriterStoreInitializationPhase {
    BindGenesis,
    ValidateEnvelope,
    ValidateEdit { index: usize },
    CloneInitial { field: u8, offset: usize },
    SeedHistory { edit: usize, lane: u8, index: usize },
    FoldSupersessions { transition: usize },
    FindApplied { position: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize },
    CommitRedo { position: usize, edit: usize },
    BuildCandidate,
    RetireCancelled,
    RetireFault,
    Complete,
    Cancelled,
    Fault,
}

impl WriterStoreInitializationAuthority {
    fn new(envelope: store::ArtifactEnvelope<WriterSnapshot, WriterMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Self {
        Self {
            actor,
            operation,
            generation,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            initial: std::mem::ManuallyDrop::new(None),
            edit_index: store::ArtifactStoreInitializationEditIndex::default(),
            resume_phase: None,
            phase: WriterStoreInitializationPhase::ValidateEnvelope,
            cancel_requested: false,
            fault: None,
            terminal_handoff: false,
        }
    }

    fn initial_field(value: &WriterSnapshot, field: u8) -> &str {
        match field {
            0 => &value.schema,
            1 => &value.id,
            2 => &value.language_id,
            3 => &value.uri,
            4 => &value.text,
            5 => &value.document.child_id,
            6 => &value.document.target.artifact_id,
            7 => &value.document.target.dialect.artifact_kind,
            8 => &value.document.target.dialect.standard,
            9 => &value.document.target.dialect.subset,
            _ => unreachable!("Writer initial field cursor is validated"),
        }
    }

    fn initial_field_mut(value: &mut WriterSnapshot, field: u8) -> &mut String {
        match field {
            0 => &mut value.schema,
            1 => &mut value.id,
            2 => &mut value.language_id,
            3 => &mut value.uri,
            4 => &mut value.text,
            5 => &mut value.document.child_id,
            6 => &mut value.document.target.artifact_id,
            7 => &mut value.document.target.dialect.artifact_kind,
            8 => &mut value.document.target.dialect.standard,
            9 => &mut value.document.target.dialect.subset,
            _ => unreachable!("Writer initial field cursor is validated"),
        }
    }

    fn applied_id(&self, position: usize) -> Option<&str> {
        let envelope = self.envelope.as_ref()?;
        match &envelope.cursor {
            Some(cursor) => cursor.applied_edit_ids.get(position).map(String::as_str),
            None => envelope.vcs.edits.get(position).map(|edit| edit.id.as_str()),
        }
    }

    fn redo_id(&self, position: usize) -> Option<&str> {
        self.envelope.as_ref()?.cursor.as_ref()?.redo_edit_ids.get(position).map(String::as_str)
    }

    fn fail(&mut self, code: &'static [u8]) {
        self.fault = Some(code.to_vec());
        self.phase = WriterStoreInitializationPhase::RetireFault;
    }

    fn pump_active(&mut self) -> Result<bool, semio_framework_value::ValueError> {
        let Some(active) = self.active.as_mut() else { return Ok(false) };
        match active.close_step(1, WRITER_ENVELOPE_FIELD_BYTES)? {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= WRITER_ENVELOPE_FIELD_BYTES => Ok(true),
            store::SnapshotRetirementStep::Pending { .. } => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Writer store initializer retirement exceeded its exact grant")),
            store::SnapshotRetirementStep::Blocked => Ok(true),
            store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                drop(self.active.take());
                Ok(true)
            }
            store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Writer store initializer retirement reported a false terminal")),
        }
    }

    fn pump_terminal_retirement(&mut self) -> Result<bool, semio_framework_value::ValueError> {
        if self.pump_active()? {
            return Ok(false);
        }
        if let Some(runtime) = self.runtime.as_mut() {
            match runtime.close_step(&WriterSnapshotRetirementFactory, 1, WRITER_ENVELOPE_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    drop(self.runtime.take());
                    return Ok(false);
                }
                store::SnapshotRetirementStep::Complete => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Writer initialization runtime reported a false terminal")),
                _ => return Ok(false),
            }
        }
        if let Some(initial) = self.initial.take() {
            *self.active = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&WriterSnapshotRetirementFactory, initial));
            return Ok(false);
        }
        if self.envelope_retirement.is_none() {
            if let Some(envelope) = self.envelope.take() {
                *self.envelope_retirement = Some(writer_envelope_decode_owner_bundle().retire_envelope(envelope));
                return Ok(false);
            }
        }
        if let Some(retirement) = self.envelope_retirement.as_mut() {
            return match retirement.close_step(1, WRITER_ENVELOPE_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.envelope_retirement.take());
                    Ok(true)
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Writer initialization envelope retirement reported a false terminal")),
                _ => Ok(false),
            };
        }
        Ok(true)
    }

    fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.envelope_retirement.is_none()
            && self.initial.is_none()
    }
}

impl Drop for WriterStoreInitializationAuthority {
    fn drop(&mut self) {
        // 🧯️ `std::thread::panicking()` FIRST, exactly like the framework's own
        // `store::ArtifactEnvelope::drop`: a Drop witness exists to catch a leak on a HEALTHY path.
        // Firing it while the thread is ALREADY unwinding turns a reported failure into
        // `panic in a destructor during cleanup` — a non-unwinding abort that kills the whole test
        // binary and hides the first, real failure (that is how one red test took this crate's
        // other 300 with it).
        assert!(std::thread::panicking() || (self.terminal_is_empty_inner()), "Writer store initialization authority reached Drop before exact candidate handoff or retained rejection close");
    }
}

pub fn writer_document_store_initialization_job(
    envelope: store::ArtifactEnvelope<WriterSnapshot, WriterMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    actor: protocol::ActorId,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<WriterSnapshot, WriterMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(WriterStoreInitializationAuthority::new(envelope, operation, generation, actor)))
}

struct WriterSnapshotRetirement {
    value: std::mem::ManuallyDrop<Option<WriterSnapshot>>,
    phase: u8,
}

impl WriterSnapshotRetirement {
    fn take_field(value: &mut WriterSnapshot, phase: u8) -> &mut String {
        match phase {
            0 => &mut value.schema,
            1 => &mut value.id,
            2 => &mut value.language_id,
            3 => &mut value.uri,
            4 => &mut value.text,
            5 => &mut value.document.child_id,
            6 => &mut value.document.target.artifact_id,
            7 => &mut value.document.target.dialect.artifact_kind,
            8 => &mut value.document.target.dialect.standard,
            9 => &mut value.document.target.dialect.subset,
            _ => unreachable!("Writer snapshot retirement phase is validated"),
        }
    }
}

impl store::ErasedSnapshotRetirement for WriterSnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        let Some(value) = self.value.as_mut() else { return Ok(store::SnapshotRetirementStep::Complete) };
        if self.phase < 10 {
            if maximum_items == 0 || maximum_bytes == 0 {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let field = Self::take_field(value, self.phase);
            // ✂️ A field longer than one grant is released a PAGE at a time instead of refusing forever:
            // `WriterSnapshot::text` is an authored body with no length ceiling, and the old
            // "too long ⇒ Pending {0, 0}" arm made every close loop spin until its deadline.
            // Mirrors the framework's own `Bytes` retirement cursor (`♻️retirement/🦀️.rs`).
            if field.len() > maximum_bytes {
                let keep = writer_page_boundary(field, field.len() - maximum_bytes);
                let released_bytes = field.len() - keep;
                field.truncate(keep);
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes });
            }
            let released_bytes = field.len();
            drop(std::mem::take(field));
            self.phase += 1;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
        }
        drop(self.value.take());
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none()
    }
}

impl Drop for WriterSnapshotRetirement {
    fn drop(&mut self) {
        // 🧯️ `std::thread::panicking()` FIRST, exactly like the framework's own
        // `store::ArtifactEnvelope::drop`: a Drop witness exists to catch a leak on a HEALTHY path.
        // Firing it while the thread is ALREADY unwinding turns a reported failure into
        // `panic in a destructor during cleanup` — a non-unwinding abort that kills the whole test
        // binary and hides the first, real failure (that is how one red test took this crate's
        // other 300 with it).
        assert!(std::thread::panicking() || (self.value.is_none()), "Writer snapshot retirement reached Drop before every exact string and child-handle owner was terminal-empty");
    }
}

struct WriterSnapshotRootRetirement {
    owner: std::mem::ManuallyDrop<Option<std::sync::Arc<WriterSnapshot>>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl store::ErasedSnapshotRetirement for WriterSnapshotRootRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(maximum_items.min(1), maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Writer snapshot root retirement reported Complete without terminal-empty authority")),
                step => Ok(step),
            };
        }
        let Some(owner) = self.owner.take() else { return Ok(store::SnapshotRetirementStep::Complete) };
        match std::sync::Arc::try_unwrap(owner) {
            Ok(value) => {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&WriterSnapshotRetirementFactory, value));
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            Err(owner) => {
                *self.owner = Some(owner);
                Ok(store::SnapshotRetirementStep::Blocked)
            }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none() && self.retirement.is_none()
    }
}

impl Drop for WriterSnapshotRootRetirement {
    fn drop(&mut self) {
        // 🧯️ `std::thread::panicking()` FIRST, exactly like the framework's own
        // `store::ArtifactEnvelope::drop`: a Drop witness exists to catch a leak on a HEALTHY path.
        // Firing it while the thread is ALREADY unwinding turns a reported failure into
        // `panic in a destructor during cleanup` — a non-unwinding abort that kills the whole test
        // binary and hides the first, real failure (that is how one red test took this crate's
        // other 300 with it).
        assert!(std::thread::panicking() || (self.owner.is_none() && self.retirement.is_none()), "Writer snapshot root retirement reached Drop before exact Arc handback and bounded field disposal");
    }
}

struct WriterMutationRetirement {
    value: std::mem::ManuallyDrop<Option<WriterMutation>>,
    field_released: bool,
}

impl store::ErasedSnapshotRetirement for WriterMutationRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        let Some(value) = self.value.as_mut() else { return Ok(store::SnapshotRetirementStep::Complete) };
        if !self.field_released {
            if maximum_items == 0 {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let field = match value {
                WriterMutation::RenameWriter(value) => &mut value.new_id,
                WriterMutation::ChangeUri(value) => &mut value.new_uri,
                WriterMutation::ChangeLanguage(value) => &mut value.new_language_id,
                WriterMutation::EditText(value) => &mut value.text,
                WriterMutation::SpliceText(value) => {
                    let released_bytes = value.deleted.len() + value.insert.len() + value.before.len() + value.after.len();
                    if released_bytes > maximum_bytes {
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    for field in [&mut value.deleted, &mut value.insert, &mut value.before, &mut value.after] {
                        drop(std::mem::take(field));
                    }
                    self.field_released = true;
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
                }
            };
            if field.len() > maximum_bytes {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let released_bytes = field.len();
            drop(std::mem::take(field));
            self.field_released = true;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
        }
        drop(self.value.take());
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none()
    }
}

impl Drop for WriterMutationRetirement {
    fn drop(&mut self) {
        // 🧯️ `std::thread::panicking()` FIRST, exactly like the framework's own
        // `store::ArtifactEnvelope::drop`: a Drop witness exists to catch a leak on a HEALTHY path.
        // Firing it while the thread is ALREADY unwinding turns a reported failure into
        // `panic in a destructor during cleanup` — a non-unwinding abort that kills the whole test
        // binary and hides the first, real failure (that is how one red test took this crate's
        // other 300 with it).
        assert!(std::thread::panicking() || (self.value.is_none()), "Writer mutation retirement reached Drop before its exact string owner was terminal-empty");
    }
}

enum WriterSnapshotDecodeState {
    AwaitToken,
    Decode(store::OwnedSchemaHexAuthority<WRITER_ENVELOPE_FIELD_BYTES>),
    Ready,
    Published,
    Closing,
    Complete,
}

impl store::ArtifactEnvelopeSnapshotFieldAuthority<WriterSnapshot> for WriterSnapshotDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path };
        if matches!(self.state, WriterSnapshotDecodeState::AwaitToken) {
            if !terminal {
                return Err(diagnostic("writer-envelope.snapshot-pack-must-be-scalar", token.start));
            }
            self.state = WriterSnapshotDecodeState::Decode(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
        }
        let WriterSnapshotDecodeState::Decode(authority) = &mut self.state else { return Err(diagnostic("writer-envelope.snapshot-pack-token-replayed", token.start)) };
        match authority.step(source, cx) {
            store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
            store::OwnedSchemaHexStep::Complete => {
                let bytes = authority.as_bytes().ok_or_else(|| diagnostic("writer-envelope.snapshot-pack-missing", token.start))?;
                let value = <WriterSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|_| diagnostic("writer-envelope.snapshot-pack-malformed", token.start))?;
                assert!(authority.release(), "completed Writer snapshot pack releases its inline bytes exactly once");
                *self.value = Some(value);
                self.state = WriterSnapshotDecodeState::Ready;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            store::OwnedSchemaHexStep::Cancelled => Err(diagnostic("writer-envelope.snapshot-pack-cancelled", token.start)),
            store::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
        }
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::ArtifactEnvelopeSnapshotFieldTarget<WriterSnapshot>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, WriterSnapshotDecodeState::Ready) {
            return Err(self.diagnostic("writer-envelope.snapshot-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("writer-envelope.snapshot-owner-missing", 0))?;
        target.publish_snapshot_reserved(reservation, value);
        self.state = WriterSnapshotDecodeState::Published;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn next_close_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(usize::from(self.retirement.is_some()) * store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)
    }

    fn maximum_close_byte_demand(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let WriterSnapshotDecodeState::Decode(authority) = &mut self.state {
            authority.cancel();
            self.state = WriterSnapshotDecodeState::Closing;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&WriterSnapshotRetirementFactory, value));
                self.state = WriterSnapshotDecodeState::Closing;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.state = WriterSnapshotDecodeState::Complete;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let path = self.path;
        let retirement = self.retirement.as_mut().expect("Writer snapshot retirement remains retained");
        match retirement.close_step(maximum_items, maximum_bytes).map_err(|_| store::OwnedSchemaDecodeDiagnostic { code: "writer-envelope.snapshot-retirement-fault", offset: 0, line: 0, column: 0, path })? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                self.state = WriterSnapshotDecodeState::Complete;
                Ok(store::SnapshotRetirementStep::Complete)
            }
            store::SnapshotRetirementStep::Complete => Err(self.diagnostic("writer-envelope.snapshot-retirement-false-terminal", 0)),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.state, WriterSnapshotDecodeState::Published | WriterSnapshotDecodeState::Complete) && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for WriterSnapshotDecodeAuthority {
    fn drop(&mut self) {
        // 🧯️ `std::thread::panicking()` FIRST, exactly like the framework's own
        // `store::ArtifactEnvelope::drop`: a Drop witness exists to catch a leak on a HEALTHY path.
        // Firing it while the thread is ALREADY unwinding turns a reported failure into
        // `panic in a destructor during cleanup` — a non-unwinding abort that kills the whole test
        // binary and hides the first, real failure (that is how one red test took this crate's
        // other 300 with it).
        assert!(std::thread::panicking() || (store::ArtifactEnvelopeSnapshotFieldAuthority::terminal_is_empty(self)), "Writer snapshot decode reached Drop before publication or bounded retirement");
    }
}

struct WriterMutationString {
    field_id: u16,
    authority: store::OwnedSchemaStringAuthority<WRITER_ENVELOPE_FIELD_BYTES>,
}

impl WriterMutationDecodeAuthority {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self {
            operation,
            generation,
            path,
            cursor: store::OwnedSchemaNestedRecordCursor::try_new(store::OwnedSchemaRecordSpec { fields: WRITER_MUTATION_FIELDS }).expect("Writer mutation schema is a validated static catalog"),
            active: None,
            kind: None,
            payload_field: None,
            payload: std::mem::ManuallyDrop::new(None),
            splice: WriterSplicePayload::default(),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            published: false,
            terminal: false,
        }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
    }

    fn finish_string(&mut self, field_id: u16, authority: &mut store::OwnedSchemaStringAuthority<WRITER_ENVELOPE_FIELD_BYTES>) -> Result<(), store::OwnedSchemaDecodeDiagnostic> {
        if field_id == 1 {
            self.kind = Some(match authority.as_str() {
                Some("renameWriter") => WriterMutationKind::RenameWriter,
                Some("changeUri") => WriterMutationKind::ChangeUri,
                Some("changeLanguage") => WriterMutationKind::ChangeLanguage,
                Some("editText") => WriterMutationKind::EditText,
                Some("spliceText") => WriterMutationKind::SpliceText,
                _ => return Err(self.diagnostic("writer-envelope.unknown-mutation", 0)),
            });
            authority.cancel();
            return Ok(());
        }
        if (7..=10).contains(&field_id) {
            let slot = &mut self.splice.strings[usize::from(field_id - 7)];
            if slot.is_some() {
                return Err(self.diagnostic("writer-envelope.duplicate-mutation-payload", 0));
            }
            *slot = authority.take_string();
            return Ok(());
        }
        if self.payload_field.replace(field_id).is_some() {
            return Err(self.diagnostic("writer-envelope.duplicate-mutation-payload", 0));
        }
        *self.payload = authority.take_string();
        Ok(())
    }

    /// 🔢️ `SpliceText.start`: the one scalar field of the mutation record, a non-negative integer that fits `u32`.
    fn finish_start(&mut self, token: store::OwnedSchemaToken, source: &store::OwnedSchemaRecordCursor) -> Result<(), store::OwnedSchemaDecodeDiagnostic> {
        let mut digits = [0u8; 10];
        let length = usize::try_from(token.end - token.start).map_err(|_| self.diagnostic("writer-envelope.splice-start", token.start))?;
        if token.kind != store::OwnedSchemaTokenKind::Number || length == 0 || length > digits.len() || self.splice.start.is_some() {
            return Err(self.diagnostic("writer-envelope.splice-start", token.start));
        }
        source.copy_token_bytes(token, 0, &mut digits[..length]);
        let text = std::str::from_utf8(&digits[..length]).map_err(|_| self.diagnostic("writer-envelope.splice-start", token.start))?;
        self.splice.start = Some(text.parse::<u32>().map_err(|_| self.diagnostic("writer-envelope.splice-start", token.start))?);
        Ok(())
    }

    fn finish_record(&mut self) -> Result<(), store::OwnedSchemaDecodeDiagnostic> {
        let kind = self.kind.ok_or_else(|| self.diagnostic("writer-envelope.missing-mutation-kind", 0))?;
        let expected = match kind {
            WriterMutationKind::RenameWriter => 2,
            WriterMutationKind::ChangeUri => 3,
            WriterMutationKind::ChangeLanguage => 4,
            WriterMutationKind::EditText => 5,
            WriterMutationKind::SpliceText => {
                let [deleted, insert, before, after] = std::mem::take(&mut self.splice.strings);
                let (Some(start), Some(deleted), Some(insert), Some(before), Some(after), None) = (self.splice.start.take(), deleted, insert, before, after, self.payload_field) else {
                    return Err(self.diagnostic("writer-envelope.mutation-payload-mismatch", 0));
                };
                *self.value = Some(WriterMutation::SpliceText(schema::mutations::SpliceText { start, deleted, insert, before, after }));
                return Ok(());
            }
        };
        if self.payload_field != Some(expected) || self.splice.start.is_some() || self.splice.strings.iter().any(Option::is_some) {
            return Err(self.diagnostic("writer-envelope.mutation-payload-mismatch", 0));
        }
        let payload = self.payload.take().ok_or_else(|| self.diagnostic("writer-envelope.missing-mutation-payload", 0))?;
        *self.value = Some(match kind {
            WriterMutationKind::RenameWriter => WriterMutation::RenameWriter(schema::mutations::RenameWriter { new_id: payload }),
            WriterMutationKind::ChangeUri => WriterMutation::ChangeUri(schema::mutations::ChangeUri { new_uri: payload }),
            WriterMutationKind::ChangeLanguage => WriterMutation::ChangeLanguage(schema::mutations::ChangeLanguage { new_language_id: payload }),
            WriterMutationKind::EditText => WriterMutation::EditText(schema::mutations::EditText { text: payload }),
            WriterMutationKind::SpliceText => return Err(self.diagnostic("writer-envelope.mutation-payload-mismatch", 0)),
        });
        Ok(())
    }
}

impl store::ArtifactEnvelopeMutationFieldAuthority<WriterMutation> for WriterMutationDecodeAuthority {
    fn next_close_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        if self.active.is_some() { return Ok(0); }
        if let Some(retirement) = self.retirement.as_ref() { return Ok(store::artifact_retirement_box_byte_demand(retirement)); }
        if self.value.is_some() { return Ok(0); }
        Ok(self.splice.strings.iter().find_map(|value| value.as_ref().map(String::capacity)).or_else(|| self.payload.as_ref().map(String::capacity)).unwrap_or(0))
    }
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if let Some(mut active) = self.active.take() {
            return match active.authority.step(source, cx) {
                store::OwnedSchemaStringStep::Pending => {
                    self.active = Some(active);
                    Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending)
                }
                store::OwnedSchemaStringStep::Complete => {
                    self.finish_string(active.field_id, &mut active.authority)?;
                    Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                }
                store::OwnedSchemaStringStep::Cancelled => Err(self.diagnostic("writer-envelope.mutation-string-cancelled", token.start)),
                store::OwnedSchemaStringStep::Fault(diagnostic) => Err(diagnostic),
            };
        }
        match self.cursor.accept(token, source) {
            store::OwnedSchemaNestedRecordStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete),
            store::OwnedSchemaNestedRecordStep::FieldToken { field_id, token, terminal: true } => {
                let authority = store::OwnedSchemaStringAuthority::try_new(self.operation, self.generation, token, self.path).map_err(|token| self.diagnostic("writer-envelope.mutation-field-string", token.start))?;
                self.active = Some(WriterMutationString { field_id, authority });
                self.accept_token(token, true, source, cx)
            }
            store::OwnedSchemaNestedRecordStep::FieldToken { field_id: 6, token, .. } => {
                self.finish_start(token, source)?;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete)
            }
            store::OwnedSchemaNestedRecordStep::FieldToken { token, .. } => Err(self.diagnostic("writer-envelope.mutation-field-scalar", token.start)),
            store::OwnedSchemaNestedRecordStep::Complete => {
                self.finish_record()?;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            store::OwnedSchemaNestedRecordStep::Fault(diagnostic) => Err(diagnostic),
        }
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn ArtifactEnvelopeMutationFieldTarget<WriterMutation>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        let value = self.value.take().ok_or_else(|| self.diagnostic("writer-envelope.mutation-not-ready", 0))?;
        target.publish_mutation_reserved(reservation, value);
        self.published = true;
        self.terminal = true;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(mut active) = self.active.take() {
            active.authority.cancel();
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.retirement.is_some() { return store::artifact_retirement_box_close_step(&mut self.retirement, maximum_items, maximum_bytes).map_err(|_| store::OwnedSchemaDecodeDiagnostic { code: "writer-envelope.mutation-retirement-fault", offset: 0, line: 0, column: 0, path: self.path }); }
        if let Some(value) = self.value.take() {
            *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&WriterMutationRetirementFactory, value));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(index) = self.splice.strings.iter().position(Option::is_some) {
            let released_bytes = self.splice.strings[index].as_ref().map_or(0, String::capacity);
            if released_bytes > maximum_bytes {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            drop(self.splice.strings[index].take());
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
        }
        if let Some(payload) = self.payload.as_ref() {
            if payload.capacity() > maximum_bytes {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let released_bytes = payload.capacity();
            drop(self.payload.take());
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.active.is_none() && self.payload.is_none() && self.splice.strings.iter().all(Option::is_none) && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for WriterMutationDecodeAuthority {
    fn drop(&mut self) {
        // 🧯️ `std::thread::panicking()` FIRST, exactly like the framework's own
        // `store::ArtifactEnvelope::drop`: a Drop witness exists to catch a leak on a HEALTHY path.
        // Firing it while the thread is ALREADY unwinding turns a reported failure into
        // `panic in a destructor during cleanup` — a non-unwinding abort that kills the whole test
        // binary and hides the first, real failure (that is how one red test took this crate's
        // other 300 with it).
        assert!(std::thread::panicking() || (store::ArtifactEnvelopeMutationFieldAuthority::terminal_is_empty(self)), "Writer mutation decode reached Drop before exact publication or bounded retirement");
    }
}

impl store::ArtifactEnvelopeOwnedFieldCatalog<WriterSnapshot, WriterMutation> for WriterEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Result<Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<WriterSnapshot, WriterMutation>>, Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<WriterSnapshot>>> {
        store::ArtifactEnvelopeFreshVcsAuthority::try_new(self.begin_snapshot(operation, generation, path), std::sync::Arc::new(WriterSnapshotRetirementFactory), std::sync::Arc::new(WriterMutationRetirementFactory), self.edit_history_decoder())
            .map(|authority| Box::new(authority) as Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<WriterSnapshot, WriterMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<WriterSnapshot>> {
        Box::new(WriterSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<WriterMutation>> {
        Box::new(WriterMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(WriterRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<WriterMutation>>> {
        store::artifact_owned_spr_edit_history_decoder::<WriterSnapshot, WriterMutation>(std::sync::Arc::new(WriterEnvelopeOwnedFieldCatalog), std::sync::Arc::new(WriterMutationRetirementFactory))
    }
}

pub fn writer_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<WriterSnapshot, WriterMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(WriterEnvelopeOwnedFieldCatalog), std::sync::Arc::new(WriterSnapshotRetirementFactory), std::sync::Arc::new(WriterMutationRetirementFactory))
}

pub type WriterDocumentEnvelope = store::ArtifactEnvelope<WriterSnapshot, WriterMutation>;

struct WriterSnapshotDecodeAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    state: WriterSnapshotDecodeState,
    value: std::mem::ManuallyDrop<Option<WriterSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl WriterSnapshotDecodeAuthority {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self { operation, generation, path, state: WriterSnapshotDecodeState::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
    }
}

const WRITER_MUTATION_FIELDS: &[store::OwnedSchemaFieldSpec] = &[
    store::OwnedSchemaFieldSpec { id: 1, key: "mutation", required: true },
    store::OwnedSchemaFieldSpec { id: 2, key: "newId", required: false },
    store::OwnedSchemaFieldSpec { id: 3, key: "newUri", required: false },
    store::OwnedSchemaFieldSpec { id: 4, key: "newLanguageId", required: false },
    store::OwnedSchemaFieldSpec { id: 5, key: "text", required: false },
    store::OwnedSchemaFieldSpec { id: 6, key: "start", required: false },
    store::OwnedSchemaFieldSpec { id: 7, key: "deleted", required: false },
    store::OwnedSchemaFieldSpec { id: 8, key: "insert", required: false },
    store::OwnedSchemaFieldSpec { id: 9, key: "before", required: false },
    store::OwnedSchemaFieldSpec { id: 10, key: "after", required: false },
];

/// ✂️ `SpliceText`'s four string fields (ids 7–10) and its scalar `start` (id 6), gathered before the record completes.
#[derive(Default)]
struct WriterSplicePayload {
    start: Option<u32>,
    strings: [Option<String>; 4],
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WriterMutationKind {
    RenameWriter,
    ChangeUri,
    ChangeLanguage,
    EditText,
    SpliceText,
}

struct WriterMutationDecodeAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    cursor: store::OwnedSchemaNestedRecordCursor,
    active: Option<WriterMutationString>,
    kind: Option<WriterMutationKind>,
    payload_field: Option<u16>,
    payload: std::mem::ManuallyDrop<Option<String>>,
    splice: WriterSplicePayload,
    value: std::mem::ManuallyDrop<Option<WriterMutation>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    published: bool,
    terminal: bool,
}

struct WriterRejectedConflictAuthority {
    terminal: bool,
}

impl store::ArtifactEnvelopeSprConflictAuthority for WriterRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "writer-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })
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

pub struct WriterEnvelopeOwnedFieldCatalog;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
