//! 🏠️ Artifact document-store and publication authorities.

use crate::standards::v1::subsets::any::schema::mutations::GisMapMutation;
use crate::{GisMapSnapshot, MapFeature};
use protocol::{Mutation, MutationDiff, OpBinary};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::RetirementDemand;
struct GisMapStoreInitializationAuthority {
    actor: protocol::ActorId,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    envelope: std::mem::ManuallyDrop<Option<store::ArtifactEnvelope<GisMapSnapshot, GisMapMutation>>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<GisMapSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<GisMapSnapshot, GisMapMutation>>>,
    owners: std::mem::ManuallyDrop<Option<store::DocumentStoreOwners<GisMapSnapshot, GisMapMutation>>>,
    pending: std::mem::ManuallyDrop<Option<GisMapSnapshot>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    clone: std::mem::ManuallyDrop<Option<GisMapSnapshotCloneAuthority>>,
    edit_index: store::ArtifactStoreInitializationEditIndex,
    phase: GisMapStoreInitializationPhase,
    resume_phase: Option<GisMapStoreInitializationPhase>,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    fault_detail: Option<Vec<u8>>,
    publication: semio_framework_job::RetainedJobPublication,
    terminal_handoff: bool,
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
                self.publication.advance_from_source(semio_framework_job::JobPublicationKind::Fault, detail, cx)
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

pub(crate) const GIS_MAP_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct GisMapSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<GisMapSnapshot> for GisMapSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &GisMapSnapshot) -> usize { semio_framework_value::retirement::owned_retirement_birth_bytes::<GisMapSnapshot>() }

    fn retire_owned(&self, value: GisMapSnapshot, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (semio_framework_value::ValueError, GisMapSnapshot)> {
        semio_framework_value::retirement::admit_owned_retirement(value, grant)
    }
}

impl store::SnapshotRetirementFactory<GisMapSnapshot> for GisMapSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<GisMapSnapshot>) -> usize { semio_framework_value::shared_retirement_birth_bytes::<GisMapSnapshot>() }

    fn retire(&self, snapshot: std::sync::Arc<GisMapSnapshot>, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<GisMapSnapshot>)> {
        semio_framework_value::admit_shared_retirement(snapshot, grant, true)
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct GisMapMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<GisMapMutation> for GisMapMutationRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &GisMapMutation) -> usize { semio_framework_value::retirement::owned_retirement_birth_bytes::<GisMapMutation>() }

    fn retire_owned(&self, value: GisMapMutation, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (semio_framework_value::ValueError, GisMapMutation)> {
        semio_framework_value::retirement::admit_owned_retirement(value, grant)
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

    fn close_demands(&self, body: usize) -> Result<RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, semio_framework_value::ValueError> {
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "GIS clone retirement depth overflow"))?;
            Ok(demand)
        };
        if let Some(active) = self.retirement.as_ref() {
            return nested(store::artifact_retirement_box_demands(active, body)?);
        }
        if self.value.is_some() {
            return nested(store::artifact_retirement_owned_birth_demands(&*self.value)?);
        }
        Ok(Default::default())
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        if self.retirement.is_some() {
            return store::artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| {
                self.terminal = self.value.is_none() && self.retirement.is_none();
                RetainedCloneStep::Progress(step.progress())
            });
        }
        if self.value.is_some() {
            return store::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant);
        }
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(Default::default()))
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

enum GisMapInitializerTurn {
    Yield,
    Complete,
    Cancelled,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GisMapStoreInitializationPhase {
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
    fn new(envelope: store::ArtifactEnvelope<GisMapSnapshot, GisMapMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Self {
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
            publication: semio_framework_job::RetainedJobPublication::new(),
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
                    match store::bounded_artifact_store_owners::<GisMapSnapshot, GisMapMutation>(grant) {
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

    fn nested(mut demand: RetirementDemand) -> Result<RetirementDemand, semio_framework_value::ValueError> {
        demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "GIS initializer retirement depth overflow"))?;
        Ok(demand)
    }

    fn displaced_factory() -> std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<GisMapSnapshot>> {
        std::sync::Arc::new(GisMapSnapshotRetirementFactory)
    }

    fn close_demands(&self, body: usize) -> Result<RetirementDemand, semio_framework_value::ValueError> {
        if let Some(active) = self.active.as_ref() {
            return Self::nested(store::artifact_retirement_box_demands(active, body)?);
        }
        if self.pending.is_some() {
            return Self::nested(store::artifact_retirement_owned_birth_demands(&*self.pending)?);
        }
        if let Some(runtime) = self.runtime.as_ref() {
            return Self::nested(runtime.initialization_retirement_demands(body)?);
        }
        if let Some(clone) = self.clone.as_ref() {
            return clone.close_demands(body);
        }
        if let Some(active) = self.envelope_retirement.as_ref() {
            return Self::nested(store::artifact_retirement_box_demands(active, body)?);
        }
        if !self.publication.terminal_is_empty() && self.close_core_is_terminal() {
            return Self::nested(self.publication.retirement_demands()?);
        }
        if let Some(envelope) = self.envelope.as_ref() {
            let Some(owners) = self.owners.as_ref() else {
                return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<Option<store::DocumentStoreOwners<GisMapSnapshot, GisMapMutation>>>(), capacity_bytes: store::bounded_artifact_store_owners_birth_bytes::<GisMapSnapshot, GisMapMutation>(), depth: 1, ..Default::default() });
            };
            if !owners.constructor_is_complete() {
                return Self::nested(owners.constructor_demands(body)?);
            }
            let mut demand = Self::nested(owners.uninstalled_envelope_retirement_demands(envelope))?;
            demand.copy_bytes = demand.copy_bytes.checked_add(std::mem::size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>()).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "GIS initializer envelope placement overflow"))?;
            return Ok(demand);
        }
        if let Some(owners) = self.owners.as_ref() {
            return if owners.uninstalled_owners_terminal_is_empty() {
                Ok(RetirementDemand { copy_bytes: std::mem::size_of::<Option<store::DocumentStoreOwners<GisMapSnapshot, GisMapMutation>>>(), depth: 1, ..Default::default() })
            } else {
                Self::nested(owners.uninstalled_owners_demands(body)?)
            };
        }
        Ok(Default::default())
    }

    fn drain_displaced(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if self.active.is_some() {
            return store::artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        store::artifact_retirement_admit_owned(&mut self.pending, &mut self.active, child)
    }

    fn close_is_terminal(&self) -> bool {
        self.close_core_is_terminal() && self.publication.terminal_is_empty()
    }

    fn close_core_is_terminal(&self) -> bool {
        self.envelope.is_none() && self.runtime.is_none() && self.clone.is_none() && self.active.is_none() && self.pending.is_none() && self.envelope_retirement.is_none() && self.owners.is_none()
    }

    fn close_original(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.close_is_terminal() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth.max(1) {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "GIS initializer retirement exceeds admitted depth"));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if self.active.is_some() || self.pending.is_some() {
            return self.drain_displaced(grant);
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let step = runtime.close_step(&Self::displaced_factory(), child)?;
            if matches!(step, RetainedCloneStep::Complete(_)) {
                drop(self.runtime.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(clone) = self.clone.as_mut() {
            let step = clone.close_step(child)?;
            if clone.terminal_is_empty() {
                drop(self.clone.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(retirement) = self.envelope_retirement.as_mut() {
            if retirement.terminal_is_empty() {
                let released = std::mem::size_of_val(retirement.as_ref());
                drop(self.envelope_retirement.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: released, ..empty }));
            }
            let step = retirement.close_step(child)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, retirement.terminal_is_empty(), "GIS initializer envelope")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope.is_some() {
            if self.owners.is_none() {
                let placement = demand.copy_bytes;
                let owner_grant = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - placement, ..grant };
                return match store::bounded_artifact_store_owners::<GisMapSnapshot, GisMapMutation>(owner_grant) {
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
            let owners = self.owners.as_mut().expect("retained initializer owner catalog");
            if !owners.constructor_is_complete() {
                return owners.admit_constructor(child).map(RetainedCloneStep::Progress).map_err(|(error, progress)| error.with_retained_progress(progress));
            }
            let placement = std::mem::size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>();
            let child = RetainedCloneGrant { maximum_copy_bytes: child.maximum_copy_bytes - placement, ..child };
            let owners = self.owners.take().expect("retained initializer owner catalog");
            let envelope = self.envelope.take().expect("retained initializer envelope");
            return match owners.retire_envelope_uninstalled(envelope, child) {
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
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owners.uninstalled_owners_terminal_is_empty(), "GIS initializer original catalog")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if !self.publication.terminal_is_empty() {
            return self.publication.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.envelope_retirement.is_none()
            && self.owners.is_none()
            && self.pending.is_none()
            && self.publication.terminal_is_empty()
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

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct GisMapEnvelopeOwnedFieldCatalog;

impl store::ArtifactEnvelopeOwnedFieldCatalog<GisMapSnapshot, GisMapMutation> for GisMapEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Result<Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<GisMapSnapshot, GisMapMutation>>, Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<GisMapSnapshot>>> {
        store::ArtifactEnvelopeFreshVcsAuthority::try_new(
            self.begin_snapshot(operation, generation, path),
            std::sync::Arc::new(GisMapSnapshotRetirementFactory),
            std::sync::Arc::new(GisMapMutationRetirementFactory),
            self.edit_history_decoder(),
        )
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
    (@demands, $value:ty) => {
        fn next_close_copy_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.copy_bytes) }

        fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> { self.close_demands(maximum_copy_bytes).map(|demand| demand.capacity_bytes) }

        fn next_close_release_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.release_bytes) }

        fn next_close_depth_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.depth) }
    };
    (ArtifactEnvelopeSnapshotFieldAuthority, $value:ty) => {
        gis_map_owned_field_close_capacity!(@demands, $value);

        fn maximum_close_byte_demand(&self) -> usize {
            store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
        }

        fn maximum_retained_close_bytes(&self) -> usize {
            store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
        }
    };
    (ArtifactEnvelopeMutationFieldAuthority, $value:ty) => {
        gis_map_owned_field_close_capacity!(@demands, $value);
    };
}

macro_rules! gis_map_owned_field_authority {
    ($state:ident, $authority:ident, $value:ty, $authority_trait:ident, $target_trait:ident, $publish:ident, $decode:path, $kind:literal) => {
        #[expect(clippy::large_enum_variant, reason = "The active decoder keeps its fixed path and admitted hex authority inline without a second allocation at the state transition.")]
        enum $state {
            AwaitToken,
            Decode(store::OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>),
            Ready,
            Published,
            Closing,
            Complete,
        }

        struct $authority {
            operation: semio_framework_job::OperationId,
            generation: semio_framework_job::Generation,
            path: store::OwnedSchemaPath,
            state: $state,
            value: std::mem::ManuallyDrop<Option<$value>>,
            retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
        }

        impl $authority {
            fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
                Self { operation, generation, path, state: $state::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
            }

            fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
                store::OwnedSchemaDecodeDiagnostic { offset, ..store::OwnedSchemaDecodeDiagnostic::before(code, self.path) }
            }

            fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, store::OwnedSchemaDecodeDiagnostic> {
                if matches!(self.state, $state::Decode(_)) {
                    return Ok(semio_framework_value::RetirementDemand { release_bytes: GIS_MAP_OWNED_FIELD_BYTES, depth: 1, ..Default::default() });
                }
                let demand = match self.retirement.as_ref() {
                    Some(owner) => store::artifact_retirement_box_demands(owner, maximum_copy_bytes),
                    None => store::artifact_retirement_owned_birth_demands(&self.value),
                };
                demand.map_err(|error| self.diagnostic(concat!("gis-map-envelope.", $kind, "-retirement-demand-fault"), 0).with_native(error))
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
                let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { offset, ..store::OwnedSchemaDecodeDiagnostic::before(code, path) };
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

            gis_map_owned_field_close_capacity!($authority_trait, $value);

            fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, store::OwnedSchemaDecodeDiagnostic> {
                let empty = RetainedCloneProgress::default();
                if self.terminal_is_empty() {
                    return Ok(RetainedCloneStep::Complete(empty));
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(empty));
                }
                let demand = self.close_demands(grant.maximum_copy_bytes)?;
                if grant.maximum_depth < demand.depth {
                    return Err(self.diagnostic(concat!("gis-map-envelope.", $kind, "-retirement-depth"), 0));
                }
                if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
                    return Ok(RetainedCloneStep::Progress(empty));
                }
                if let $state::Decode(authority) = &mut self.state {
                    authority.cancel();
                    self.state = $state::Closing;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: GIS_MAP_OWNED_FIELD_BYTES, ..empty }));
                }
                let step = if self.retirement.is_some() {
                    store::artifact_retirement_box_close_step(&mut self.retirement, grant)
                } else if self.value.is_some() {
                    store::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant)
                } else {
                    Ok(RetainedCloneStep::Complete(empty))
                }
                .map_err(|error| self.diagnostic(concat!("gis-map-envelope.", $kind, "-retirement-fault"), 0).with_native(error))?;
                self.state = $state::Closing;
                if self.value.is_none() && self.retirement.is_none() {
                    self.state = $state::Complete;
                    return Ok(RetainedCloneStep::Complete(step.progress()));
                }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }

            fn terminal_is_empty(&self) -> bool {
                matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none()
            }
        }

        impl Drop for $authority {
            fn drop(&mut self) {
                assert!(
                    (matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none()) || std::thread::panicking(),
                    concat!("GIS ", $kind, " decode reached Drop before publication or bounded retirement")
                );
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
    "mutation"
);

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
        Err(store::OwnedSchemaDecodeDiagnostic { offset: token.start, ..store::OwnedSchemaDecodeDiagnostic::before("gis-map-envelope.fresh-conflict-not-admitted", store::OwnedSchemaPath::ROOT) })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> { Ok(0) }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> { Ok(0) }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> { Ok(0) }

    fn next_close_depth_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> { Ok(usize::from(!self.terminal)) }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, store::OwnedSchemaDecodeDiagnostic> {
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


