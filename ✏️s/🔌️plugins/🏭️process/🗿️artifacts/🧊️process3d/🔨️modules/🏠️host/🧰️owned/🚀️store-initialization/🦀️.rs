//! 🚀️ process3d store initialization: the document store initialization authority, its turn and phase machine, the lifecycle step helpers, the retirement value factories and the initialization job.

use super::*;

pub(super) struct Process3dStoreInitializationAuthority {
    actor: protocol::ActorId,
    pub(super) operation: semio_framework_job::OperationId,
    pub(super) generation: semio_framework_job::Generation,
    pub(super) base_revision: u64,
    pub(super) parent_revision: u64,
    history_items: usize,
    machine_growth: usize,
    envelope: std::mem::ManuallyDrop<Option<store::os_store::store_schema::ArtifactEnvelope<Process3dSnapshot, Process3dMutation>>>,
    owners: std::mem::ManuallyDrop<Option<store::os_store::document_store_owners::DocumentStoreOwners<Process3dSnapshot, Process3dMutation>>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<Process3dSnapshot>>>,
    pub(super) candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<Process3dSnapshot, Process3dMutation>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    released: std::mem::ManuallyDrop<Option<Process3dReleased>>,
    clone_cursor: std::mem::ManuallyDrop<Option<Process3dSnapshotCopyCursor>>,
    timeline_cursor: std::mem::ManuallyDrop<Option<Process3dTimelineCursor>>,
    census: std::mem::ManuallyDrop<Option<Process3dOwnerCensusCursor>>,
    candidate_disposer: std::mem::ManuallyDrop<Option<semio_framework_plugin::ArtifactDocumentStoreDisposer<Process3dSnapshot, Process3dMutation>>>,
    edit_index: store::ArtifactStoreInitializationEditIndex,
    pub(super) phase: Process3dStoreInitializationPhase,
    resume_phase: Option<Process3dStoreInitializationPhase>,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    publication: semio_framework_job_publication::RetainedJobPublication,
    pub(super) terminal_handoff: bool,
}

/// 🚦️ What one initializer turn owes its driver; terminal outcomes are lent through the retained publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dTurn {
    Yield,
    Complete,
    Cancelled,
    Fault,
}

impl Process3dStoreInitializationAuthority {
    fn turn(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Process3dTurn {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"process3d-store.initializer-stale-aba");
        }
        if (self.cancel_requested || cx.is_cancelled()) && !matches!(self.phase, Process3dStoreInitializationPhase::RetireCancelled | Process3dStoreInitializationPhase::Cancelled) {
            self.phase = Process3dStoreInitializationPhase::RetireCancelled;
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Process3dTurn::Yield;
        }
        if !matches!(self.phase, Process3dStoreInitializationPhase::RetireCancelled | Process3dStoreInitializationPhase::RetireFault | Process3dStoreInitializationPhase::Cancelled | Process3dStoreInitializationPhase::Fault | Process3dStoreInitializationPhase::Complete) {
            match self.pump_release(cx) {
                Ok(true) => {
                    cx.consume_fuel(1);
                    return Process3dTurn::Yield;
                }
                Ok(false) => {}
                Err(error) => {
                    self.fault = Some(error.into_message().into_bytes());
                    self.phase = Process3dStoreInitializationPhase::RetireFault;
                }
            }
        }
        match self.phase {
            Process3dStoreInitializationPhase::BindGenesis => {
                let envelope = self.envelope.as_ref().expect("retained initializer genesis");
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, envelope.vcs.genesis.facts().share_snapshot(), envelope.vcs.genesis.facts().digest(), self.actor.clone()));
                drop(self.census.take());
                self.phase = Process3dStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                cx.consume_fuel(1);
                return Process3dTurn::Yield;
            }
            Process3dStoreInitializationPhase::ValidateEnvelope => {
                let valid = self.envelope.as_ref().is_some_and(|envelope| envelope.schema == semio_s_artifact_process_process3d_model::snapshot::PROCESS_3D_SCHEMA && !envelope.id.is_empty() && envelope.id.len() <= PROCESS3D_OWNER_BYTES);
                self.phase = if valid { Process3dStoreInitializationPhase::ValidateEdit { index: 0 } } else { Process3dStoreInitializationPhase::RetireFault };
                if !valid {
                    self.fault = Some(process3d_fault_detail(b"process3d-store.initializer-envelope-invalid"));
                }
            }
            Process3dStoreInitializationPhase::ValidateEdit { index } => {
                let envelope = self.envelope.as_ref().expect("Process3d envelope retained");
                match self.edit_index.admit(&envelope.vcs.edits, index, usize::MAX) {
                    store::ArtifactStoreInitializationEditAdmission::Complete => self.phase = Process3dStoreInitializationPhase::CensusHistory { edit: 0, mutation: 0 },
                    store::ArtifactStoreInitializationEditAdmission::Admitted => self.phase = Process3dStoreInitializationPhase::ValidateEdit { index: index + 1 },
                    store::ArtifactStoreInitializationEditAdmission::Oversized | store::ArtifactStoreInitializationEditAdmission::Duplicate => self.fail(b"process3d-store.initializer-duplicate-edit"),
                }
            }
            Process3dStoreInitializationPhase::CensusHistory { edit, mutation } => self.turn_census_history(edit, mutation),
            Process3dStoreInitializationPhase::Census => {
                let source = &self.envelope.as_ref().expect("Process3d envelope retained").vcs.genesis.facts().snapshot();
                match self.census.as_mut().expect("Process3d census retained").step(source, cx) {
                    Ok(true) => self.phase = Process3dStoreInitializationPhase::BindGenesis,
                    Ok(false) => {}
                    Err(code) => self.fail(code.as_bytes()),
                }
            }
            Process3dStoreInitializationPhase::CloneInitial => self.turn_clone_initial(cx),
            Process3dStoreInitializationPhase::SeedHistory { edit, lane, index } => self.turn_seed_history(edit, lane, index),
            Process3dStoreInitializationPhase::FoldSupersessions { transition } => {
                let envelope = self.envelope.as_ref().expect("Process3d envelope remains retained while its supersessions fold");
                match self.runtime.as_mut().expect("Process3d runtime remains retained while its supersessions fold").fold_supersession_step(envelope, transition) {
                    Ok(true) => self.phase = Process3dStoreInitializationPhase::FoldSupersessions { transition: transition + 1 },
                    Ok(false) => self.phase = Process3dStoreInitializationPhase::FindApplied { position: 0 },
                    Err(error) => {
                        self.fault = Some(error.into_bytes());
                        self.phase = Process3dStoreInitializationPhase::RetireFault;
                    }
                }
            }
            Process3dStoreInitializationPhase::FindApplied { position } => self.turn_find_applied(position),
            Process3dStoreInitializationPhase::ApplyForward { position, edit, mutation } => self.turn_apply_forward(position, edit, mutation, cx),

            Process3dStoreInitializationPhase::ApplyTimeline { position, edit, mutation } => self.turn_apply_timeline(position, edit, mutation, cx),

            Process3dStoreInitializationPhase::CommitApplied { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Process3d applied retained");
                let runtime = self.runtime.as_mut().expect("Process3d runtime retained");
                match runtime.push_applied_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    Ok(()) => {

                        self.phase = Process3dStoreInitializationPhase::FindApplied { position: position + 1 };
                    }
                    Err(error) => {
                        self.fault = Some(error.into_bytes());
                        self.phase = Process3dStoreInitializationPhase::RetireFault;
                    }
                }
            }
            Process3dStoreInitializationPhase::FindRedo { position } => self.turn_find_redo(position),
            Process3dStoreInitializationPhase::CommitRedo { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Process3d redo edit retained");
                match self.runtime.as_mut().expect("Process3d runtime retained").push_redo_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    Ok(()) => self.phase = Process3dStoreInitializationPhase::FindRedo { position: position + 1 },
                    Err(error) => {
                        self.fault = Some(error.into_bytes());
                        self.phase = Process3dStoreInitializationPhase::RetireFault;
                    }
                }
            }
            Process3dStoreInitializationPhase::AdmitOwners => self.turn_admit_owners(cx),
            Process3dStoreInitializationPhase::ConstructOwners => self.turn_construct_owners(cx),
            Process3dStoreInitializationPhase::BuildCandidate => self.turn_build_candidate(cx),
            Process3dStoreInitializationPhase::RetireCancelled | Process3dStoreInitializationPhase::RetireFault => self.turn_retire_cancelled_or_retire_fault(cx),
            Process3dStoreInitializationPhase::Complete => {
                return Process3dTurn::Complete;
            }
            Process3dStoreInitializationPhase::Cancelled => return Process3dTurn::Cancelled,
            Process3dStoreInitializationPhase::Fault => return Process3dTurn::Fault,
        }
        cx.consume_fuel(1);
        Process3dTurn::Yield
    }

    /// 🧱️ Runs the `census history` phase of `turn`.
    fn turn_census_history(&mut self, edit: usize, mutation: usize) -> Process3dTurn {
        let envelope = self.envelope.as_ref().expect("Process3d envelope retained");
        let Some(entry) = envelope.vcs.edits.get(edit) else {
            let initial_machines = envelope.vcs.genesis.facts().snapshot().workshop.machines.len();
            let machine_capacity = match initial_machines.checked_add(self.machine_growth) {
                Some(value) if value <= PROCESS3D_MAXIMUM_DOMAIN_ITEMS => value,
                _ => {
                    self.fail(b"process3d-store.initializer-machine-growth");
                    return Process3dTurn::Yield;
                }
            };
            if machine_capacity.checked_mul(size_of::<WorkshopMachine>()).is_none_or(|bytes| bytes > PROCESS3D_MAXIMUM_DOMAIN_BYTES) {
                self.fail(b"process3d-store.initializer-machine-bytes");
                return Process3dTurn::Yield;
            }
            self.phase = Process3dStoreInitializationPhase::Census;
            return Process3dTurn::Yield;
        };
        if let Some(operation) = entry.forwards.get(mutation) {
            self.history_items = match self.history_items.checked_add(1) {
                Some(value) if value <= PROCESS3D_MAXIMUM_DOMAIN_ITEMS => value,
                _ => {
                    self.fail(b"process3d-store.initializer-history-capacity");
                    return Process3dTurn::Yield;
                }
            };
            if matches!(operation, Process3dMutation::CreateMachine(_)) {
                self.machine_growth = match self.machine_growth.checked_add(1) {
                    Some(value) => value,
                    None => {
                        self.fail(b"process3d-store.initializer-machine-growth-overflow");
                        return Process3dTurn::Yield;
                    }
                };
            }
            self.phase = Process3dStoreInitializationPhase::CensusHistory { edit, mutation: mutation + 1 };
        } else {
            self.phase = Process3dStoreInitializationPhase::CensusHistory { edit: edit + 1, mutation: 0 };
        }
    }

    /// 🪵️ Runs the `clone initial` phase of `turn`.
    fn turn_clone_initial(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Process3dTurn {
        let source = &self.envelope.as_ref().expect("Process3d envelope retained").vcs.genesis.facts().snapshot();
        let complete = match self.clone_cursor.as_mut().expect("Process3d clone retained").step(source, cx) {
            Ok(value) => value,
            Err(code) => {
                self.fail(code.as_bytes());
                false
            }
        };
        if complete {
            let initial = self.clone_cursor.as_mut().expect("Process3d clone retained").take().expect("Process3d clone handoff");
            drop(self.clone_cursor.take());
            *self.census = None;
            match self.runtime.as_mut().expect("retained initializer runtime").adopt_current_owned(initial, process3d_snapshot_value_factory()) {
                Ok(()) => self.phase = self.resume_phase.take().expect("retained mutation resume phase"),
                Err(initial) => {
                    *self.released = Some(Process3dReleased::Snapshot(initial));
                    self.fail(b"initializer-owned-workspace-adoption");
                }
            }
        }
        return Process3dTurn::Yield;
    }

    /// 🔩️ Runs the `seed history` phase of `turn`.
    fn turn_seed_history(&mut self, edit: usize, lane: u8, index: usize) -> Process3dTurn {
        let envelope = self.envelope.as_ref().expect("Process3d history retained");
        let Some(entry) = envelope.vcs.edits.get(edit) else {
            self.phase = Process3dStoreInitializationPhase::FoldSupersessions { transition: 0 };
            return Process3dTurn::Yield;
        };
        let runtime = self.runtime.as_mut().expect("Process3d runtime retained");
        match lane {
            0 => match runtime.seed_mutation(protocol::MutationId(process3d_copy_string(&entry.id).unwrap_or_default())) {
                Ok(()) => {
                    runtime.observe_sequence(entry.sequence_number);
                    self.phase = Process3dStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                }
                Err(error) => {
                    self.fault = Some(error.into_bytes());
                    self.phase = Process3dStoreInitializationPhase::RetireFault;
                }
            },
            1 if index < entry.forwards.len() => {
                let id =
                    entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.as_ref()).map_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)), |id| protocol::MutationId(process3d_copy_string(&id.0).unwrap_or_default()));
                match runtime.seed_edit_operation(&entry.id, id) {
                    Ok(()) => self.phase = Process3dStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 },
                    Err(error) => {
                        self.fault = Some(error.into_bytes());
                        self.phase = Process3dStoreInitializationPhase::RetireFault;
                    }
                }
            }
            1 => self.phase = Process3dStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
            2 if index < entry.mutation_meta.len() => {
                runtime.observe_timestamp(entry.mutation_meta[index].timestamp);
                self.phase = Process3dStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
            }
            _ => self.phase = Process3dStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
        }
    }

    /// ⚙️ Runs the `find applied` phase of `turn`.
    fn turn_find_applied(&mut self, position: usize) -> Process3dTurn {
        let Some(id) = self.applied_id(position) else {
            let checkpoint = self
                .envelope
                .as_ref()
                .and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.as_ref()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| &checkpoint.id)))
                .and_then(|id| process3d_copy_string(id).ok());
            self.runtime.as_mut().expect("Process3d runtime retained").set_current_checkpoint_id(checkpoint);
            self.phase = Process3dStoreInitializationPhase::FindRedo { position: 0 };
            return Process3dTurn::Yield;
        };
        let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
        let envelope = self.envelope.as_ref().expect("Process3d envelope retained");
        match envelope.vcs.edits.get(scan) {
            Some(edit) if edit.id == id => {
                self.phase = Process3dStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
            }
            Some(_) => self.fail(b"process3d-store.initializer-applied-missing"),
            None => self.fail(b"process3d-store.initializer-applied-missing"),
        }
    }

    /// 🧲️ Runs the `apply forward` phase of `turn`.
    fn turn_apply_forward(&mut self, position: usize, edit: usize, mutation: usize, cx: &mut semio_framework_job::StepContext<'_>) -> Process3dTurn {
        let needs_workspace = {
            let envelope = self.envelope.as_ref().expect("retained initializer envelope");
            let runtime = self.runtime.as_ref().expect("retained initializer runtime");
            envelope.vcs.edits.get(edit).and_then(|entry| runtime.effective_forward(entry, mutation, &envelope.schema)).is_some_and(|effective| effective.operation().is_some())
        };
        if needs_workspace && self.runtime.as_mut().expect("retained initializer runtime").current_mut().is_none() {
            self.resume_phase = Some(self.phase);
            let capacity = self.envelope.as_ref().expect("retained genesis").vcs.genesis.facts().snapshot().workshop.machines.len() + self.machine_growth;
            *self.clone_cursor = Some(Process3dSnapshotCopyCursor::new(capacity));
            self.phase = Process3dStoreInitializationPhase::CloneInitial;
            cx.consume_fuel(1);
            return Process3dTurn::Yield;
        }
        let operation = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).and_then(|entry| entry.forwards.get(mutation));
        let Some(_operation) = operation else {
            self.phase = Process3dStoreInitializationPhase::CommitApplied { position, edit };
            return Process3dTurn::Yield;
        };
        let envelope = self.envelope.as_ref().expect("Process3d envelope remains retained while its forwards fold");
        let entry = envelope.vcs.edits.get(edit).expect("Process3d applied edit remains retained");
        let effective = self.runtime.as_ref().and_then(|runtime| runtime.effective_forward(entry, mutation, &envelope.schema)).expect("Process3d applied forward remains retained");
        if effective.operation().is_none() {
            drop(effective);
            self.phase = Process3dStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
            return Process3dTurn::Yield;
        }
        if effective.operation().is_some_and(|operation| Process3dTimelineCursor::target(operation).is_some()) {
            *self.timeline_cursor = Some(Process3dTimelineCursor::new());
            self.phase = Process3dStoreInitializationPhase::ApplyTimeline { position, edit, mutation };
            cx.consume_fuel(1);
            return Process3dTurn::Yield;
        }
        let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("Process3d runtime current retained");
        let applied = effective.operation().map(|operation| process3d_apply_retained_mutation(current, operation));
        drop(effective);
        match applied {
            Some(Ok(retired)) => {
                *self.released = retired;
                self.phase = Process3dStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
            }
            None => self.phase = Process3dStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 },
            Some(Err(code)) => self.fail(code.as_bytes()),
        }
    }

    /// 🪛️ Runs the `apply timeline` phase of `turn`.
    fn turn_apply_timeline(&mut self, position: usize, edit: usize, mutation: usize, cx: &mut semio_framework_job::StepContext<'_>) -> Process3dTurn {
        let envelope = self.envelope.as_ref().expect("retained timeline envelope");
        let entry = envelope.vcs.edits.get(edit).expect("retained timeline edit");
        let effective = self.runtime.as_ref().and_then(|runtime| runtime.effective_forward(entry, mutation, &envelope.schema)).expect("retained timeline forward");
        let operation = effective.operation().expect("retained timeline operation");
        let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("retained timeline workspace");
        let advanced = self.timeline_cursor.as_mut().expect("retained timeline cursor").step(current, operation, cx);
        drop(effective);
        match advanced {
            Ok(true) => {
                *self.released = self.timeline_cursor.as_mut().expect("completed timeline cursor").commit(current);
                drop(self.timeline_cursor.take());
                self.phase = Process3dStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
            }
            Ok(false) => {}
            Err(code) => {
                self.fail(code.as_bytes());
                cx.consume_fuel(1);
                return Process3dTurn::Yield;
            }
        }
    }

    /// 🔧️ Runs the `find redo` phase of `turn`.
    fn turn_find_redo(&mut self, position: usize) -> Process3dTurn {
        let Some(id) = self.redo_id(position) else {
            self.edit_index.clear();
            self.phase = Process3dStoreInitializationPhase::AdmitOwners;
            return Process3dTurn::Yield;
        };
        let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
        match self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(scan)) {
            Some(edit) if edit.id == id => {
                self.phase = Process3dStoreInitializationPhase::CommitRedo { position, edit: scan };
            }
            Some(_) => self.fail(b"process3d-store.initializer-redo-missing"),
            None => self.fail(b"process3d-store.initializer-redo-missing"),
        }
    }

    /// 🔨️ Runs the `admit owners` phase of `turn`.
    fn turn_admit_owners(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Process3dTurn {
        match store::os_store::document_store_owners::bounded_artifact_store_owners::<Process3dSnapshot, Process3dMutation>(cx.retained_grant()) {
            Ok((owners, progress)) => {
                *self.owners = Some(owners);
                match cx.consume_retained(progress) {
                    Ok(()) => self.phase = Process3dStoreInitializationPhase::ConstructOwners,
                    Err(_) => self.fail(b"process3d-store.initializer-owners-receipt"),
                }
            }
            Err(refused) => {
                *self.owners = refused.owners;
                let _ = cx.consume_retained(refused.progress);
                self.fail(b"process3d-store.initializer-owners-admission");
            }
        }
    }

    /// 🪚️ Runs the `construct owners` phase of `turn`.
    fn turn_construct_owners(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Process3dTurn {
        let owners = self.owners.as_mut().expect("Process3d catalog retained while its constructor completes");
        if owners.constructor_is_complete() {
            self.phase = Process3dStoreInitializationPhase::BuildCandidate;
        } else {
            match owners.admit_constructor(cx.retained_grant()) {
                Ok(progress) => {
                    if cx.consume_retained(progress).is_err() {
                        self.fail(b"process3d-store.initializer-owners-receipt");
                    }
                }
                Err((error, progress)) => {
                    let _ = cx.consume_retained(progress);
                    self.fault = Some(error.into_message().into_bytes());
                    self.phase = Process3dStoreInitializationPhase::RetireFault;
                }
            }
        }
    }

    /// 🧰️ Runs the `build candidate` phase of `turn`.
    fn turn_build_candidate(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Process3dTurn {
        let authoritative = process3d_validate_publication_authority(self.operation, self.generation);
        let publication_fresh =
            cx.operation() == self.operation && cx.generation() == self.generation && authoritative == Ok((self.base_revision, self.parent_revision)) && self.base_revision == self.parent_revision && self.parent_revision == self.generation.0;
        let Some(candidate_generation) = self.parent_revision.checked_add(1) else {
            self.fail(b"process3d-store.initializer-generation-exhausted");
            return Process3dTurn::Yield;
        };
        if !publication_fresh {
            self.fail(b"process3d-store.initializer-parent-stale-aba");
            return Process3dTurn::Yield;
        }
        let envelope = self.envelope.take().expect("Process3d envelope retained until publication");
        let runtime = self.runtime.take().expect("Process3d runtime retained until publication");
        let owners = self.owners.take().expect("Process3d completed catalog retained until publication");
        *self.candidate = Some(store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, owners));
        self.phase = Process3dStoreInitializationPhase::Complete;
        return Process3dTurn::Complete;
    }

    /// 🧪️ Runs the `retire cancelled` or `retire fault` phase of `turn`.
    fn turn_retire_cancelled_or_retire_fault(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Process3dTurn {
        match self.close_original(cx.retained_grant()) {
            Ok(step) => {
                if let Err(error) = cx.consume_retained(step.progress()) {
                    self.fault = Some(error.into_message().into_bytes());
                }
                if !matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {
                    cx.consume_fuel(1);
                    return Process3dTurn::Yield;
                }
                process3d_release_app_publication_authority(self.operation);
                self.terminal_handoff = true;
                if self.phase == Process3dStoreInitializationPhase::RetireCancelled {
                    self.phase = Process3dStoreInitializationPhase::Cancelled;
                    return Process3dTurn::Cancelled;
                }
                self.phase = Process3dStoreInitializationPhase::Fault;
                self.fault.get_or_insert_with(|| process3d_fault_detail(b"process3d-store.initializer-fault"));
                return Process3dTurn::Fault;
            }
            Err(error) => {
                let _ = cx.consume_retained(error.retained_progress());
                self.fault = Some(error.into_message().into_bytes());
            }
        }
    }
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<Process3dSnapshot, Process3dMutation> for Process3dStoreInitializationAuthority {
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        match self.turn(cx) {
            Process3dTurn::Yield => semio_framework_job::JobOutcomeBorrow::admit_yield(cx),
            Process3dTurn::Complete => semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None),
            Process3dTurn::Cancelled => semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx),
            Process3dTurn::Fault => {
                let detail = self.fault.as_deref().unwrap_or(b"process3d-store.initializer-fault");
                self.publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Fault, detail, cx)
            }
        }
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        use semio_framework_job::JobOutcomeKind;
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete => descriptor.complete(None, None),
            JobOutcomeKind::Fault => self.publication.borrow_outcome(descriptor),
            _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d initializer lends only yield and terminal outcomes")),
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<Process3dSnapshot, Process3dMutation>> {
        if self.phase != Process3dStoreInitializationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, Process3dStoreInitializationPhase::Cancelled | Process3dStoreInitializationPhase::Fault) {
            self.phase = Process3dStoreInitializationPhase::RetireCancelled;
        }
    }

    fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        self.close_demands(body)
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.begin_close();
        let step = self.close_original(grant)?;
        if matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {
            process3d_release_app_publication_authority(self.operation);
            self.terminal_handoff = true;
        }
        Ok(step)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}

/// 🎟️ Funds one synchronous fixture turn from exactly the demand its own owner quoted.
#[cfg(any(test, feature = "testkit"))]
pub fn process3d_demand_grant(demand: semio_framework_value::RetirementDemand) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

/// ♻️ Superseded or abandoned original values the initializer hands to admitted retirement instead of dropping.
#[derive(semio_framework_value::RetireOwned)]
pub(super) enum Process3dReleased {
    String(String),
    Machine(WorkshopMachine),
    Capabilities(Vec<Capability>),
    Child(store::os_store::artifact_child::ArtifactChild<semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot>),
    Timeline(Process3dTimelineOwner),
    Snapshot(Process3dSnapshot),
    Json(semio_framework_pack_json::JsonWriteCursor<semio_framework_value::DslValue>),
}

/// 🧾️ Projects a plugin lifecycle receipt onto the retained-clone step the close ladder consumes.
fn process3d_lifecycle_step(step: semio_framework_plugin::PluginLifecycleStep) -> semio_framework_value::retained_clone::RetainedCloneStep {
    use semio_framework_plugin::PluginLifecycleStep;
    use semio_framework_value::retained_clone::RetainedCloneStep;
    match step {
        PluginLifecycleStep::Progress(progress) => RetainedCloneStep::Progress(progress),
        PluginLifecycleStep::Complete(progress) => RetainedCloneStep::Complete(progress),
        PluginLifecycleStep::AwaitingInput { .. } | PluginLifecycleStep::Blocked { .. } => RetainedCloneStep::Progress(Default::default()),
    }
}

/// 📏️ Adds the one structural level a nested original owner contributes.
fn process3d_nested(mut demand: semio_framework_value::RetirementDemand) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "Process3d retirement depth overflow"))?;
    Ok(demand)
}

/// 🏭️ Original owned-value issuer for Process3d snapshots.
pub(crate) fn process3d_snapshot_value_factory() -> std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<Process3dSnapshot>> {
    std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Process3dSnapshot>::default())
}

/// 🏭️ Original owned-value issuer for Process3d mutations.
pub(crate) fn process3d_mutation_value_factory() -> std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<Process3dMutation>> {
    std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Process3dMutation>::default())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dStoreInitializationPhase {
    BindGenesis,
    ValidateEnvelope,
    ValidateEdit { index: usize },
    CensusHistory { edit: usize, mutation: usize },
    Census,
    CloneInitial,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FoldSupersessions { transition: usize },
    FindApplied { position: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    ApplyTimeline { position: usize, edit: usize, mutation: usize },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize },
    CommitRedo { position: usize, edit: usize },
    AdmitOwners,
    ConstructOwners,
    BuildCandidate,
    Complete,
    RetireCancelled,
    RetireFault,
    Cancelled,
    Fault,
}

impl Process3dStoreInitializationAuthority {
    pub(super) fn new(envelope: store::os_store::store_schema::ArtifactEnvelope<Process3dSnapshot, Process3dMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Self {
        if process3d_validate_publication_authority(operation, generation).is_err() {
            let _ = process3d_admit_app_publication_authority(operation, generation);
        }
        let (base_revision, parent_revision) = process3d_validate_publication_authority(operation, generation).unwrap_or((u64::MAX, u64::MAX));
        Self {
            actor,
            operation,
            generation,
            base_revision,
            parent_revision,
            history_items: 0,
            machine_growth: 0,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            owners: std::mem::ManuallyDrop::new(None),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            released: std::mem::ManuallyDrop::new(None),
            clone_cursor: std::mem::ManuallyDrop::new(None),
            timeline_cursor: std::mem::ManuallyDrop::new(None),
            census: std::mem::ManuallyDrop::new(Some(Process3dOwnerCensusCursor::new())),
            candidate_disposer: std::mem::ManuallyDrop::new(None),
            edit_index: store::ArtifactStoreInitializationEditIndex::default(),
            resume_phase: None,
            phase: Process3dStoreInitializationPhase::ValidateEnvelope,
            cancel_requested: false,
            fault: None,
            publication: semio_framework_job_publication::RetainedJobPublication::new(),
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
        self.fault = Some(process3d_fault_detail(code));
        self.phase = Process3dStoreInitializationPhase::RetireFault;
    }

    /// ♻️ Spends one forward turn on a superseded original or its in-flight retirement.
    fn pump_release(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::RetainedCloneStep;
        let grant = cx.retained_grant();
        let step = if self.active.is_some() {
            store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, grant)?
        } else if self.released.is_some() {
            store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.released, &mut self.active, grant)?
        } else if let Some(runtime) = self.runtime.as_mut() {
            match runtime.settle_current_retirement_step(grant)? {
                RetainedCloneStep::Complete(_) => return Ok(false),
                step => step,
            }
        } else {
            return Ok(false);
        };
        cx.consume_retained(step.progress())?;
        Ok(true)
    }

    fn close_is_empty(&self) -> bool {
        self.envelope.is_none()
            && self.owners.is_none()
            && self.runtime.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.released.is_none()
            && self.clone_cursor.is_none()
            && self.timeline_cursor.is_none()
            && self.census.is_none()
            && self.candidate_disposer.is_none()
            && self.publication.terminal_is_empty()
    }

    /// 📏️ Quotes the one original owner the next close turn retires.
    pub(super) fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
        if let Some(active) = self.active.as_ref() {
            return process3d_nested(store::os_store::snapshot_read::artifact_retirement_box_demands(active, body)?);
        }
        if self.released.is_some() {
            return process3d_nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.released)?);
        }
        if let Some(candidate) = self.candidate.as_ref() {
            let Some(disposer) = self.candidate_disposer.as_ref() else {
                return Ok(RetirementDemand { copy_bytes: size_of::<Option<semio_framework_plugin::ArtifactDocumentStoreDisposer<Process3dSnapshot, Process3dMutation>>>(), depth: 1, ..Default::default() });
            };
            return process3d_nested(disposer.retirement_demands(candidate, body)?);
        }
        if let Some(runtime) = self.runtime.as_ref() {
            return process3d_nested(runtime.initialization_retirement_demands(body)?);
        }
        if let Some(cursor) = self.timeline_cursor.as_ref() {
            return process3d_nested(cursor.close_demands(body)?);
        }
        if let Some(cursor) = self.clone_cursor.as_ref() {
            return process3d_nested(cursor.close_demands(body)?);
        }
        if let Some(owners) = self.owners.as_ref() {
            if !owners.constructor_is_complete() {
                return process3d_nested(owners.constructor_demands(body)?);
            }
            if let Some(envelope) = self.envelope.as_ref() {
                let mut demand = process3d_nested(owners.uninstalled_envelope_retirement_demands(envelope))?;
                demand.copy_bytes = demand.copy_bytes.checked_add(size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>()).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "Process3d initializer original frame copy overflow"))?;
                return Ok(demand);
            }
            return if owners.uninstalled_owners_terminal_is_empty() {
                Ok(RetirementDemand { copy_bytes: size_of::<Option<store::os_store::document_store_owners::DocumentStoreOwners<Process3dSnapshot, Process3dMutation>>>(), depth: 1, ..Default::default() })
            } else {
                process3d_nested(owners.uninstalled_owners_demands(body)?)
            };
        }
        if self.envelope.is_some() {
            return Ok(RetirementDemand {
                copy_bytes: size_of::<Option<store::os_store::document_store_owners::DocumentStoreOwners<Process3dSnapshot, Process3dMutation>>>(),
                capacity_bytes: store::os_store::document_store_owners::bounded_artifact_store_owners_birth_bytes::<Process3dSnapshot, Process3dMutation>(),
                depth: 1,
                ..Default::default()
            });
        }
        if !self.publication.terminal_is_empty() {
            return process3d_nested(self.publication.retirement_demands()?);
        }
        Ok(Default::default())
    }

    /// ♻️ Retires exactly one original owner per granted turn.
    fn close_original(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::{admit_retained_clone_close, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
        use semio_framework_value::{ValueError, ValueRefusalKind};
        let empty = RetainedCloneProgress::default();
        *self.census = None;
        if self.close_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "Process3d initializer close exceeds original depth"));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.released.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.released, &mut self.active, child);
        }
        if self.candidate.is_some() {
            if self.candidate_disposer.is_none() {
                *self.candidate_disposer = Some(semio_framework_plugin::ArtifactDocumentStoreDisposer::new());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let candidate = self.candidate.as_mut().expect("observed original candidate store");
            let disposer = self.candidate_disposer.as_mut().expect("observed original candidate disposer");
            let step = process3d_lifecycle_step(disposer.close_step(candidate, child).map_err(|fault| ValueError::new(ValueRefusalKind::InvariantViolated, fault.message))?);
            if disposer.terminal_is_empty(candidate) {
                *self.candidate_disposer = None;
                drop(self.candidate.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let step = runtime.close_step(&process3d_snapshot_value_factory(), child)?;
            if runtime.terminal_is_empty() {
                drop(self.runtime.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(cursor) = self.timeline_cursor.as_mut() {
            let step = cursor.close_step(child)?;
            if cursor.terminal_is_empty() {
                drop(self.timeline_cursor.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(cursor) = self.clone_cursor.as_mut() {
            let step = cursor.close_step(child)?;
            if cursor.terminal_is_empty() {
                drop(self.clone_cursor.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope.is_some() {
            if self.owners.is_none() {
                let placement = demand.copy_bytes;
                let funded = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - placement, ..grant };
                return match store::os_store::document_store_owners::bounded_artifact_store_owners::<Process3dSnapshot, Process3dMutation>(funded) {
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
            let owners = self.owners.as_mut().expect("observed original catalog");
            if !owners.constructor_is_complete() {
                return owners.admit_constructor(child).map(RetainedCloneStep::Progress).map_err(|(error, receipt)| error.with_retained_progress(receipt));
            }
            let placement = size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>();
            let funded = RetainedCloneGrant { maximum_copy_bytes: child.maximum_copy_bytes - placement, ..child };
            let owners = self.owners.take().expect("observed original catalog");
            let envelope = self.envelope.take().expect("observed original envelope");
            return match owners.retire_envelope_uninstalled(envelope, funded) {
                Ok((owner, mut receipt)) => {
                    *self.active = Some(owner);
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
            let step = admit_retained_clone_close(child, step, owners.uninstalled_owners_terminal_is_empty(), "Process3d initializer uninstalled catalog")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if !self.publication.terminal_is_empty() {
            return self.publication.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff && self.close_is_empty()
    }
}

impl Drop for Process3dStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty_inner(), "Process3d initializer reached Drop before candidate handoff or terminal-empty close");
    }
}

pub fn process3d_document_store_initialization_job(
    envelope: store::os_store::store_schema::ArtifactEnvelope<Process3dSnapshot, Process3dMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    actor: protocol::ActorId,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<Process3dSnapshot, Process3dMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(Process3dStoreInitializationAuthority::new(envelope, operation, generation, actor)))
}
