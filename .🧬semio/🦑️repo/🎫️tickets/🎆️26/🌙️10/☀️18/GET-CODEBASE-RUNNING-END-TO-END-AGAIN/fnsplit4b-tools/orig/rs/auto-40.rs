//! 🚀️ raster store initialization: the document store owners, the store initialization authority with its phase machine and the initialization job.

use super::*;

pub type RasterDocumentStoreOwners = store::os_store::document_store_owners::DocumentStoreOwners<RasterSnapshot, RasterMutation>;

pub type RasterOwnersAdmission = Result<(RasterDocumentStoreOwners, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<RasterSnapshot, RasterMutation>>;

pub fn raster_document_store_owners_admission(grant: RetainedCloneGrant) -> RasterOwnersAdmission {
    RasterDocumentStoreOwners::admit_source_constructor(grant, || (RasterSnapshotRetirementFactory, RasterSnapshotRetirementFactory, RasterMutationRetirementFactory, store::ArtifactStoreCursorDisposer::<RasterSnapshot, RasterMutation>::new()))
}

/// 📏️ Quotes the source Arcs and disposer the Raster owner catalog admits before any factory ticket is born.
pub fn raster_document_store_owners_source_demands() -> Result<semio_framework_value::RetirementDemand, ValueError> {
    let capacity_bytes = RasterDocumentStoreOwners::source_birth_bytes::<RasterSnapshotRetirementFactory, RasterSnapshotRetirementFactory, RasterMutationRetirementFactory, store::ArtifactStoreCursorDisposer<RasterSnapshot, RasterMutation>>()?;
    Ok(semio_framework_value::RetirementDemand { capacity_bytes, depth: 1, ..Default::default() })
}

pub fn raster_document_store_owners() -> RasterDocumentStoreOwners {
    let capacity_bytes = RasterDocumentStoreOwners::source_birth_bytes::<RasterSnapshotRetirementFactory, RasterSnapshotRetirementFactory, RasterMutationRetirementFactory, store::ArtifactStoreCursorDisposer<RasterSnapshot, RasterMutation>>().expect("Raster owner catalog source layout");
    store::os_store::document_store_owners::fund_document_store_owners(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes, depth: 1 }, raster_document_store_owners_admission).expect("funded Raster owner catalog")
}

pub(super) struct RasterStoreInitializationAuthority {
    actor: std::mem::ManuallyDrop<Option<protocol::ActorId>>,
    actor_close: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<protocol::ActorId>>>,
    pub(super) operation: semio_framework_job::OperationId,
    pub(super) generation: semio_framework_job::Generation,
    pub(super) envelope: std::mem::ManuallyDrop<Option<store::os_store::store_schema::ArtifactEnvelope<RasterSnapshot, RasterMutation>>>,
    owners: std::mem::ManuallyDrop<Option<RasterDocumentStoreOwners>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<RasterSnapshot>>>,
    pub(super) candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<RasterSnapshot, RasterMutation>>>,
    pub(super) active: RasterRetirementSlot,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub(super) clone: std::mem::ManuallyDrop<Option<RasterSnapshotCloneAuthority>>,
    pub(super) mutation_candidate: std::mem::ManuallyDrop<Option<RasterMutationCandidateAuthority>>,
    publication: semio_framework_job_publication::RetainedJobPublication,
    delivered: bool,
    edit_index: store::ArtifactStoreInitializationEditIndex,
    pub(super) phase: RasterStoreInitializationPhase,
    resume_phase: Option<RasterStoreInitializationPhase>,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    terminal_handoff: bool,
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<RasterSnapshot, RasterMutation> for RasterStoreInitializationAuthority {
    fn retirement_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        self.close_demands(maximum_copy_bytes)
    }

    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, ValueError> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"raster-store.initializer-stale-authority");
        }
        if (self.cancel_requested || cx.is_cancelled()) && !matches!(self.phase, RasterStoreInitializationPhase::RetireCancelled | RasterStoreInitializationPhase::Cancelled) {
            self.phase = RasterStoreInitializationPhase::RetireCancelled;
        }
        if cx.should_yield() {
            return Ok(None);
        }
        match self.pump_active(cx) {
            Ok(true) => return Ok(None),
            Ok(false) => {}
            Err(error) => {
                self.fault = Some(error.into_message().into_bytes());
                self.phase = RasterStoreInitializationPhase::RetireFault;
            }
        }
        if !matches!(self.phase, RasterStoreInitializationPhase::RetireCancelled | RasterStoreInitializationPhase::RetireFault | RasterStoreInitializationPhase::Cancelled | RasterStoreInitializationPhase::Fault | RasterStoreInitializationPhase::Complete) {
            if let Some(runtime) = self.runtime.as_mut() {
                match runtime.settle_current_retirement_step(cx.retained_grant()) {
                    Ok(RetainedCloneStep::Complete(_)) => {}
                    Ok(RetainedCloneStep::Progress(progress)) => {
                        if let Err(error) = cx.consume_retained(progress) {
                            self.fault = Some(error.into_message().into_bytes());
                            self.phase = RasterStoreInitializationPhase::RetireFault;
                        }
                        cx.consume_fuel(1);
                        return Ok(None);
                    }
                    Err(error) => { self.fault = Some(error.into_message().into_bytes()); self.phase = RasterStoreInitializationPhase::RetireFault; }
                }
            }
        }
        match self.phase {
            RasterStoreInitializationPhase::BindGenesis => {
                let envelope = self.envelope.as_ref().expect("retained initializer genesis");
                let actor = self.actor.as_ref().expect("retained initializer actor").clone();
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, envelope.vcs.genesis.facts().share_snapshot(), envelope.vcs.genesis.facts().digest(), actor));
                self.phase = RasterStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::ValidateEnvelope => {
                let Some(envelope) = self.envelope.as_ref() else {
                    self.fail(b"raster-store.initializer-envelope-missing");
                    return Ok(None);
                };
                if envelope.schema != semio_s_artifact_raster_raster_model::RASTER_DOCUMENT_SCHEMA || envelope.id.is_empty() || envelope.id.len() > RASTER_OWNED_FIELD_BYTES {
                    self.fail(b"raster-store.initializer-envelope-invalid");
                } else {
                    self.phase = RasterStoreInitializationPhase::ValidateEdit { index: 0 };
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::ValidateEdit { index } => {
                let envelope = self.envelope.as_ref().expect("validated Raster envelope remains retained");
                match self.edit_index.admit(&envelope.vcs.edits, index, RASTER_OWNED_FIELD_BYTES) {
                    store::ArtifactStoreInitializationEditAdmission::Complete => self.phase = RasterStoreInitializationPhase::BindGenesis,
                    store::ArtifactStoreInitializationEditAdmission::Admitted => self.phase = RasterStoreInitializationPhase::ValidateEdit { index: index + 1 },
                    store::ArtifactStoreInitializationEditAdmission::Oversized => self.fail(b"raster-store.initializer-hostile-edit-id"),
                    store::ArtifactStoreInitializationEditAdmission::Duplicate => self.fail(b"raster-store.initializer-duplicate-edit"),
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::CloneInitial => {
                let source = &self.envelope.as_ref().expect("Raster envelope remains retained during initial clone").vcs.genesis.facts().snapshot();
                let clone = self.clone.as_mut().expect("Raster initial clone authority remains retained");
                let complete = match clone.step(source, cx) {
                    Ok(complete) => complete,
                    Err(code) => {
                        self.fail(code.as_bytes());
                        return Ok(None);
                    }
                };
                if complete {
                    let initial = clone.take_value().expect("Raster initial snapshot was built one semantic item at a time");
                    drop(self.clone.take());
                    match self.runtime.as_mut().expect("retained initializer runtime").adopt_current_owned(initial, std::sync::Arc::new(RasterSnapshotRetirementFactory)) {
                        Ok(()) => self.phase = self.resume_phase.take().expect("retained mutation resume phase"),
                        Err(initial) => {
                            self.active.put(RasterDisplaced::Snapshot(initial));
                            self.fail(b"initializer-owned-workspace-adoption");
                        }
                    }
                }
                Ok(None)
            }
            RasterStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("Raster envelope remains retained while causal history is seeded");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = RasterStoreInitializationPhase::FoldSupersessions { transition: 0 };
                    return Ok(None);
                };
                let runtime = self.runtime.as_mut().expect("Raster runtime remains retained while history is seeded");
                match lane {
                    0 => {
                        if let Err(error) = runtime.seed_mutation(protocol::MutationId(entry.id.clone())) {
                            self.fault = Some(error.into_bytes());
                            self.phase = RasterStoreInitializationPhase::RetireFault;
                        } else {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = RasterStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                    }
                    1 if index < entry.forwards.len() => {
                        let id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| entry.forwards[index].mutation_id()).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                        if let Err(error) = runtime.seed_edit_operation(&entry.id, id) {
                            self.fault = Some(error.into_bytes());
                            self.phase = RasterStoreInitializationPhase::RetireFault;
                        } else {
                            self.phase = RasterStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                        }
                    }
                    1 => self.phase = RasterStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp);
                        self.phase = RasterStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = RasterStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::FoldSupersessions { transition } => {
                let envelope = self.envelope.as_ref().expect("Raster envelope remains retained while its supersessions fold");
                match self.runtime.as_mut().expect("Raster runtime remains retained while its supersessions fold").fold_supersession_step(envelope, transition) {
                    Ok(true) => self.phase = RasterStoreInitializationPhase::FoldSupersessions { transition: transition + 1 },
                    Ok(false) => self.phase = RasterStoreInitializationPhase::FindApplied { position: 0 },
                    Err(error) => {
                        self.fault = Some(error.into_bytes());
                        self.phase = RasterStoreInitializationPhase::RetireFault;
                    }
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::FindApplied { position } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
                    self.runtime.as_mut().expect("Raster runtime remains retained").set_current_checkpoint_id(checkpoint);
                    self.phase = RasterStoreInitializationPhase::FindRedo { position: 0 };
                    return Ok(None);
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("Raster envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"raster-store.initializer-applied-edit-missing");
                    return Ok(None);
                };
                if edit.id == id {
                    self.phase = RasterStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                } else {
                    self.fail(b"raster-store.initializer-applied-edit-missing");
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let needs_workspace = {
                    let envelope = self.envelope.as_ref().expect("retained initializer envelope");
                    let runtime = self.runtime.as_ref().expect("retained initializer runtime");
                    envelope.vcs.edits.get(edit).and_then(|entry| runtime.effective_forward(entry, mutation, &envelope.schema)).is_some_and(|effective| effective.operation().is_some())
                };
                if needs_workspace && self.runtime.as_mut().expect("retained initializer runtime").current_mut().is_none() {
                    self.resume_phase = Some(self.phase);
                    *self.clone = Some(RasterSnapshotCloneAuthority::new());
                    self.phase = RasterStoreInitializationPhase::CloneInitial;
                    cx.consume_fuel(1);
                    return Ok(None);
                }
                if self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).is_none_or(|entry| mutation >= entry.forwards.len()) {
                    self.phase = RasterStoreInitializationPhase::CommitApplied { position, edit };
                    return Ok(None);
                }
                let withdrawn = {
                    let envelope = self.envelope.as_ref().expect("Raster envelope remains retained while its forwards fold");
                    envelope.vcs.edits.get(edit).and_then(|entry| self.runtime.as_ref().and_then(|runtime| runtime.effective_forward(entry, mutation, &envelope.schema))).is_none_or(|effective| effective.operation().is_none())
                };
                if withdrawn {
                    self.phase = RasterStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                    cx.consume_fuel(1);
                    return Ok(None);
                }
                if self.mutation_candidate.is_none() {
                    if !raster_reserve_unit(cx) {
                        return Ok(None);
                    }
                    *self.mutation_candidate = Some(RasterMutationCandidateAuthority::new());
                    return Ok(None);
                }
                let envelope = self.envelope.as_ref().expect("Raster envelope remains retained while its forwards fold");
                let effective = envelope.vcs.edits.get(edit).and_then(|entry| self.runtime.as_ref().and_then(|runtime| runtime.effective_forward(entry, mutation, &envelope.schema))).expect("Raster applied forward remains retained");
                let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("Raster runtime current snapshot remains retained");
                let stepped = self.mutation_candidate.as_mut().expect("Raster mutation candidate remains retained").step(current, effective.operation().expect("Raster effective forward was checked"), cx);
                drop(effective);
                let candidate_complete = match stepped {
                    Ok(value) => value,
                    Err(code) => {
                        self.fail(code.as_bytes());
                        return Ok(None);
                    }
                };
                if candidate_complete {
                    let next = self.mutation_candidate.as_mut().expect("Raster completed mutation candidate remains retained").take().expect("Raster mutation candidate handoff");
                    drop(self.mutation_candidate.take());
                    let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("Raster runtime current snapshot remains retained");
                    let previous = std::mem::replace(current, next);
                    self.active.put(RasterDisplaced::Snapshot(previous));
                    self.phase = RasterStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                }
                Ok(None)
            }
            RasterStoreInitializationPhase::CommitApplied { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Raster applied edit remains retained");

                let runtime = self.runtime.as_mut().expect("Raster runtime remains retained");
                if let Err(error) = runtime.push_applied_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fault = Some(error.into_bytes());
                    self.phase = RasterStoreInitializationPhase::RetireFault;
                } else {

                    self.phase = RasterStoreInitializationPhase::FindApplied { position: position + 1 };
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::FindRedo { position } => {
                let Some(id) = self.redo_id(position) else {
                    self.edit_index.clear();
                    self.phase = RasterStoreInitializationPhase::RetireActor;
                    return Ok(None);
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("Raster envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"raster-store.initializer-redo-edit-missing");
                    return Ok(None);
                };
                if edit.id == id {
                    self.phase = RasterStoreInitializationPhase::CommitRedo { position, edit: scan };
                } else {
                    self.fail(b"raster-store.initializer-redo-edit-missing");
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::CommitRedo { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Raster redo edit remains retained");
                if let Err(error) = self.runtime.as_mut().expect("Raster runtime remains retained").push_redo_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fault = Some(error.into_bytes());
                    self.phase = RasterStoreInitializationPhase::RetireFault;
                } else {
                    self.phase = RasterStoreInitializationPhase::FindRedo { position: position + 1 };
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::RetireActor => {
                if self.actor.is_none() && self.actor_close.is_none() {
                    self.phase = RasterStoreInitializationPhase::BuildCandidate;
                    return Ok(None);
                }
                match self.step_actor(cx.retained_grant()) {
                    Ok(step) => {
                        if let Err(error) = cx.consume_retained(step.progress()) {
                            self.fault = Some(error.into_message().into_bytes());
                            self.phase = RasterStoreInitializationPhase::RetireFault;
                        }
                    }
                    Err(error) => {
                        self.fault = Some(error.into_message().into_bytes());
                        self.phase = RasterStoreInitializationPhase::RetireFault;
                    }
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::BuildCandidate => {
                if !raster_reserve_unit(cx) {
                    return Ok(None);
                }
                let Some(candidate_generation) = self.generation.0.checked_add(1) else {
                    self.fail(b"raster-store.initializer-generation-exhausted");
                    return Ok(None);
                };
                let envelope = self.envelope.take().expect("Raster envelope remains retained until atomic store construction");
                let runtime = self.runtime.take().expect("Raster runtime remains retained until atomic store construction");
                let candidate = store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, raster_document_store_owners());
                *self.candidate = Some(candidate);
                self.phase = RasterStoreInitializationPhase::Complete;
                semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None)
            }
            RasterStoreInitializationPhase::RetireCancelled | RasterStoreInitializationPhase::RetireFault => match self.pump_terminal_retirement(cx) {
                Ok(false) => Ok(None),
                Ok(true) => {
                    self.terminal_handoff = true;
                    if self.phase == RasterStoreInitializationPhase::RetireCancelled {
                        self.phase = RasterStoreInitializationPhase::Cancelled;
                        semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx)
                    } else {
                        self.phase = RasterStoreInitializationPhase::Fault;
                        Ok(None)
                    }
                }
                Err(error) => {
                    self.fault = Some(error.into_message().into_bytes());
                    Ok(None)
                }
            },
            RasterStoreInitializationPhase::Complete => semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None),
            RasterStoreInitializationPhase::Cancelled => semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx),
            RasterStoreInitializationPhase::Fault => {
                if self.delivered {
                    let step = self.publication.close_step(cx.retained_grant())?;
                    cx.consume_retained(step.progress())?;
                    if matches!(step, RetainedCloneStep::Complete(_)) {
                        self.delivered = false;
                    }
                    return Ok(None);
                }
                let result = self.publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Fault, self.fault.as_deref().unwrap_or(b"raster-store.initializer-fault"), cx)?;
                if result.is_some() {
                    self.delivered = true;
                }
                Ok(result)
            }
        }
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Complete if self.phase == RasterStoreInitializationPhase::Complete => descriptor.complete(None, None),
            _ => self.publication.borrow_outcome(descriptor),
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, RasterStoreInitializationPhase::Cancelled | RasterStoreInitializationPhase::Fault) {
            self.phase = RasterStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.begin_close();
        let step = self.close_original(grant)?;
        if self.close_is_exhausted() {
            self.terminal_handoff = true;
            return Ok(RetainedCloneStep::Complete(step.progress()));
        }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<RasterSnapshot, RasterMutation>> {
        if self.phase != RasterStoreInitializationPhase::Complete || self.terminal_handoff {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RasterStoreInitializationPhase {
    BindGenesis,
    ValidateEnvelope,
    ValidateEdit { index: usize },
    CloneInitial,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FoldSupersessions { transition: usize },
    FindApplied { position: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize },
    CommitRedo { position: usize, edit: usize },
    RetireActor,
    BuildCandidate,
    RetireCancelled,
    RetireFault,
    Complete,
    Cancelled,
    Fault,
}

impl RasterStoreInitializationAuthority {
    pub(super) fn new(envelope: store::os_store::store_schema::ArtifactEnvelope<RasterSnapshot, RasterMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Self {
        Self {
            actor: std::mem::ManuallyDrop::new(Some(actor)),
            actor_close: std::mem::ManuallyDrop::new(None),
            operation,
            generation,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            owners: std::mem::ManuallyDrop::new(None),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            active: RasterRetirementSlot::new(),
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            clone: std::mem::ManuallyDrop::new(None),
            mutation_candidate: std::mem::ManuallyDrop::new(None),
            publication: semio_framework_job_publication::RetainedJobPublication::new(),
            delivered: false,
            edit_index: store::ArtifactStoreInitializationEditIndex::default(),
            resume_phase: None,
            phase: RasterStoreInitializationPhase::ValidateEnvelope,
            cancel_requested: false,
            fault: None,
            terminal_handoff: false,
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
        self.phase = RasterStoreInitializationPhase::RetireFault;
    }

    fn pump_active(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, ValueError> {
        if self.active.is_empty() {
            return Ok(false);
        }
        let step = self.active.close(cx.retained_grant())?;
        cx.consume_retained(step.progress())?;
        cx.consume_fuel(1);
        Ok(true)
    }

    fn actor_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(actor) = self.actor_close.as_ref() {
            if actor.terminal_is_empty() {
                return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Option<semio_framework_value::retirement::controlled::ControlledRetirement<protocol::ActorId>>>(), depth: 1, ..Default::default() });
            }
            return raster_nested(semio_framework_value::RetirementDemand { copy_bytes: actor.next_copy_byte_demand()?, capacity_bytes: actor.next_capacity_byte_demand(body)?, release_bytes: actor.next_release_byte_demand()?, depth: actor.next_depth_demand()? });
        }
        if self.actor.is_some() {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Option<protocol::ActorId>>() + size_of::<Option<semio_framework_value::retirement::controlled::ControlledRetirement<protocol::ActorId>>>(), depth: 1, ..Default::default() });
        }
        Ok(Default::default())
    }

    fn step_actor(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.actor.is_none() && self.actor_close.is_none() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.actor_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "Raster initializer actor exceeds admitted depth"));
        }
        if raster_short(grant, demand) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let child = raster_child_grant(grant);
        if let Some(actor) = self.actor_close.as_mut() {
            if actor.terminal_is_empty() {
                drop(self.actor_close.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let step = actor.step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, actor.terminal_is_empty(), "Raster initializer original actor")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let actor = self.actor.take().expect("Raster initializer actor remains retained until its retirement frame");
        match semio_framework_value::retirement::controlled::ControlledRetirement::new(actor) {
            Ok(owner) => {
                *self.actor_close = Some(owner);
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }))
            }
            Err((error, actor)) => {
                *self.actor = Some(actor);
                Err(error)
            }
        }
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_plugin::ArtifactOwnedDisposer;
        if !self.publication.terminal_is_empty() {
            return self.publication.retirement_demands();
        }
        if self.delivered {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        if !self.active.is_empty() {
            return self.active.demands(body);
        }
        if let Some(candidate) = self.candidate.as_ref() {
            return semio_framework_plugin::ArtifactDocumentStoreDisposer::<RasterSnapshot, RasterMutation>::new().retirement_demands(candidate, body);
        }
        if let Some(candidate) = self.mutation_candidate.as_ref() {
            return candidate.close_demands(body);
        }
        if let Some(runtime) = self.runtime.as_ref() {
            return runtime.initialization_retirement_demands(body);
        }
        if let Some(clone) = self.clone.as_ref() {
            return clone.close_demands(body);
        }
        if let Some(retirement) = self.envelope_retirement.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(retirement, body);
        }
        if let Some(envelope) = self.envelope.as_ref() {
            let Some(owners) = self.owners.as_ref() else {
                let capacity_bytes = RasterDocumentStoreOwners::source_birth_bytes::<RasterSnapshotRetirementFactory, RasterSnapshotRetirementFactory, RasterMutationRetirementFactory, store::ArtifactStoreCursorDisposer<RasterSnapshot, RasterMutation>>()?;
                return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Option<RasterDocumentStoreOwners>>(), capacity_bytes, depth: 1, ..Default::default() });
            };
            if !owners.constructor_is_complete() {
                return raster_nested(owners.constructor_demands(body)?);
            }
            let mut demand = raster_nested(owners.uninstalled_envelope_retirement_demands(envelope))?;
            demand.copy_bytes = demand.copy_bytes.checked_add(size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>()).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "Raster initializer envelope frame copy overflow"))?;
            return Ok(demand);
        }
        if let Some(owners) = self.owners.as_ref() {
            if owners.uninstalled_owners_terminal_is_empty() {
                return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Option<RasterDocumentStoreOwners>>(), depth: 1, ..Default::default() });
            }
            return raster_nested(owners.uninstalled_owners_demands(body)?);
        }
        self.actor_demands(body)
    }

    fn close_original(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_plugin::ArtifactOwnedDisposer;
        let empty = RetainedCloneProgress::default();
        if self.close_is_exhausted() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "Raster initializer close exceeds admitted depth"));
        }
        if raster_short(grant, demand) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let child = raster_child_grant(grant);
        if !self.publication.terminal_is_empty() {
            return self.publication.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.delivered {
            self.delivered = false;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if !self.active.is_empty() {
            return self.active.close(grant);
        }
        if let Some(candidate) = self.candidate.as_mut() {
            let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<RasterSnapshot, RasterMutation>::new();
            let step = disposer.close_step(candidate, grant).map_err(|fault| ValueError::new(ValueRefusalKind::InvariantViolated, format!("{}: {}", fault.code.0, fault.message)))?;
            if disposer.terminal_is_empty(candidate) {
                drop(self.candidate.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress().unwrap_or_default()));
        }
        if let Some(candidate) = self.mutation_candidate.as_mut() {
            let step = candidate.close_step(grant)?;
            if candidate.terminal_is_empty() {
                drop(self.mutation_candidate.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let factory: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<RasterSnapshot>> = std::sync::Arc::new(RasterSnapshotRetirementFactory);
            let step = runtime.close_step(&factory, grant)?;
            if runtime.terminal_is_empty() {
                drop(self.runtime.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(clone) = self.clone.as_mut() {
            let step = clone.close_step(grant)?;
            if clone.terminal_is_empty() {
                drop(self.clone.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope_retirement.is_some() {
            let step = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.envelope_retirement, grant)?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope.is_some() {
            if self.owners.is_none() {
                let placement = demand.copy_bytes;
                let funded = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - placement, ..grant };
                return match raster_document_store_owners_admission(funded) {
                    Ok((owners, mut receipt)) => {
                        *self.owners = Some(owners);
                        receipt.copied_bytes += placement;
                        Ok(RetainedCloneStep::Progress(receipt))
                    }
                    Err(refused) => {
                        *self.owners = refused.owners;
                        Err(refused.error.with_retained_progress(refused.progress))
                    }
                };
            }
            let owners = self.owners.as_mut().expect("Raster initializer catalog remains retained");
            if !owners.constructor_is_complete() {
                return owners.admit_constructor(child).map(RetainedCloneStep::Progress).map_err(|(error, receipt)| error.with_retained_progress(receipt));
            }
            let placement = size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>();
            let funded = RetainedCloneGrant { maximum_copy_bytes: child.maximum_copy_bytes - placement, ..child };
            let owners = self.owners.take().expect("Raster initializer catalog remains retained");
            let envelope = self.envelope.take().expect("Raster initializer envelope remains retained");
            return match owners.retire_envelope_uninstalled(envelope, funded) {
                Ok((owner, mut receipt)) => {
                    *self.envelope_retirement = Some(owner);
                    receipt.copied_bytes += placement;
                    Ok(RetainedCloneStep::Progress(receipt))
                }
                Err((error, owners, envelope)) => {
                    *self.owners = Some(owners);
                    *self.envelope = Some(envelope);
                    Err(error)
                }
            };
        }
        if let Some(owners) = self.owners.as_mut() {
            if owners.uninstalled_owners_terminal_is_empty() {
                drop(self.owners.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let step = owners.close_uninstalled_owners_step(child)?;
            return semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owners.uninstalled_owners_terminal_is_empty(), "Raster initializer original catalog").map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        self.step_actor(grant)
    }

    fn close_is_exhausted(&self) -> bool {
        self.publication.terminal_is_empty() && !self.delivered && self.active.is_empty() && self.candidate.is_none() && self.mutation_candidate.is_none() && self.runtime.is_none() && self.clone.is_none() && self.envelope_retirement.is_none() && self.envelope.is_none() && self.owners.is_none() && self.actor.is_none() && self.actor_close.is_none()
    }

    fn pump_terminal_retirement(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, ValueError> {
        if self.close_is_exhausted() {
            return Ok(true);
        }
        let step = self.close_original(cx.retained_grant())?;
        cx.consume_retained(step.progress())?;
        cx.consume_fuel(1);
        Ok(self.close_is_exhausted())
    }

    fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff && self.close_is_exhausted()
    }
}

impl Drop for RasterStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!((self.terminal_is_empty_inner()) || std::thread::panicking(), "Raster store initialization authority reached Drop before exact candidate handoff or retained rejection close");
    }
}

pub fn raster_document_store_initialization_job(
    envelope: store::os_store::store_schema::ArtifactEnvelope<RasterSnapshot, RasterMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    actor: protocol::ActorId,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<RasterSnapshot, RasterMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(RasterStoreInitializationAuthority::new(envelope, operation, generation, actor)))
}

/// 🧯️ A retirement or clone cursor found its own bookkeeping inconsistent.
pub(super) fn invariant(message: &str) -> ValueError {
    ValueError::new(ValueRefusalKind::InvariantViolated, message)
}
