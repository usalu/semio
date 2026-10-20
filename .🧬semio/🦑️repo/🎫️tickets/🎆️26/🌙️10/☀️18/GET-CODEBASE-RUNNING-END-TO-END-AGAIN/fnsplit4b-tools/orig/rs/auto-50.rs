//! 🏁️ Store initialization authority of the GIS map document: the authority state, its phases and the stepped initialization that applies the genesis envelope. See [`super`].

use super::*;

pub(super) struct GisMapStoreInitializationAuthority {
    pub(super) actor: protocol::ActorId,
    pub(super) operation: semio_framework_job::OperationId,
    pub(super) generation: semio_framework_job::Generation,
    pub(super) envelope: std::mem::ManuallyDrop<Option<store::os_store::store_schema::ArtifactEnvelope<GisMapSnapshot, GisMapMutation>>>,
    pub(super) runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<GisMapSnapshot>>>,
    pub(super) candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<GisMapSnapshot, GisMapMutation>>>,
    pub(super) owners: std::mem::ManuallyDrop<Option<store::os_store::document_store_owners::DocumentStoreOwners<GisMapSnapshot, GisMapMutation>>>,
    pub(super) pending: std::mem::ManuallyDrop<Option<GisMapSnapshot>>,
    pub(super) active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub(super) envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub(super) clone: std::mem::ManuallyDrop<Option<GisMapSnapshotCloneAuthority>>,
    edit_index: store::ArtifactStoreInitializationEditIndex,
    pub(super) phase: GisMapStoreInitializationPhase,
    resume_phase: Option<GisMapStoreInitializationPhase>,
    cancel_requested: bool,
    pub(super) fault: Option<Vec<u8>>,
    fault_detail: Option<Vec<u8>>,
    pub(super) publication: semio_framework_job_publication::RetainedJobPublication,
    pub(super) terminal_handoff: bool,
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<GisMapSnapshot, GisMapMutation> for GisMapStoreInitializationAuthority {
    fn retirement_demands(&self, maximum_copy_bytes: usize) -> Result<RetirementDemand, semio_framework_value::ValueError> {
        self.close_demands(maximum_copy_bytes)
    }

    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        match self.step_phase(cx) {
            GisMapInitializerTurn::Yield => semio_framework_job::JobOutcomeBorrow::admit_yield(cx),
            GisMapInitializerTurn::Complete => semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None),
            GisMapInitializerTurn::Cancelled => semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx),
            GisMapInitializerTurn::Fault => {
                let detail = self.fault_detail.as_deref().unwrap_or(b"gis-map-store.initializer-fault");
                self.publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Fault, detail, cx)
            }
        }
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Complete => descriptor.complete(None, None),
            semio_framework_job::JobOutcomeKind::Fault => self.publication.borrow_outcome(descriptor),
            _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "GIS store initializer lends only terminal and yield outcomes")),
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, GisMapStoreInitializationPhase::Cancelled | GisMapStoreInitializationPhase::Fault) {
            self.phase = GisMapStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        self.begin_close();
        let step = self.close_original(grant)?;
        if self.close_is_terminal() {
            self.terminal_handoff = true;
            return Ok(RetainedCloneStep::Complete(step.progress()));
        }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<GisMapSnapshot, GisMapMutation>> {
        if self.phase != GisMapStoreInitializationPhase::Complete || self.terminal_handoff {
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

enum GisMapInitializerTurn {
    Yield,
    Complete,
    Cancelled,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GisMapStoreInitializationPhase {
    AdmitOwners,
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
    BuildCandidate,
    RetireCancelled,
    RetireFault,
    Complete,
    Cancelled,
    Fault,
}

impl GisMapStoreInitializationAuthority {
    pub(super) fn new(envelope: store::os_store::store_schema::ArtifactEnvelope<GisMapSnapshot, GisMapMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Self {
        Self {
            actor,
            operation,
            generation,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            owners: std::mem::ManuallyDrop::new(None),
            pending: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            clone: std::mem::ManuallyDrop::new(None),
            edit_index: store::ArtifactStoreInitializationEditIndex::default(),
            resume_phase: None,
            phase: GisMapStoreInitializationPhase::AdmitOwners,
            cancel_requested: false,
            fault: None,
            fault_detail: None,
            publication: semio_framework_job_publication::RetainedJobPublication::new(),
            terminal_handoff: false,
        }
    }

    fn step_phase(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> GisMapInitializerTurn {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"gis-map-store.initializer-stale-authority");
        }
        if self.cancel_requested && !matches!(self.phase, GisMapStoreInitializationPhase::RetireCancelled | GisMapStoreInitializationPhase::Cancelled) {
            self.phase = GisMapStoreInitializationPhase::RetireCancelled;
        }
        if self.active.is_some() || self.pending.is_some() {
            return match self.drain_displaced(cx.retained_grant()) {
                Ok(step) => {
                    if cx.consume_retained(step.progress()).is_ok() {
                        cx.consume_fuel(1);
                    }
                    GisMapInitializerTurn::Yield
                }
                Err(error) => {
                    self.fault = Some(error.into_message().into_bytes());
                    self.phase = GisMapStoreInitializationPhase::RetireFault;
                    GisMapInitializerTurn::Yield
                }
            };
        }
        if !matches!(self.phase, GisMapStoreInitializationPhase::AdmitOwners | GisMapStoreInitializationPhase::RetireCancelled | GisMapStoreInitializationPhase::RetireFault | GisMapStoreInitializationPhase::Cancelled | GisMapStoreInitializationPhase::Fault | GisMapStoreInitializationPhase::Complete) {
            if let Some(runtime) = self.runtime.as_mut() {
                match runtime.settle_current_retirement_step(cx.retained_grant()) {
                    Ok(RetainedCloneStep::Complete(_)) => {}
                    Ok(step) => {
                        if cx.consume_retained(step.progress()).is_ok() {
                            cx.consume_fuel(1);
                        }
                        return GisMapInitializerTurn::Yield;
                    }
                    Err(error) => {
                        self.fault = Some(error.into_message().into_bytes());
                        self.phase = GisMapStoreInitializationPhase::RetireFault;
                    }
                }
            }
        }
        match self.phase {
            GisMapStoreInitializationPhase::AdmitOwners => {
                let grant = cx.retained_grant();
                let admitted = if self.owners.is_none() {
                    match store::os_store::document_store_owners::bounded_artifact_store_owners::<GisMapSnapshot, GisMapMutation>(grant) {
                        Ok((owners, progress)) => {
                            *self.owners = Some(owners);
                            Ok(progress)
                        }
                        Err(refused) => {
                            *self.owners = refused.owners;
                            Err(refused.error.with_retained_progress(refused.progress))
                        }
                    }
                } else if !self.owners.as_ref().is_some_and(|owners| owners.constructor_is_complete()) {
                    self.owners.as_mut().expect("retained initializer owner catalog").admit_constructor(grant).map_err(|(error, progress)| error.with_retained_progress(progress))
                } else {
                    Ok(RetainedCloneProgress::default())
                };
                match admitted {
                    Ok(progress) => {
                        if cx.consume_retained(progress).is_err() {
                            return GisMapInitializerTurn::Yield;
                        }
                        if self.owners.as_ref().is_some_and(|owners| owners.constructor_is_complete()) {
                            self.phase = GisMapStoreInitializationPhase::ValidateEnvelope;
                        }
                    }
                    Err(error) => {
                        self.fault = Some(error.into_message().into_bytes());
                        self.phase = GisMapStoreInitializationPhase::RetireFault;
                    }
                }
                cx.consume_fuel(1);
                GisMapInitializerTurn::Yield
            }
            GisMapStoreInitializationPhase::BindGenesis => {
                let envelope = self.envelope.as_ref().expect("retained initializer genesis");
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, envelope.vcs.genesis.facts().share_snapshot(), envelope.vcs.genesis.facts().digest(), self.actor.clone()));
                self.phase = GisMapStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                cx.consume_fuel(1);
                GisMapInitializerTurn::Yield
            }
            GisMapStoreInitializationPhase::ValidateEnvelope => {
                let Some(envelope) = self.envelope.as_ref() else {
                    self.fail(b"gis-map-store.initializer-envelope-missing");
                    return GisMapInitializerTurn::Yield;
                };
                if envelope.schema != crate::GIS_MAP_SCHEMA || envelope.id.is_empty() || envelope.id.len() > GIS_MAP_OWNED_FIELD_BYTES {
                    self.fail(b"gis-map-store.initializer-envelope-invalid");
                } else {
                    self.phase = GisMapStoreInitializationPhase::ValidateEdit { index: 0 };
                }
                cx.consume_fuel(1);
                GisMapInitializerTurn::Yield
            }
            GisMapStoreInitializationPhase::ValidateEdit { index } => {
                let envelope = self.envelope.as_ref().expect("validated GIS envelope remains retained");
                match self.edit_index.admit(&envelope.vcs.edits, index, GIS_MAP_OWNED_FIELD_BYTES) {
                    store::ArtifactStoreInitializationEditAdmission::Complete => self.phase = GisMapStoreInitializationPhase::BindGenesis,
                    store::ArtifactStoreInitializationEditAdmission::Admitted => self.phase = GisMapStoreInitializationPhase::ValidateEdit { index: index + 1 },
                    store::ArtifactStoreInitializationEditAdmission::Oversized | store::ArtifactStoreInitializationEditAdmission::Duplicate => self.fail(b"gis-map-store.initializer-duplicate-or-hostile-edit"),
                }
                cx.consume_fuel(1);
                GisMapInitializerTurn::Yield
            }
            GisMapStoreInitializationPhase::CloneInitial => {
                let source = &self.envelope.as_ref().expect("GIS envelope remains retained during initial clone").vcs.genesis.facts().snapshot();
                let clone = self.clone.as_mut().expect("GIS initial clone authority remains retained");
                let complete = match clone.step(source, cx) {
                    Ok(complete) => complete,
                    Err(code) => {
                        self.fail(code.as_bytes());
                        return GisMapInitializerTurn::Yield;
                    }
                };
                if complete {
                    let initial = clone.take_value().expect("GIS initial snapshot was built one semantic item at a time");
                    drop(self.clone.take());
                    match self.runtime.as_mut().expect("retained initializer runtime").adopt_current_owned(initial, std::sync::Arc::new(GisMapSnapshotRetirementFactory) as std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<GisMapSnapshot>>) {
                        Ok(()) => self.phase = self.resume_phase.take().expect("retained mutation resume phase"),
                        Err(initial) => {
                            *self.pending = Some(initial);
                            self.fail(b"initializer-owned-workspace-adoption");
                        }
                    }
                }
                GisMapInitializerTurn::Yield
            }
            GisMapStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("GIS envelope remains retained while causal history is seeded");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = GisMapStoreInitializationPhase::FoldSupersessions { transition: 0 };
                    return GisMapInitializerTurn::Yield;
                };
                let runtime = self.runtime.as_mut().expect("GIS runtime remains retained while history is seeded");
                match lane {
                    0 => {
                        if let Err(error) = runtime.seed_mutation(protocol::MutationId(entry.id.clone())) {
                            self.fault = Some(error.into_bytes());
                            self.phase = GisMapStoreInitializationPhase::RetireFault;
                        } else {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = GisMapStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                    }
                    1 if index < entry.forwards.len() => {
                        let id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| entry.forwards[index].mutation_id()).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                        if let Err(error) = runtime.seed_edit_operation(&entry.id, id) {
                            self.fault = Some(error.into_bytes());
                            self.phase = GisMapStoreInitializationPhase::RetireFault;
                        } else {
                            self.phase = GisMapStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                        }
                    }
                    1 => self.phase = GisMapStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp);
                        self.phase = GisMapStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = GisMapStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
                cx.consume_fuel(1);
                GisMapInitializerTurn::Yield
            }
            GisMapStoreInitializationPhase::FoldSupersessions { transition } => {
                let envelope = self.envelope.as_ref().expect("GIS envelope remains retained while its supersessions fold");
                match self.runtime.as_mut().expect("GIS runtime remains retained while its supersessions fold").fold_supersession_step(envelope, transition) {
                    Ok(true) => self.phase = GisMapStoreInitializationPhase::FoldSupersessions { transition: transition + 1 },
                    Ok(false) => self.phase = GisMapStoreInitializationPhase::FindApplied { position: 0 },
                    Err(error) => {
                        self.fault = Some(error.into_bytes());
                        self.phase = GisMapStoreInitializationPhase::RetireFault;
                    }
                }
                cx.consume_fuel(1);
                GisMapInitializerTurn::Yield
            }
            GisMapStoreInitializationPhase::FindApplied { position } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
                    self.runtime.as_mut().expect("GIS runtime remains retained").set_current_checkpoint_id(checkpoint);
                    self.phase = GisMapStoreInitializationPhase::FindRedo { position: 0 };
                    return GisMapInitializerTurn::Yield;
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("GIS envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"gis-map-store.initializer-applied-edit-missing");
                    return GisMapInitializerTurn::Yield;
                };
                if edit.id == id {
                    self.phase = GisMapStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                } else {
                    self.fail(b"gis-map-store.initializer-applied-edit-missing");
                }
                cx.consume_fuel(1);
                GisMapInitializerTurn::Yield
            }
            GisMapStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let needs_workspace = {
                    let envelope = self.envelope.as_ref().expect("retained initializer envelope");
                    let runtime = self.runtime.as_ref().expect("retained initializer runtime");
                    envelope.vcs.edits.get(edit).and_then(|entry| runtime.effective_forward(entry, mutation, &envelope.schema)).is_some_and(|effective| effective.operation().is_some())
                };
                if needs_workspace && self.runtime.as_mut().expect("retained initializer runtime").current_mut().is_none() {
                    self.resume_phase = Some(self.phase);
                    *self.clone = Some(GisMapSnapshotCloneAuthority::new());
                    self.phase = GisMapStoreInitializationPhase::CloneInitial;
                    cx.consume_fuel(1);
                    return GisMapInitializerTurn::Yield;
                }
                let envelope = self.envelope.as_ref().expect("GIS envelope remains retained while its forwards fold");
                let entry = envelope.vcs.edits.get(edit).expect("GIS applied edit remains retained");
                match self.runtime.as_mut().expect("GIS runtime remains retained while its forwards fold").fold_forward(entry, mutation, &envelope.schema, GIS_MAP_OWNED_FIELD_BYTES) {
                    Ok(store::ArtifactStoreInitializationForward::Folded { displaced, fuel }) => {
                        if let Some(previous) = displaced {
                            *self.pending = Some(previous);
                        }
                        self.phase = GisMapStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(fuel as u64);
                    }
                    Ok(store::ArtifactStoreInitializationForward::Exhausted) => self.phase = GisMapStoreInitializationPhase::CommitApplied { position, edit },
                    Err(_) => self.fail(b"gis-map-store.initializer-forward-encoding"),
                }
                GisMapInitializerTurn::Yield
            }
            GisMapStoreInitializationPhase::CommitApplied { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("GIS applied edit remains retained");

                let runtime = self.runtime.as_mut().expect("GIS runtime remains retained");
                if let Err(error) = runtime.push_applied_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fault = Some(error.into_bytes());
                    self.phase = GisMapStoreInitializationPhase::RetireFault;
                } else {

                    self.phase = GisMapStoreInitializationPhase::FindApplied { position: position + 1 };
                }
                cx.consume_fuel(1);
                GisMapInitializerTurn::Yield
            }
            GisMapStoreInitializationPhase::FindRedo { position } => {
                let Some(id) = self.redo_id(position) else {
                    self.edit_index.clear();
                    self.phase = GisMapStoreInitializationPhase::BuildCandidate;
                    return GisMapInitializerTurn::Yield;
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("GIS envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"gis-map-store.initializer-redo-edit-missing");
                    return GisMapInitializerTurn::Yield;
                };
                if edit.id == id {
                    self.phase = GisMapStoreInitializationPhase::CommitRedo { position, edit: scan };
                } else {
                    self.fail(b"gis-map-store.initializer-redo-edit-missing");
                }
                cx.consume_fuel(1);
                GisMapInitializerTurn::Yield
            }
            GisMapStoreInitializationPhase::CommitRedo { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("GIS redo edit remains retained");
                if let Err(error) = self.runtime.as_mut().expect("GIS runtime remains retained").push_redo_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fault = Some(error.into_bytes());
                    self.phase = GisMapStoreInitializationPhase::RetireFault;
                } else {
                    self.phase = GisMapStoreInitializationPhase::FindRedo { position: position + 1 };
                }
                cx.consume_fuel(1);
                GisMapInitializerTurn::Yield
            }
            GisMapStoreInitializationPhase::BuildCandidate => {
                let Some(candidate_generation) = self.generation.0.checked_add(1) else {
                    self.fail(b"gis-map-store.initializer-generation-exhausted");
                    return GisMapInitializerTurn::Yield;
                };
                let envelope = self.envelope.take().expect("GIS envelope remains retained until atomic store construction");
                let runtime = self.runtime.take().expect("GIS runtime remains retained until atomic store construction");
                let Some(owners) = self.owners.take() else {
                    self.fail(b"gis-map-store.initializer-owners-missing");
                    *self.envelope = Some(envelope);
                    *self.runtime = Some(runtime);
                    return GisMapInitializerTurn::Yield;
                };
                let candidate = store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, owners);
                *self.candidate = Some(candidate);
                self.phase = GisMapStoreInitializationPhase::Complete;
                GisMapInitializerTurn::Complete
            }
            GisMapStoreInitializationPhase::RetireCancelled | GisMapStoreInitializationPhase::RetireFault => match self.close_original(cx.retained_grant()) {
                Ok(step) => {
                    if cx.consume_retained(step.progress()).is_err() {
                        return GisMapInitializerTurn::Yield;
                    }
                    cx.consume_fuel(1);
                    if !self.close_is_terminal() {
                        return GisMapInitializerTurn::Yield;
                    }
                    self.terminal_handoff = true;
                    if self.phase == GisMapStoreInitializationPhase::RetireCancelled {
                        self.phase = GisMapStoreInitializationPhase::Cancelled;
                        GisMapInitializerTurn::Cancelled
                    } else {
                        self.phase = GisMapStoreInitializationPhase::Fault;
                        self.fault_detail = Some(self.fault.take().unwrap_or_else(|| b"gis-map-store.initializer-fault".to_vec()));
                        GisMapInitializerTurn::Fault
                    }
                }
                Err(error) => {
                    self.fault = Some(error.into_message().into_bytes());
                    GisMapInitializerTurn::Yield
                }
            },
            GisMapStoreInitializationPhase::Complete => GisMapInitializerTurn::Complete,
            GisMapStoreInitializationPhase::Cancelled => GisMapInitializerTurn::Cancelled,
            GisMapStoreInitializationPhase::Fault => {
                GisMapInitializerTurn::Fault
            }
        }
    }
}
