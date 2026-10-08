//! 🏠️ Artifact document-store and publication authorities.

use crate::standards::v1::subsets::any::schema::mutations::GisMapMutation;
use crate::{GisMapSnapshot, MapFeature};
use protocol::{Mutation, MutationDiff, OpBinary};
pub fn gis_map_document_store_owners() -> store::DocumentStoreOwners<GisMapSnapshot, GisMapMutation> {
    store::DocumentStoreOwners::new(
        std::sync::Arc::new(GisMapSnapshotRetirementFactory),
        std::sync::Arc::new(GisMapSnapshotRetirementFactory),
        std::sync::Arc::new(GisMapMutationRetirementFactory),
        Box::new(store::ArtifactStoreCursorDisposer::<GisMapSnapshot, GisMapMutation>::new()),
    )
}

struct GisMapStoreInitializationAuthority {
    actor: protocol::ActorId,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    envelope: std::mem::ManuallyDrop<Option<store::ArtifactEnvelope<GisMapSnapshot, GisMapMutation>>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<GisMapSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<GisMapSnapshot, GisMapMutation>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    clone: std::mem::ManuallyDrop<Option<GisMapSnapshotCloneAuthority>>,
    edit_index: store::ArtifactStoreInitializationEditIndex,
    phase: GisMapStoreInitializationPhase,
    resume_phase: Option<GisMapStoreInitializationPhase>,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    terminal_handoff: bool,
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<GisMapSnapshot, GisMapMutation> for GisMapStoreInitializationAuthority {
    fn next_close_byte_demand(&self) -> usize {
        self.active.as_ref().or(self.envelope_retirement.as_ref()).map_or(GIS_MAP_OWNED_FIELD_BYTES, |owner| owner.next_close_byte_demand())
    }

    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"gis-map-store.initializer-stale-authority");
        }
        if self.cancel_requested && !matches!(self.phase, GisMapStoreInitializationPhase::RetireCancelled | GisMapStoreInitializationPhase::Cancelled) {
            self.phase = GisMapStoreInitializationPhase::RetireCancelled;
        }
        if let Err(error) = self.pump_active() {
            self.fault = Some(error.into_message().into_bytes());
            self.phase = GisMapStoreInitializationPhase::RetireFault;
        } else if self.active.is_some() {
            return semio_framework_job::StepOutcome::Yield;
        }
        if !matches!(self.phase, GisMapStoreInitializationPhase::RetireCancelled | GisMapStoreInitializationPhase::RetireFault | GisMapStoreInitializationPhase::Cancelled | GisMapStoreInitializationPhase::Fault | GisMapStoreInitializationPhase::Complete) {
            if let Some(runtime) = self.runtime.as_mut() {
                match runtime.settle_current_retirement_step(1, GIS_MAP_OWNED_FIELD_BYTES) {
                    Ok(store::SnapshotRetirementStep::Complete) => {}
                    Ok(_) => { cx.consume_fuel(1); return semio_framework_job::StepOutcome::Yield; }
                    Err(error) => { self.fault = Some(error.into_message().into_bytes()); self.phase = GisMapStoreInitializationPhase::RetireFault; }
                }
            }
        }
        match self.phase {
            GisMapStoreInitializationPhase::BindGenesis => {
                let envelope = self.envelope.as_ref().expect("retained initializer genesis");
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, envelope.vcs.genesis.share_snapshot(), envelope.vcs.genesis.digest(), self.actor.clone()));
                self.phase = GisMapStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::ValidateEnvelope => {
                let Some(envelope) = self.envelope.as_ref() else {
                    self.fail(b"gis-map-store.initializer-envelope-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if envelope.schema != crate::GIS_MAP_SCHEMA || envelope.id.is_empty() || envelope.id.len() > GIS_MAP_OWNED_FIELD_BYTES {
                    self.fail(b"gis-map-store.initializer-envelope-invalid");
                } else {
                    self.phase = GisMapStoreInitializationPhase::ValidateEdit { index: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::ValidateEdit { index } => {
                let envelope = self.envelope.as_ref().expect("validated GIS envelope remains retained");
                match self.edit_index.admit(&envelope.vcs.edits, index, GIS_MAP_OWNED_FIELD_BYTES) {
                    store::ArtifactStoreInitializationEditAdmission::Complete => self.phase = GisMapStoreInitializationPhase::BindGenesis,
                    store::ArtifactStoreInitializationEditAdmission::Admitted => self.phase = GisMapStoreInitializationPhase::ValidateEdit { index: index + 1 },
                    store::ArtifactStoreInitializationEditAdmission::Oversized | store::ArtifactStoreInitializationEditAdmission::Duplicate => self.fail(b"gis-map-store.initializer-duplicate-or-hostile-edit"),
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::CloneInitial => {
                let source = &self.envelope.as_ref().expect("GIS envelope remains retained during initial clone").vcs.genesis.snapshot();
                let clone = self.clone.as_mut().expect("GIS initial clone authority remains retained");
                let complete = match clone.step(source, cx) {
                    Ok(complete) => complete,
                    Err(code) => {
                        self.fail(code.as_bytes());
                        return semio_framework_job::StepOutcome::Yield;
                    }
                };
                if complete {
                    let initial = clone.take_value().expect("GIS initial snapshot was built one semantic item at a time");
                    drop(self.clone.take());
                    match self.runtime.as_mut().expect("retained initializer runtime").adopt_current_owned(initial, std::sync::Arc::new(GisMapSnapshotRetirementFactory)) {
                        Ok(()) => self.phase = self.resume_phase.take().expect("retained mutation resume phase"),
                        Err(initial) => {
                            *self.active = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&GisMapSnapshotRetirementFactory, initial));
                            self.fail(b"initializer-owned-workspace-adoption");
                        }
                    }
                }
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("GIS envelope remains retained while causal history is seeded");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = GisMapStoreInitializationPhase::FoldSupersessions { transition: 0 };
                    return semio_framework_job::StepOutcome::Yield;
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
                semio_framework_job::StepOutcome::Yield
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
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::FindApplied { position } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
                    self.runtime.as_mut().expect("GIS runtime remains retained").set_current_checkpoint_id(checkpoint);
                    self.phase = GisMapStoreInitializationPhase::FindRedo { position: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("GIS envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"gis-map-store.initializer-applied-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    self.phase = GisMapStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                } else {
                    self.fail(b"gis-map-store.initializer-applied-edit-missing");
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
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
                    return semio_framework_job::StepOutcome::Yield;
                }
                let envelope = self.envelope.as_ref().expect("GIS envelope remains retained while its forwards fold");
                let entry = envelope.vcs.edits.get(edit).expect("GIS applied edit remains retained");
                match self.runtime.as_mut().expect("GIS runtime remains retained while its forwards fold").fold_forward(entry, mutation, &envelope.schema, GIS_MAP_OWNED_FIELD_BYTES) {
                    Ok(store::ArtifactStoreInitializationForward::Folded { displaced, fuel }) => {
                        if let Some(previous) = displaced {
                            *self.active = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&GisMapSnapshotRetirementFactory, previous));
                        }
                        self.phase = GisMapStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(fuel as u64);
                    }
                    Ok(store::ArtifactStoreInitializationForward::Exhausted) => self.phase = GisMapStoreInitializationPhase::CommitApplied { position, edit },
                    Err(_) => self.fail(b"gis-map-store.initializer-forward-encoding"),
                }
                semio_framework_job::StepOutcome::Yield
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
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::FindRedo { position } => {
                let Some(id) = self.redo_id(position) else {
                    self.edit_index.clear();
                    self.phase = GisMapStoreInitializationPhase::BuildCandidate;
                    return semio_framework_job::StepOutcome::Yield;
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("GIS envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"gis-map-store.initializer-redo-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    self.phase = GisMapStoreInitializationPhase::CommitRedo { position, edit: scan };
                } else {
                    self.fail(b"gis-map-store.initializer-redo-edit-missing");
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
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
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::BuildCandidate => {
                let Some(candidate_generation) = self.generation.0.checked_add(1) else {
                    self.fail(b"gis-map-store.initializer-generation-exhausted");
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.take().expect("GIS envelope remains retained until atomic store construction");
                let runtime = self.runtime.take().expect("GIS runtime remains retained until atomic store construction");
                let candidate = store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, gis_map_document_store_owners());
                *self.candidate = Some(candidate);
                self.phase = GisMapStoreInitializationPhase::Complete;
                semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                })
            }
            GisMapStoreInitializationPhase::RetireCancelled | GisMapStoreInitializationPhase::RetireFault => match self.pump_terminal_retirement() {
                Ok(false) => semio_framework_job::StepOutcome::Yield,
                Ok(true) => {
                    self.terminal_handoff = true;
                    if self.phase == GisMapStoreInitializationPhase::RetireCancelled {
                        self.phase = GisMapStoreInitializationPhase::Cancelled;
                        semio_framework_job::StepOutcome::Cancelled
                    } else {
                        self.phase = GisMapStoreInitializationPhase::Fault;
                        let fault = self.fault.take().unwrap_or_else(|| b"gis-map-store.initializer-fault".to_vec());
                        let detail = cx.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, &fault).unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                        semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail })
                    }
                }
                Err(error) => {
                    self.fault = Some(error.into_message().into_bytes());
                    semio_framework_job::StepOutcome::Yield
                }
            },
            GisMapStoreInitializationPhase::Complete => semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
            }),
            GisMapStoreInitializationPhase::Cancelled => semio_framework_job::StepOutcome::Cancelled,
            GisMapStoreInitializationPhase::Fault => {
                let fault = self.fault.as_deref().unwrap_or(b"gis-map-store.initializer-fault");
                let detail = cx.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, fault).unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail })
            }
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

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, semio_framework_diagnostic::Fault> {
        self.begin_close();
        if maximum_items == 0 || maximum_bytes < GIS_MAP_OWNED_FIELD_BYTES {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        match self.pump_terminal_retirement() {
            Ok(false) => Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }),
            Ok(true) => {
                self.terminal_handoff = true;
                Ok(semio_framework_plugin::PluginCloseStep::Complete)
            }
            Err(error) => Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Plugin, semio_framework_diagnostic::FaultCode::new("artifact-store.initializer-close"), format!("GIS Map initializer close failed: {error}"))),
        }
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

pub(crate) const GIS_MAP_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct GisMapSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<GisMapSnapshot> for GisMapSnapshotRetirementFactory {
    fn retire_owned(&self, value: GisMapSnapshot) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(GisMapOwnedRetirement::new(GisMapRetirementOwner::Snapshot(value)))
    }
}

impl store::SnapshotRetirementFactory<GisMapSnapshot> for GisMapSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<GisMapSnapshot>) -> usize { std::mem::size_of::<GisMapSnapshotRootRetirement>() }

    fn retire(&self, snapshot: std::sync::Arc<GisMapSnapshot>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(GisMapSnapshotRootRetirement { owner: std::mem::ManuallyDrop::new(Some(snapshot)), retirement: std::mem::ManuallyDrop::new(None) })
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct GisMapMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<GisMapMutation> for GisMapMutationRetirementFactory {
    fn retire_owned(&self, value: GisMapMutation) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(GisMapOwnedRetirement::new(GisMapRetirementOwner::Mutation(value)))
    }
}

struct GisMapSnapshotCloneAuthority {
    value: std::mem::ManuallyDrop<Option<GisMapSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    phase: u8,
    index: usize,
    terminal: bool,
}

impl GisMapSnapshotCloneAuthority {
    fn empty_child<S>() -> store::ArtifactChild<S> {
        store::ArtifactChild::new(String::new(), semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } })
    }

    fn new() -> Self {
        Self {
            value: std::mem::ManuallyDrop::new(Some(GisMapSnapshot { positions: Vec::new(), routes: Vec::new(), regions: Vec::new(), drawing: Self::empty_child(), image: None, value: Self::empty_child() })),
            retirement: std::mem::ManuallyDrop::new(None),
            phase: 0,
            index: 0,
            terminal: false,
        }
    }

    fn clone_string(source: &str) -> Result<String, &'static str> {
        if source.len() > GIS_MAP_OWNED_FIELD_BYTES {
            return Err("gis-map-store.initializer-field-too-large");
        }
        let mut value = String::new();
        value.try_reserve_exact(source.len()).map_err(|_| "gis-map-store.initializer-string-admission")?;
        value.push_str(source);
        Ok(value)
    }

    fn clone_feature(source: &MapFeature) -> Result<MapFeature, &'static str> {
        let encoded = semio_framework_pack_json::to_json_string(source).into_bytes();
        if encoded.len() > GIS_MAP_OWNED_FIELD_BYTES {
            return Err("gis-map-store.initializer-feature-too-large");
        }
        Ok(source.clone())
    }

    fn copy_child_field<'a, S>(target: &mut store::ArtifactChild<S>, source: &'a store::ArtifactChild<S>, phase: u8, base: u8) -> Result<&'a [u8], &'static str> {
        let observed = match phase - base {
            0 => {
                target.child_id = Self::clone_string(&source.child_id)?;
                source.child_id.as_bytes()
            }
            1 => {
                target.target.artifact_id = Self::clone_string(&source.target.artifact_id)?;
                source.target.artifact_id.as_bytes()
            }
            2 => {
                target.target.dialect.artifact_kind = Self::clone_string(&source.target.dialect.artifact_kind)?;
                source.target.dialect.artifact_kind.as_bytes()
            }
            3 => {
                target.target.dialect.standard = Self::clone_string(&source.target.dialect.standard)?;
                source.target.dialect.standard.as_bytes()
            }
            4 => {
                target.target.dialect.subset = Self::clone_string(&source.target.dialect.subset)?;
                source.target.dialect.subset.as_bytes()
            }
            _ => return Err("gis-map-store.initializer-child-phase"),
        };
        Ok(observed)
    }

    fn step(&mut self, source: &GisMapSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let target = self.value.as_mut().ok_or("gis-map-store.initializer-clone-target")?;
        let observed = match self.phase {
            0..=2 => {
                let (source, target, admission) = match self.phase {
                    0 => (&source.positions, &mut target.positions, "gis-map-store.initializer-position-admission"),
                    1 => (&source.routes, &mut target.routes, "gis-map-store.initializer-route-admission"),
                    _ => (&source.regions, &mut target.regions, "gis-map-store.initializer-region-admission"),
                };
                if self.index == 0 && target.capacity() == 0 {
                    target.try_reserve_exact(source.len()).map_err(|_| admission)?;
                }
                if let Some(feature) = source.get(self.index) {
                    let encoded = semio_framework_pack_json::to_json_string(feature).into_bytes();
                    if encoded.len() > GIS_MAP_OWNED_FIELD_BYTES {
                        return Err("gis-map-store.initializer-feature-too-large");
                    }
                    target.push(Self::clone_feature(feature)?);
                    self.index += 1;
                    cx.consume_fuel(encoded.len().max(1) as u64);
                    return Ok(false);
                }
                self.index = 0;
                &[]
            }
            3..=7 => Self::copy_child_field(&mut target.drawing, &source.drawing, self.phase, 3)?,
            8 => {
                if source.image.is_some() {
                    target.image = Some(Self::empty_child());
                }
                &[]
            }
            9..=13 => {
                let source = source.image.as_ref().ok_or("gis-map-store.initializer-image-source")?;
                let target = target.image.as_mut().ok_or("gis-map-store.initializer-image-target")?;
                Self::copy_child_field(target, source, self.phase, 9)?
            }
            14..=18 => Self::copy_child_field(&mut target.value, &source.value, self.phase, 14)?,
            _ => {
                self.terminal = true;
                return Ok(true);
            }
        };
        self.phase = match self.phase {
            8 if source.image.is_none() => 14,
            value => value + 1,
        };
        cx.consume_fuel(observed.len().max(1) as u64);
        Ok(false)
    }

    fn take_value(&mut self) -> Option<GisMapSnapshot> {
        if !self.terminal {
            return None;
        }
        self.value.take()
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&GisMapSnapshotRetirementFactory, value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.terminal = true;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let retirement = self.retirement.as_mut().expect("GIS clone retirement remains exact");
        match retirement.close_step(1, maximum_bytes)? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "GIS clone retirement reported false terminal")),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for GisMapSnapshotCloneAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "GIS snapshot clone reached Drop before exact handoff or cursor retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GisMapStoreInitializationPhase {
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
    fn new(envelope: store::ArtifactEnvelope<GisMapSnapshot, GisMapMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Self {
        Self {
            actor,
            operation,
            generation,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            clone: std::mem::ManuallyDrop::new(None),
            edit_index: store::ArtifactStoreInitializationEditIndex::default(),
            resume_phase: None,
            phase: GisMapStoreInitializationPhase::ValidateEnvelope,
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
        self.phase = GisMapStoreInitializationPhase::RetireFault;
    }

    fn pump_active(&mut self) -> Result<bool, semio_framework_value::ValueError> {
        let Some(active) = self.active.as_mut() else { return Ok(false) };
        match active.close_step(1, GIS_MAP_OWNED_FIELD_BYTES)? {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= GIS_MAP_OWNED_FIELD_BYTES => Ok(true),
            store::SnapshotRetirementStep::Pending { .. } => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "GIS store initializer retirement exceeded its exact grant")),
            store::SnapshotRetirementStep::Blocked => Ok(true),
            store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                drop(self.active.take());
                Ok(true)
            }
            store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "GIS store initializer retirement reported a false terminal")),
        }
    }

    fn pump_terminal_retirement(&mut self) -> Result<bool, semio_framework_value::ValueError> {
        if self.pump_active()? {
            return Ok(false);
        }
        if let Some(runtime) = self.runtime.as_mut() {
            match runtime.close_step(&GisMapSnapshotRetirementFactory, 1, GIS_MAP_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    drop(self.runtime.take());
                    return Ok(false);
                }
                store::SnapshotRetirementStep::Complete => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "GIS initialization runtime reported a false terminal")),
                _ => return Ok(false),
            }
        }
        if let Some(clone) = self.clone.as_mut() {
            match clone.close_step(1, GIS_MAP_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if clone.terminal_is_empty() => {
                    drop(self.clone.take());
                    return Ok(false);
                }
                store::SnapshotRetirementStep::Complete => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "GIS snapshot clone reported a false terminal")),
                _ => return Ok(false),
            }
        }
        if self.envelope_retirement.is_none() {
            if let Some(envelope) = self.envelope.take() {
                *self.envelope_retirement = Some(gis_map_envelope_decode_owner_bundle().retire_envelope(envelope));
                return Ok(false);
            }
        }
        if let Some(retirement) = self.envelope_retirement.as_mut() {
            return match retirement.close_step(1, GIS_MAP_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.envelope_retirement.take());
                    Ok(true)
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "GIS initialization envelope retirement reported a false terminal")),
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
            && self.clone.is_none()
    }
}

impl Drop for GisMapStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty_inner(), "GIS store initialization authority reached Drop before exact candidate handoff or retained rejection close");
    }
}

pub fn gis_map_document_store_initialization_job(
    envelope: store::ArtifactEnvelope<GisMapSnapshot, GisMapMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    actor: protocol::ActorId,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<GisMapSnapshot, GisMapMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(GisMapStoreInitializationAuthority::new(envelope, operation, generation, actor)))
}

enum GisMapRetirementOwner {
    Snapshot(GisMapSnapshot),
    Mutation(GisMapMutation),
    MutationFields(GisMapMutationFields),
    Feature(MapFeature),
    Value(semio_framework_value::DslValue),
    ValueEntry { key: String, value: Option<semio_framework_value::DslValue> },
}

enum GisMapMutationFields {
    Feature(Option<MapFeature>),
    String(String),
    Value { id: String, value: Option<semio_framework_value::DslValue> },
}

struct GisMapOwnedRetirement {
    owner: std::mem::ManuallyDrop<Option<GisMapRetirementOwner>>,
    active: std::mem::ManuallyDrop<Option<Box<GisMapOwnedRetirement>>>,
    phase: u8,
}

impl GisMapOwnedRetirement {
    fn new(owner: GisMapRetirementOwner) -> Self {
        Self { owner: std::mem::ManuallyDrop::new(Some(owner)), active: std::mem::ManuallyDrop::new(None), phase: 0 }
    }

    fn spawn(active: &mut Option<Box<Self>>, owner: GisMapRetirementOwner) -> store::SnapshotRetirementStep {
        *active = Some(Box::new(Self::new(owner)));
        store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
    }

    fn release_string(value: &mut String, phase: &mut u8, next: u8, maximum_items: usize, maximum_bytes: usize) -> store::SnapshotRetirementStep {
        if maximum_items == 0 || value.len() > maximum_bytes {
            return store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let value = std::mem::take(value);
        let released_bytes = value.len();
        drop(value);
        *phase = next;
        store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes }
    }

    fn child_step<S>(child: &mut store::ArtifactChild<S>, phase: &mut u8, base: u8, maximum_items: usize, maximum_bytes: usize) -> Option<store::SnapshotRetirementStep> {
        let step = match *phase - base {
            0 => Self::release_string(&mut child.child_id, phase, base + 1, maximum_items, maximum_bytes),
            1 => Self::release_string(&mut child.target.artifact_id, phase, base + 2, maximum_items, maximum_bytes),
            2 => Self::release_string(&mut child.target.dialect.artifact_kind, phase, base + 3, maximum_items, maximum_bytes),
            3 => Self::release_string(&mut child.target.dialect.standard, phase, base + 4, maximum_items, maximum_bytes),
            4 => Self::release_string(&mut child.target.dialect.subset, phase, base + 5, maximum_items, maximum_bytes),
            _ => return None,
        };
        Some(step)
    }

    fn advance(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let Some(owner) = self.owner.as_mut() else { return Ok(store::SnapshotRetirementStep::Complete) };
        match owner {
            GisMapRetirementOwner::Snapshot(value) => match self.phase {
                0 => {
                    if let Some(value) = value.positions.pop() {
                        return Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::Feature(value)));
                    }
                    self.phase = 1;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                1 => {
                    if let Some(value) = value.routes.pop() {
                        return Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::Feature(value)));
                    }
                    self.phase = 2;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                2 => {
                    if let Some(value) = value.regions.pop() {
                        return Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::Feature(value)));
                    }
                    self.phase = 3;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                3..=7 => Ok(Self::child_step(&mut value.drawing, &mut self.phase, 3, maximum_items, maximum_bytes).expect("GIS drawing child phase is exact")),
                8..=12 => {
                    if let Some(image) = value.image.as_mut() {
                        Ok(Self::child_step(image, &mut self.phase, 8, maximum_items, maximum_bytes).expect("GIS image child phase is exact"))
                    } else {
                        self.phase = 13;
                        Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                    }
                }
                13..=17 => Ok(Self::child_step(&mut value.value, &mut self.phase, 13, maximum_items, maximum_bytes).expect("GIS value child phase is exact")),
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            GisMapRetirementOwner::Feature(value) => match self.phase {
                0 => Ok(Self::release_string(&mut value.id, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => {
                    let data = std::mem::replace(&mut value.data, semio_framework_value::DslValue::Null);
                    self.phase = 2;
                    Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::Value(data)))
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            GisMapRetirementOwner::Value(value) => match value {
                semio_framework_value::DslValue::String(value) => {
                    if self.phase == 0 {
                        return Ok(Self::release_string(value, &mut self.phase, 1, maximum_items, maximum_bytes));
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                semio_framework_value::DslValue::Bytes(value) => {
                    if self.phase == 0 {
                        if value.len() > maximum_bytes { return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
                        let bytes = std::mem::take(value);
                        let released_bytes = bytes.len();
                        drop(bytes);
                        self.phase = 1;
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                semio_framework_value::DslValue::Array(values) => {
                    if let Some(value) = values.pop() {
                        Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::Value(value)))
                    } else {
                        drop(self.owner.take());
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                }
                semio_framework_value::DslValue::Object(values) => {
                    if let Some((key, value)) = values.pop() {
                        Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::ValueEntry { key, value: Some(value) }))
                    } else {
                        drop(self.owner.take());
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                }
                semio_framework_value::DslValue::Null | semio_framework_value::DslValue::Bool(_) | semio_framework_value::DslValue::Number(_) => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            GisMapRetirementOwner::ValueEntry { key, value } => match self.phase {
                0 => Ok(Self::release_string(key, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => {
                    let value = value.take().ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "GIS value entry lost its retained value"))?;
                    self.phase = 2;
                    Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::Value(value)))
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            GisMapRetirementOwner::Mutation(_) => {
                use GisMapMutation::*;
                let mutation = match self.owner.take() {
                    Some(GisMapRetirementOwner::Mutation(value)) => value,
                    _ => unreachable!("GIS mutation owner variant remains exact"),
                };
                let fields = match mutation {
                    CreatePosition(payload) => GisMapMutationFields::Feature(Some(payload.item)),
                    CreateRoute(payload) => GisMapMutationFields::Feature(Some(payload.item)),
                    CreateRegion(payload) => GisMapMutationFields::Feature(Some(payload.item)),
                    DeletePosition(payload) => GisMapMutationFields::String(payload.id),
                    DeleteRoute(payload) => GisMapMutationFields::String(payload.id),
                    DeleteRegion(payload) => GisMapMutationFields::String(payload.id),
                    ReorderPositions(payload) => GisMapMutationFields::String(payload.id),
                    ReorderRoutes(payload) => GisMapMutationFields::String(payload.id),
                    ReorderRegions(payload) => GisMapMutationFields::String(payload.id),
                    ReplacePositionData(payload) => GisMapMutationFields::Value { id: payload.id, value: Some(payload.new_data) },
                    ReplaceRouteData(payload) => GisMapMutationFields::Value { id: payload.id, value: Some(payload.new_data) },
                    ReplaceRegionData(payload) => GisMapMutationFields::Value { id: payload.id, value: Some(payload.new_data) },
                };
                *self.owner = Some(GisMapRetirementOwner::MutationFields(fields));
                Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
            }
            GisMapRetirementOwner::MutationFields(fields) => match fields {
                GisMapMutationFields::Feature(value) => {
                    if let Some(value) = value.take() {
                        return Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::Feature(value)));
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                GisMapMutationFields::String(value) => {
                    if self.phase == 0 {
                        return Ok(Self::release_string(value, &mut self.phase, 1, maximum_items, maximum_bytes));
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                GisMapMutationFields::Value { id, value } => match self.phase {
                    0 => Ok(Self::release_string(id, &mut self.phase, 1, maximum_items, maximum_bytes)),
                    1 => {
                        let value = value.take().ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "GIS mutation value owner was lost"))?;
                        self.phase = 2;
                        Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::Value(value)))
                    }
                    _ => {
                        drop(self.owner.take());
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                },
            },
        }
    }
}

impl store::ErasedSnapshotRetirement for GisMapOwnedRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if let Some(active) = self.active.as_mut() {
            return match active.close_step(maximum_items.min(1), maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                    drop(self.active.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "GIS nested retirement reported false terminal")),
                step => Ok(step),
            };
        }
        self.advance(maximum_items.min(1), maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none() && self.active.is_none()
    }
}

impl Drop for GisMapOwnedRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || store::ErasedSnapshotRetirement::terminal_is_empty(self), "GIS owner reached Drop before cursor retirement reached terminal-empty");
    }
}

struct GisMapSnapshotRootRetirement {
    owner: std::mem::ManuallyDrop<Option<std::sync::Arc<GisMapSnapshot>>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl store::ErasedSnapshotRetirement for GisMapSnapshotRootRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "GIS snapshot root retirement reported false terminal")),
                step => Ok(step),
            };
        }
        let Some(owner) = self.owner.take() else { return Ok(store::SnapshotRetirementStep::Complete) };
        match std::sync::Arc::try_unwrap(owner) {
            Ok(value) => {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&GisMapSnapshotRetirementFactory, value));
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

impl Drop for GisMapSnapshotRootRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owner.is_none() && self.retirement.is_none()), "GIS snapshot root reached Drop before exact Arc handback");
    }
}

impl store::ArtifactEnvelopeOwnedFieldCatalog<GisMapSnapshot, GisMapMutation> for GisMapEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Result<Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<GisMapSnapshot, GisMapMutation>>, Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<GisMapSnapshot>>> {
        store::ArtifactEnvelopeFreshVcsAuthority::try_new(self.begin_snapshot(operation, generation, path), std::sync::Arc::new(GisMapSnapshotRetirementFactory), std::sync::Arc::new(GisMapMutationRetirementFactory), self.edit_history_decoder())
            .map(|authority| Box::new(authority) as Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<GisMapSnapshot, GisMapMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<GisMapSnapshot>> {
        Box::new(GisMapSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<GisMapMutation>> {
        Box::new(GisMapMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(GisMapRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<GisMapMutation>>> {
        store::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(GisMapMutationRetirementFactory))
    }
}

pub fn gis_map_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<GisMapSnapshot, GisMapMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(GisMapEnvelopeOwnedFieldCatalog), std::sync::Arc::new(GisMapSnapshotRetirementFactory), std::sync::Arc::new(GisMapMutationRetirementFactory))
}

struct GisMapRejectedConflictAuthority {
    terminal: bool,
}

impl store::ArtifactEnvelopeSprConflictAuthority for GisMapRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "gis-map-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })
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

pub struct GisMapEnvelopeOwnedFieldCatalog;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

fn decode_gis_map_snapshot_pack(bytes: &[u8]) -> Result<GisMapSnapshot, ()> {
    <GisMapSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|_| ())
}

fn decode_gis_map_mutation_pack(bytes: &[u8]) -> Result<GisMapMutation, ()> {
    GisMapMutation::decode_op(bytes).map_err(|_| ())
}

macro_rules! gis_map_owned_field_close_capacity {
    (ArtifactEnvelopeSnapshotFieldAuthority) => {
        fn next_close_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
            Ok(usize::from(self.retirement.is_some()) * GIS_MAP_OWNED_FIELD_BYTES)
        }

        fn maximum_close_byte_demand(&self) -> usize {
            store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
        }

        fn maximum_retained_close_bytes(&self) -> usize {
            store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
        }
    };
    (ArtifactEnvelopeMutationFieldAuthority) => {};
}

macro_rules! gis_map_owned_field_authority {
    ($state:ident, $authority:ident, $value:ty, $authority_trait:ident, $target_trait:ident, $publish:ident, $decode:path, $factory:expr, $kind:literal) => {
        #[expect(clippy::large_enum_variant, reason = "The active decoder keeps its fixed path and admitted hex authority inline without a second allocation at the state transition.")]
        enum $state {
            AwaitToken,
            Decode(store::OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>),
            Ready,
            Published,
            Closing,
            Complete,
        }

        pub(crate) struct $authority {
            operation: semio_framework_job::OperationId,
            generation: semio_framework_job::Generation,
            path: store::OwnedSchemaPath,
            state: $state,
            value: std::mem::ManuallyDrop<Option<$value>>,
            retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
        }

        impl $authority {
            pub(crate) fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
                Self { operation, generation, path, state: $state::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
            }

            fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
                store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
            }
        }

        impl store::$authority_trait<$value> for $authority {
            fn accept_token(
                &mut self,
                token: store::OwnedSchemaToken,
                terminal: bool,
                source: &store::OwnedSchemaRecordCursor,
                cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
                let path = self.path;
                let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path };
                if matches!(self.state, $state::AwaitToken) {
                    if !terminal {
                        return Err(diagnostic(concat!("gis-map-envelope.", $kind, "-pack-must-be-scalar"), token.start));
                    }
                    self.state = $state::Decode(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
                }
                let $state::Decode(authority) = &mut self.state else {
                    return Err(diagnostic(concat!("gis-map-envelope.", $kind, "-pack-token-replayed"), token.start));
                };
                match authority.step(source, cx) {
                    store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
                    store::OwnedSchemaHexStep::Complete => {
                        let bytes = authority.as_bytes().ok_or_else(|| diagnostic(concat!("gis-map-envelope.", $kind, "-pack-missing"), token.start))?;
                        let value = $decode(bytes).map_err(|_| diagnostic(concat!("gis-map-envelope.", $kind, "-pack-malformed"), token.start))?;
                        if !authority.release() {
                            return Err(diagnostic(concat!("gis-map-envelope.", $kind, "-pack-release-duplicate"), token.start));
                        }
                        *self.value = Some(value);
                        self.state = $state::Ready;
                        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
                    }
                    store::OwnedSchemaHexStep::Cancelled => Err(diagnostic(concat!("gis-map-envelope.", $kind, "-pack-cancelled"), token.start)),
                    store::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
                }
            }

            fn publish_reserved(
                &mut self,
                target: &mut dyn store::$target_trait<$value>,
                reservation: store::ArtifactEnvelopeFieldReservation,
                _cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
                if !matches!(self.state, $state::Ready) {
                    return Err(self.diagnostic(concat!("gis-map-envelope.", $kind, "-pack-not-ready"), 0));
                }
                let value = self.value.take().ok_or_else(|| self.diagnostic(concat!("gis-map-envelope.", $kind, "-owner-missing"), 0))?;
                target.$publish(reservation, value);
                self.state = $state::Published;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }

            gis_map_owned_field_close_capacity!($authority_trait);

            fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
                if maximum_items == 0 {
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                if let $state::Decode(authority) = &mut self.state {
                    authority.cancel();
                    self.state = $state::Closing;
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                }
                if self.retirement.is_none() {
                    if let Some(value) = self.value.take() {
                        *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned($factory, value));
                        self.state = $state::Closing;
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    self.state = $state::Complete;
                    return Ok(store::SnapshotRetirementStep::Complete);
                }
                let path = self.path;
                let retirement = self.retirement.as_mut().expect("GIS packed field retirement remains retained");
                match retirement.close_step(maximum_items.min(1), maximum_bytes).map_err(|_| store::OwnedSchemaDecodeDiagnostic { code: concat!("gis-map-envelope.", $kind, "-retirement-fault"), offset: 0, line: 0, column: 0, path })? {
                    store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                        drop(self.retirement.take());
                        self.state = $state::Complete;
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                    store::SnapshotRetirementStep::Complete => Err(self.diagnostic(concat!("gis-map-envelope.", $kind, "-retirement-false-terminal"), 0)),
                    step => Ok(step),
                }
            }

            fn terminal_is_empty(&self) -> bool {
                matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none()
            }
        }

        impl Drop for $authority {
            fn drop(&mut self) {
                assert!(std::thread::panicking() || (matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none()), concat!("GIS ", $kind, " decode reached Drop before publication or bounded retirement"));
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
    &GisMapSnapshotRetirementFactory,
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
    &GisMapMutationRetirementFactory,
    "mutation"
);
