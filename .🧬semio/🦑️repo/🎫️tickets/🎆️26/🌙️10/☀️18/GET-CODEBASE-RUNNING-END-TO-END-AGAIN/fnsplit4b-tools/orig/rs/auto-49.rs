//! 🏗️ Store initialization authority: the store initialization authority struct and its artifact store initialization authority implementation. See [`super`].

use super::*;

pub(super) struct Generation2dStoreInitializationAuthority {
    pub(super) actor: protocol::ActorId,
    pub(super) operation: semio_framework_job::OperationId,
    pub(super) generation: semio_framework_job::Generation,
    pub(super) base_revision: u64,
    pub(super) parent_revision: u64,
    pub(super) history_items: usize,
    pub(super) envelope: std::mem::ManuallyDrop<Option<store::os_store::store_schema::ArtifactEnvelope<Generation2dSnapshot, Generation2dMutation>>>,
    pub(super) copy: std::mem::ManuallyDrop<Option<Generation2dSnapshotCopyCursor>>,
    pub(super) runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<Generation2dSnapshot>>>,
    pub(super) owners: std::mem::ManuallyDrop<Option<store::os_store::document_store_owners::DocumentStoreOwners<Generation2dSnapshot, Generation2dMutation>>>,
    pub(super) candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<Generation2dSnapshot, Generation2dMutation>>>,
    pub(super) active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub(super) displaced: std::mem::ManuallyDrop<Option<Generation2dReplayDisplaced>>,
    pub(super) rejected: std::mem::ManuallyDrop<Option<Generation2dSnapshot>>,
    pub(super) candidate_disposer: std::mem::ManuallyDrop<Option<semio_framework_plugin::ArtifactDocumentStoreDisposer<Generation2dSnapshot, Generation2dMutation>>>,
    pub(super) closer: std::mem::ManuallyDrop<Option<semio_framework_plugin::ArtifactStoreInitializationJob<Generation2dSnapshot, Generation2dMutation>>>,
    pub(super) snapshot_factory: std::mem::ManuallyDrop<Option<std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<Generation2dSnapshot>>>>,
    pub(super) factory_close: std::mem::ManuallyDrop<Option<semio_framework_value::FactoryAuthority>>,
    pub(super) publication: semio_framework_job_publication::RetainedJobPublication,
    pub(super) edit_index: store::ArtifactStoreInitializationEditIndex,
    pub(super) phase: Generation2dStoreInitializationPhase,
    pub(super) resume_phase: Option<Generation2dStoreInitializationPhase>,
    pub(super) cancel_requested: bool,
    pub(super) fault: Option<Vec<u8>>,
    pub(super) terminal_handoff: bool,
}

const GENERATION2D_INITIALIZER_DEFAULT_FAULT: &[u8] = b"generation2d-store.initializer-fault";

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<Generation2dSnapshot, Generation2dMutation> for Generation2dStoreInitializationAuthority {
    fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        self.close_demands(body)
    }

    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        use semio_framework_job::JobOutcomeBorrow;
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"generation2d-store.initializer-stale-aba");
        }
        if (self.cancel_requested || cx.is_cancelled()) && !matches!(self.phase, Generation2dStoreInitializationPhase::RetireCancelled | Generation2dStoreInitializationPhase::Cancelled) {
            self.phase = Generation2dStoreInitializationPhase::RetireCancelled;
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(None);
        }
        match self.pump_owned(cx) {
            Ok(true) => {
                cx.consume_fuel(1);
                return Ok(None);
            }
            Ok(false) => {}
            Err(error) => {
                self.fault = Some(error.into_message().into_bytes());
                self.phase = Generation2dStoreInitializationPhase::RetireFault;
            }
        }
        if !matches!(self.phase, Generation2dStoreInitializationPhase::RetireCancelled | Generation2dStoreInitializationPhase::RetireFault | Generation2dStoreInitializationPhase::Cancelled | Generation2dStoreInitializationPhase::Fault | Generation2dStoreInitializationPhase::Complete) {
            if let Some(runtime) = self.runtime.as_mut() {
                match runtime.settle_current_retirement_step(cx.retained_grant()) {
                    Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) => {}
                    Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)) => {
                        cx.consume_retained(progress)?;
                        cx.consume_fuel(1);
                        return Ok(None);
                    }
                    Err(error) => {
                        self.fault = Some(error.into_message().into_bytes());
                        self.phase = Generation2dStoreInitializationPhase::RetireFault;
                    }
                }
            }
        }
        match self.phase {
            Generation2dStoreInitializationPhase::BindGenesis => {
                let envelope = self.envelope.as_ref().expect("retained initializer genesis");
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, envelope.vcs.genesis.facts().share_snapshot(), envelope.vcs.genesis.facts().digest(), self.actor.clone()));
                self.phase = Generation2dStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                cx.consume_fuel(1);
                return Ok(None);
            }
            Generation2dStoreInitializationPhase::ValidateEnvelope => {
                let valid = self.envelope.as_ref().is_some_and(|envelope| envelope.schema == semio_s_artifact_procedural_2d_model::GENERATION_2D_SCHEMA && !envelope.id.is_empty() && envelope.id.len() <= GENERATION2D_OWNER_BYTES);
                if valid {
                    self.phase = Generation2dStoreInitializationPhase::ValidateEdit { index: 0 };
                } else {
                    self.fail(b"generation2d-store.initializer-envelope-invalid");
                }
            }
            Generation2dStoreInitializationPhase::ValidateEdit { index } => {
                let envelope = self.envelope.as_ref().expect("P2 envelope retained");
                match self.edit_index.admit(&envelope.vcs.edits, index, usize::MAX) {
                    store::ArtifactStoreInitializationEditAdmission::Complete => self.phase = Generation2dStoreInitializationPhase::CensusHistory { edit: 0, mutation: 0 },
                    store::ArtifactStoreInitializationEditAdmission::Admitted => self.phase = Generation2dStoreInitializationPhase::ValidateEdit { index: index + 1 },
                    store::ArtifactStoreInitializationEditAdmission::Oversized | store::ArtifactStoreInitializationEditAdmission::Duplicate => self.fail(b"generation2d-store.initializer-duplicate-edit"),
                }
            }
            Generation2dStoreInitializationPhase::CensusHistory { edit, mutation } => {
                let envelope = self.envelope.as_ref().expect("P2 envelope retained");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = Generation2dStoreInitializationPhase::BindGenesis;
                    return Ok(None);
                };
                if entry.forwards.get(mutation).is_some() {
                    self.history_items = match self.history_items.checked_add(1) {
                        Some(value) if value <= GENERATION2D_MAXIMUM_DOMAIN_ITEMS => value,
                        _ => {
                            self.fail(b"generation2d-store.initializer-history-capacity");
                            return Ok(None);
                        }
                    };
                    self.phase = Generation2dStoreInitializationPhase::CensusHistory { edit, mutation: mutation + 1 };
                } else {
                    self.phase = Generation2dStoreInitializationPhase::CensusHistory { edit: edit + 1, mutation: 0 };
                }
            }
            Generation2dStoreInitializationPhase::CopyInitial => {
                let source = &self.envelope.as_ref().expect("P2 initializer envelope").vcs.genesis.facts().snapshot();
                match self.copy.as_mut().expect("P2 copy retained").step(source) {
                    Ok(true) => self.phase = Generation2dStoreInitializationPhase::AdoptWorkspace,
                    Ok(false) => {}
                    Err(code) => self.fail(code.as_bytes()),
                }
            }
            Generation2dStoreInitializationPhase::AdoptWorkspace => {
                let initial = self.copy.as_mut().expect("P2 copy retained").take().expect("P2 copy handoff");
                drop(self.copy.take());
                let factory = std::sync::Arc::clone(self.snapshot_factory.as_ref().expect("P2 snapshot issuer retained"));
                match self.runtime.as_mut().expect("retained initializer runtime").adopt_current_owned(initial, factory) {
                    Ok(()) => self.phase = self.resume_phase.take().expect("retained mutation resume phase"),
                    Err(initial) => {
                        *self.rejected = Some(initial);
                        self.fail(b"initializer-owned-workspace-adoption");
                    }
                }
            }
            Generation2dStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("P2 history retained");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = Generation2dStoreInitializationPhase::FoldSupersessions { transition: 0 };
                    return Ok(None);
                };
                let runtime = self.runtime.as_mut().expect("P2 runtime retained");
                match lane {
                    0 => match runtime.seed_mutation(protocol::MutationId(generation2d_copy_string(&entry.id).unwrap_or_default())) {
                        Ok(()) => {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = Generation2dStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                        Err(error) => {
                            self.fault = Some(error.into_bytes());
                            self.phase = Generation2dStoreInitializationPhase::RetireFault;
                        }
                    },
                    1 if index < entry.forwards.len() => {
                        let id = entry
                            .mutation_meta
                            .get(index)
                            .and_then(|meta| meta.mutation_id.as_ref()).map_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)), |id| protocol::MutationId(generation2d_copy_string(&id.0).unwrap_or_default()));
                        match runtime.seed_edit_operation(&entry.id, id) {
                            Ok(()) => self.phase = Generation2dStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 },
                            Err(error) => {
                                self.fault = Some(error.into_bytes());
                                self.phase = Generation2dStoreInitializationPhase::RetireFault;
                            }
                        }
                    }
                    1 => self.phase = Generation2dStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp);
                        self.phase = Generation2dStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = Generation2dStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
            }
            Generation2dStoreInitializationPhase::FoldSupersessions { transition } => {
                let envelope = self.envelope.as_ref().expect("P2 envelope remains retained while its supersessions fold");
                match self.runtime.as_mut().expect("P2 runtime remains retained while its supersessions fold").fold_supersession_step(envelope, transition) {
                    Ok(true) => self.phase = Generation2dStoreInitializationPhase::FoldSupersessions { transition: transition + 1 },
                    Ok(false) => self.phase = Generation2dStoreInitializationPhase::FindApplied { position: 0 },
                    Err(error) => {
                        self.fault = Some(error.into_bytes());
                        self.phase = Generation2dStoreInitializationPhase::RetireFault;
                    }
                }
            }
            Generation2dStoreInitializationPhase::FindApplied { position } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self
                        .envelope
                        .as_ref()
                        .and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.as_ref()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| &checkpoint.id)))
                        .and_then(|id| generation2d_copy_string(id).ok());
                    self.runtime.as_mut().expect("P2 runtime retained").set_current_checkpoint_id(checkpoint);
                    self.phase = Generation2dStoreInitializationPhase::FindRedo { position: 0 };
                    cx.consume_fuel(1);
                    return Ok(None);
                };
                let scan = self.edit_index.position(id).unwrap_or(usize::MAX);
                match self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(scan)) {
                    Some(edit) if edit.id == id => {
                        self.phase = Generation2dStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                    }
                    Some(_) => self.fail(b"generation2d-store.initializer-applied-missing"),
                    None => self.fail(b"generation2d-store.initializer-applied-missing"),
                }
            }
            Generation2dStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let needs_workspace = {
                    let envelope = self.envelope.as_ref().expect("retained initializer envelope");
                    let runtime = self.runtime.as_ref().expect("retained initializer runtime");
                    envelope.vcs.edits.get(edit).and_then(|entry| runtime.effective_forward(entry, mutation, &envelope.schema)).is_some_and(|effective| effective.operation().is_some())
                };
                if needs_workspace && self.runtime.as_mut().expect("retained initializer runtime").current_mut().is_none() {
                    self.resume_phase = Some(self.phase);
                    match Generation2dSnapshotCopyCursor::new(self.envelope.as_ref().expect("retained genesis").vcs.genesis.facts().snapshot()) {
                        Ok(copy) => *self.copy = Some(copy),
                        Err(code) => {
                            self.fail(code.as_bytes());
                            return Ok(None);
                        }
                    }
                    self.phase = Generation2dStoreInitializationPhase::CopyInitial;
                    cx.consume_fuel(1);
                    return Ok(None);
                }
                let operation = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).and_then(|entry| entry.forwards.get(mutation));
                let Some(_operation) = operation else {
                    self.phase = Generation2dStoreInitializationPhase::CommitApplied { position, edit };
                    return Ok(None);
                };
                let envelope = self.envelope.as_ref().expect("P2 envelope remains retained while its forwards fold");
                let entry = envelope.vcs.edits.get(edit).expect("P2 applied edit remains retained");
                let effective = self.runtime.as_ref().and_then(|runtime| runtime.effective_forward(entry, mutation, &envelope.schema)).expect("P2 applied forward remains retained");
                if effective.operation().is_none() {
                    drop(effective);
                    self.phase = Generation2dStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                    return Ok(None);
                }
                let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("P2 runtime current retained");
                let applied = effective.operation().map(|operation| generation2d_apply_initialization_mutation(current, operation));
                drop(effective);
                match applied {
                    Some(Ok(displaced)) => {
                        *self.displaced = displaced;
                        self.phase = Generation2dStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                    }
                    None => self.phase = Generation2dStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 },
                    Some(Err(code)) => self.fail(code.as_bytes()),
                }
            }
            Generation2dStoreInitializationPhase::CommitApplied { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("P2 applied edit retained");
                let runtime = self.runtime.as_mut().expect("P2 runtime retained");
                match runtime.push_applied_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    Ok(()) => {
                        runtime.observe_sequence(entry.sequence_number);
                        self.phase = Generation2dStoreInitializationPhase::FindApplied { position: position + 1 };
                    }
                    Err(_) => self.fail(b"generation2d-store.initializer-applied-capacity"),
                }
            }
            Generation2dStoreInitializationPhase::FindRedo { position } => {
                let Some(id) = self.redo_id(position) else {
                    self.edit_index.clear();
                    self.phase = Generation2dStoreInitializationPhase::BuildOwners;
                    cx.consume_fuel(1);
                    return Ok(None);
                };
                let scan = self.edit_index.position(id).unwrap_or(usize::MAX);
                match self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(scan)) {
                    Some(edit) if edit.id == id => {
                        self.phase = Generation2dStoreInitializationPhase::CommitRedo { position, edit: scan };
                    }
                    Some(_) => self.fail(b"generation2d-store.initializer-redo-missing"),
                    None => self.fail(b"generation2d-store.initializer-redo-missing"),
                }
            }
            Generation2dStoreInitializationPhase::CommitRedo { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("P2 redo edit retained");
                match self.runtime.as_mut().expect("P2 runtime retained").push_redo_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    Ok(()) => self.phase = Generation2dStoreInitializationPhase::FindRedo { position: position + 1 },
                    Err(_) => self.fail(b"generation2d-store.initializer-redo-capacity"),
                }
            }
            Generation2dStoreInitializationPhase::BuildOwners => {
                if self.owners.is_none() {
                    match store::os_store::document_store_owners::bounded_artifact_store_owners::<Generation2dSnapshot, Generation2dMutation>(cx.retained_grant()) {
                        Ok((owners, progress)) => {
                            *self.owners = Some(owners);
                            cx.consume_retained(progress)?;
                        }
                        Err(refused) => {
                            *self.owners = refused.owners;
                            if refused.error.retained_progress() != Default::default() {
                                cx.consume_retained(refused.error.retained_progress())?;
                            }
                        }
                    }
                } else if let Some(owners) = self.owners.as_mut().filter(|owners| !owners.constructor_is_complete()) {
                    match owners.admit_constructor(cx.retained_grant()) {
                        Ok(progress) => cx.consume_retained(progress)?,
                        Err((error, progress)) => {
                            cx.consume_retained(progress)?;
                            return Err(error);
                        }
                    }
                } else {
                    self.phase = Generation2dStoreInitializationPhase::BuildCandidate;
                }
                cx.consume_fuel(1);
                return Ok(None);
            }
            Generation2dStoreInitializationPhase::BuildCandidate => {
                let authority = generation2d_validate_publication_authority(self.operation, self.generation);
                let fresh =
                    cx.operation() == self.operation && cx.generation() == self.generation && authority == Ok((self.base_revision, self.parent_revision)) && self.base_revision == self.parent_revision && self.parent_revision == self.generation.0;
                let Some(candidate_generation) = self.parent_revision.checked_add(1) else {
                    self.fail(b"generation2d-store.initializer-generation-exhausted");
                    return Ok(None);
                };
                if !fresh {
                    self.fail(b"generation2d-store.initializer-parent-stale-aba");
                    return Ok(None);
                }
                let envelope = self.envelope.take().expect("P2 envelope retained until atomic publication");
                let runtime = self.runtime.take().expect("P2 runtime retained until atomic publication");
                let owners = self.owners.take().expect("P2 owners retained until atomic publication");
                *self.candidate = Some(store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, owners));
                self.phase = Generation2dStoreInitializationPhase::Complete;
                return JobOutcomeBorrow::admit_complete(cx, None, None);
            }
            Generation2dStoreInitializationPhase::RetireCancelled | Generation2dStoreInitializationPhase::RetireFault => match self.close_turn(cx.retained_grant()) {
                Ok(progress) => {
                    cx.consume_retained(progress)?;
                    if !self.close_is_complete() {
                        return Ok(None);
                    }
                    generation2d_release_app_publication_authority(self.operation);
                    self.terminal_handoff = true;
                    if self.phase == Generation2dStoreInitializationPhase::RetireCancelled {
                        self.phase = Generation2dStoreInitializationPhase::Cancelled;
                        return JobOutcomeBorrow::admit_cancelled(cx);
                    }
                    self.phase = Generation2dStoreInitializationPhase::Fault;
                    return self.publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Fault, self.fault.as_deref().unwrap_or(GENERATION2D_INITIALIZER_DEFAULT_FAULT), cx);
                }
                Err(_) => self.fail(b"generation2d-store.initializer-close"),
            },
            Generation2dStoreInitializationPhase::Complete => return JobOutcomeBorrow::admit_complete(cx, None, None),
            Generation2dStoreInitializationPhase::Cancelled => return JobOutcomeBorrow::admit_cancelled(cx),
            Generation2dStoreInitializationPhase::Fault => return self.publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Fault, self.fault.as_deref().unwrap_or(GENERATION2D_INITIALIZER_DEFAULT_FAULT), cx),
        }
        cx.consume_fuel(1);
        Ok(None)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Complete => descriptor.complete(None, None),
            _ => self.publication.borrow_outcome(descriptor),
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<Generation2dSnapshot, Generation2dMutation>> {
        if self.phase != Generation2dStoreInitializationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, Generation2dStoreInitializationPhase::Cancelled | Generation2dStoreInitializationPhase::Fault) {
            self.phase = Generation2dStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::RetainedCloneStep;
        self.begin_close();
        let progress = self.close_turn(grant)?;
        if self.close_is_complete() {
            generation2d_release_app_publication_authority(self.operation);
            self.terminal_handoff = true;
            return Ok(RetainedCloneStep::Complete(progress));
        }
        Ok(RetainedCloneStep::Progress(progress))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}
