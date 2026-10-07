//! 🏠️ Artifact document-store and publication authorities.

use crate::standards::v1::subsets::any::schema::mutations::Process3dMutation;
use store::{ArtifactEnvelopeMutationFieldAuthority as _, ArtifactEnvelopeSnapshotFieldAuthority as _};
use crate::{Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, Pose, Process3dSnapshot, ProcessMeasure, ProcessStep, StepOrigin, Stock, StockQuantity, WorkingSolid, WorkshopMachine};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Process3dPublicationLease {
    operation: u64,
    generation: u64,
    base_revision: u64,
    parent_revision: u64,
    live_revision: u64,
    maximum_items: usize,
    maximum_output_pages: usize,
    maximum_controls: usize,
    closing: bool,
    terminal: bool,
}

impl semio_framework_job::FixedOperationOwner for Process3dPublicationLease {
    fn retained_bytes(&self) -> usize {
        size_of::<Self>()
    }

    fn cancel(&mut self) {
        self.closing = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 || maximum_bytes < size_of::<Self>() {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if !self.terminal {
            self.terminal = true;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: size_of::<Self>() };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.terminal
    }
}

type Process3dPublicationRegistry = semio_framework_job::FixedOperationRegistry<Process3dPublicationLease, PROCESS3D_PUBLICATION_SLOTS>;

fn process3d_publication_leases() -> &'static std::sync::Mutex<Process3dPublicationRegistry> {
    static LEASES: std::sync::OnceLock<std::sync::Mutex<semio_framework_job::FixedOperationRegistry<Process3dPublicationLease, PROCESS3D_PUBLICATION_SLOTS>>> = std::sync::OnceLock::new();
    LEASES.get_or_init(|| std::sync::Mutex::new(Process3dPublicationRegistry::new(PROCESS3D_PUBLICATION_SLOTS * size_of::<Process3dPublicationLease>())))
}

pub fn process3d_admit_publication_authority(
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    base_revision: u64,
    parent_revision: u64,
    live_revision: u64,
    limits: Process3dPublicationLimits,
) -> Result<(), &'static str> {
    let Process3dPublicationLimits { maximum_items, maximum_output_pages, maximum_controls } = limits;
    if generation.0 != live_revision || base_revision != live_revision || parent_revision != base_revision {
        return Err("process3d-publication.initial-freshness");
    }
    let mut leases = process3d_publication_leases().try_lock().map_err(|_| "process3d-publication.contended")?;
    if leases.get_operation(operation).is_some() {
        return Err("process3d-publication.operation-duplicate");
    }
    if maximum_items == 0 || maximum_items > PROCESS3D_MAXIMUM_DOMAIN_ITEMS || maximum_output_pages != PROCESS3D_MOUNTED_OUTPUT_CHANNELS || maximum_controls != PROCESS3D_MOUNTED_CONTROL_CREDITS {
        return Err("process3d-publication.domain-credits");
    }
    let key = process3d_publication_key(operation, generation);
    leases
        .admit(
            key,
            Process3dPublicationLease { operation: operation.0, generation: generation.0, base_revision, parent_revision, live_revision, maximum_items, maximum_output_pages, maximum_controls, closing: false, terminal: false },
        )
        .map_err(|_| "process3d-publication.saturated")?;
    #[cfg(test)]
    process3d_publication_test_admissions().lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(key);
    Ok(())
}

/// 🔐️ The ONE lease the app grants ITSELF for a replacement the host began (`Effect::LoadDocument`
/// → `build_document_store_initialization_job`): base, parent and live revision are all the
/// generation the host started the replacement on — the only publication that commit can accept —
/// with the domain's own credits. A holder that admitted a HOST lease for the same operation first
/// keeps it (`Err`).
///
/// A self-grant is NOT a host publication and does not live in the host's fixed table. That table
/// is four DIRECT-MAPPED slots: a self-grant lying in it refuses an unrelated host publication
/// whose key happens to map to the same slot (`process3d-publication.saturated`) while three slots
/// stay free — which is exactly what a test binary hosting several app instances beside the
/// fixture laws produces. The host drives one replacement per instance at a time, so this
/// authority holds exactly one lease and a new self-grant supersedes the load the host already
/// abandoned; the happy path releases through `process3d_release_app_publication_authority` at
/// validation or initializer retirement.
fn process3d_app_publication_lease() -> &'static std::sync::Mutex<Option<(semio_framework_job::FixedOperationKey, Process3dPublicationLease)>> {
    static LEASE: std::sync::OnceLock<std::sync::Mutex<Option<(semio_framework_job::FixedOperationKey, Process3dPublicationLease)>>> = std::sync::OnceLock::new();
    LEASE.get_or_init(|| std::sync::Mutex::new(None))
}

/// 🔎️ One lease by its exact key, from the host table first and the app self-grant second.
fn process3d_publication_lease_by_key(key: semio_framework_job::FixedOperationKey) -> Result<Option<Process3dPublicationLease>, &'static str> {
    let leases = process3d_publication_leases().try_lock().map_err(|_| "process3d-publication.contended")?;
    if let Some(lease) = leases.get(key) {
        return Ok(Some(*lease));
    }
    drop(leases);
    let app = process3d_app_publication_lease().try_lock().map_err(|_| "process3d-publication.contended")?;
    Ok(app.as_ref().filter(|(held, _)| *held == key).map(|(_, lease)| *lease))
}

/// 🔎️ One lease by operation, from the host table first and the app self-grant second.
fn process3d_publication_lease_by_operation(operation: semio_framework_job::OperationId) -> Result<Option<Process3dPublicationLease>, &'static str> {
    let leases = process3d_publication_leases().try_lock().map_err(|_| "process3d-publication.contended")?;
    if let Some((_, lease)) = leases.get_operation(operation) {
        return Ok(Some(*lease));
    }
    drop(leases);
    let app = process3d_app_publication_lease().try_lock().map_err(|_| "process3d-publication.contended")?;
    Ok(app.as_ref().filter(|(_, lease)| lease.operation == operation.0).map(|(_, lease)| *lease))
}

pub fn process3d_admit_app_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<(), &'static str> {
    let leases = process3d_publication_leases().try_lock().map_err(|_| "process3d-publication.contended")?;
    if leases.get_operation(operation).is_some() {
        return Err("process3d-publication.operation-duplicate");
    }
    drop(leases);
    let mut app = process3d_app_publication_lease().try_lock().map_err(|_| "process3d-publication.contended")?;
    if app.as_ref().is_some_and(|(_, lease)| lease.operation == operation.0) {
        return Err("process3d-publication.operation-duplicate");
    }
    *app = Some((
        process3d_publication_key(operation, generation),
        Process3dPublicationLease {
            operation: operation.0,
            generation: generation.0,
            base_revision: generation.0,
            parent_revision: generation.0,
            live_revision: generation.0,
            maximum_items: PROCESS3D_MAXIMUM_DOMAIN_ITEMS,
            maximum_output_pages: PROCESS3D_MOUNTED_OUTPUT_CHANNELS,
            maximum_controls: PROCESS3D_MOUNTED_CONTROL_CREDITS,
            closing: false,
            terminal: false,
        },
    ));
    Ok(())
}

fn process3d_validate_atomic_lease(lease: Process3dPublicationLease, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), &'static str> {
    if lease.operation != operation.0 {
        return Err("process3d-publication.wrong-operation");
    }
    if lease.generation != generation.0 {
        return Err("process3d-publication.wrong-generation");
    }
    if lease.live_revision != live_generation.0 || lease.base_revision != lease.live_revision {
        return Err("process3d-publication.wrong-base");
    }
    if lease.parent_revision != lease.base_revision {
        return Err("process3d-publication.wrong-parent");
    }
    if lease.maximum_items == 0 || lease.maximum_output_pages != PROCESS3D_MOUNTED_OUTPUT_CHANNELS || lease.maximum_controls != PROCESS3D_MOUNTED_CONTROL_CREDITS {
        return Err("process3d-publication.authority-credits");
    }
    Ok(())
}

pub fn process3d_document_store_owners() -> store::DocumentStoreOwners<Process3dSnapshot, Process3dMutation> {
    store::DocumentStoreOwners::new(
        std::sync::Arc::new(Process3dSnapshotRetirementFactory),
        std::sync::Arc::new(Process3dSnapshotRetirementFactory),
        std::sync::Arc::new(Process3dMutationRetirementFactory),
        Box::new(store::ArtifactStoreCursorDisposer::<Process3dSnapshot, Process3dMutation>::new()),
    )
}

struct Process3dStoreInitializationAuthority {
    actor: protocol::ActorId,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    base_revision: u64,
    parent_revision: u64,
    history_items: usize,
    machine_growth: usize,
    envelope: std::mem::ManuallyDrop<Option<store::ArtifactEnvelope<Process3dSnapshot, Process3dMutation>>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<Process3dSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<Process3dSnapshot, Process3dMutation>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    active_terminal: bool,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    clone_cursor: std::mem::ManuallyDrop<Option<Process3dSnapshotCopyCursor>>,
    census: std::mem::ManuallyDrop<Option<Process3dOwnerCensusCursor>>,
    candidate_disposer: std::mem::ManuallyDrop<Option<semio_framework_plugin::ArtifactDocumentStoreDisposer<Process3dSnapshot, Process3dMutation>>>,
    edit_index: store::ArtifactStoreInitializationEditIndex,
    phase: Process3dStoreInitializationPhase,
    resume_phase: Option<Process3dStoreInitializationPhase>,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    terminal_handoff: bool,
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<Process3dSnapshot, Process3dMutation> for Process3dStoreInitializationAuthority {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"process3d-store.initializer-stale-aba");
        }
        if (self.cancel_requested || cx.is_cancelled()) && !matches!(self.phase, Process3dStoreInitializationPhase::RetireCancelled | Process3dStoreInitializationPhase::Cancelled) {
            self.phase = Process3dStoreInitializationPhase::RetireCancelled;
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return semio_framework_job::StepOutcome::Yield;
        }
        match self.pump_active() {
            Ok(true) => {
                cx.consume_fuel(1);
                return semio_framework_job::StepOutcome::Yield;
            }
            Ok(false) => {}
            Err(error) => {
                self.fault = Some(error.into_message().into_bytes());
                self.phase = Process3dStoreInitializationPhase::RetireFault;
            }
        }
        if !matches!(self.phase, Process3dStoreInitializationPhase::RetireCancelled | Process3dStoreInitializationPhase::RetireFault | Process3dStoreInitializationPhase::Cancelled | Process3dStoreInitializationPhase::Fault | Process3dStoreInitializationPhase::Complete) {
            if let Some(runtime) = self.runtime.as_mut() {
                match runtime.settle_current_retirement_step(1, PROCESS3D_OWNER_BYTES) {
                    Ok(store::SnapshotRetirementStep::Complete) => {}
                    Ok(_) => { cx.consume_fuel(1); return semio_framework_job::StepOutcome::Yield; }
                    Err(error) => { self.fault = Some(error.into_message().into_bytes()); self.phase = Process3dStoreInitializationPhase::RetireFault; }
                }
            }
        }
        match self.phase {
            Process3dStoreInitializationPhase::BindGenesis => {
                let envelope = self.envelope.as_ref().expect("retained initializer genesis");
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, envelope.vcs.genesis.share_snapshot(), envelope.vcs.genesis.digest(), self.actor.clone()));
                drop(self.census.take());
                self.phase = Process3dStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                cx.consume_fuel(1);
                return semio_framework_job::StepOutcome::Yield;
            }
            Process3dStoreInitializationPhase::ValidateEnvelope => {
                let valid = self.envelope.as_ref().is_some_and(|envelope| envelope.schema == crate::PROCESS_3D_SCHEMA && !envelope.id.is_empty() && envelope.id.len() <= PROCESS3D_OWNER_BYTES);
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
            Process3dStoreInitializationPhase::CensusHistory { edit, mutation } => {
                let envelope = self.envelope.as_ref().expect("Process3d envelope retained");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    let initial_machines = envelope.vcs.genesis.snapshot().workshop.machines.len();
                    let machine_capacity = match initial_machines.checked_add(self.machine_growth) {
                        Some(value) if value <= PROCESS3D_MAXIMUM_DOMAIN_ITEMS => value,
                        _ => {
                            self.fail(b"process3d-store.initializer-machine-growth");
                            return semio_framework_job::StepOutcome::Yield;
                        }
                    };
                    if machine_capacity.checked_mul(size_of::<WorkshopMachine>()).is_none_or(|bytes| bytes > PROCESS3D_MAXIMUM_DOMAIN_BYTES) {
                        self.fail(b"process3d-store.initializer-machine-bytes");
                        return semio_framework_job::StepOutcome::Yield;
                    }
                    self.phase = Process3dStoreInitializationPhase::Census;
                    return semio_framework_job::StepOutcome::Yield;
                };
                if let Some(operation) = entry.forwards.get(mutation) {
                    self.history_items = match self.history_items.checked_add(1) {
                        Some(value) if value <= PROCESS3D_MAXIMUM_DOMAIN_ITEMS => value,
                        _ => {
                            self.fail(b"process3d-store.initializer-history-capacity");
                            return semio_framework_job::StepOutcome::Yield;
                        }
                    };
                    if matches!(operation, Process3dMutation::CreateMachine(_)) {
                        self.machine_growth = match self.machine_growth.checked_add(1) {
                            Some(value) => value,
                            None => {
                                self.fail(b"process3d-store.initializer-machine-growth-overflow");
                                return semio_framework_job::StepOutcome::Yield;
                            }
                        };
                    }
                    self.phase = Process3dStoreInitializationPhase::CensusHistory { edit, mutation: mutation + 1 };
                } else {
                    self.phase = Process3dStoreInitializationPhase::CensusHistory { edit: edit + 1, mutation: 0 };
                }
            }
            Process3dStoreInitializationPhase::Census => {
                let source = &self.envelope.as_ref().expect("Process3d envelope retained").vcs.genesis.snapshot();
                match self.census.as_mut().expect("Process3d census retained").step(source, cx) {
                    Ok(true) => self.phase = Process3dStoreInitializationPhase::BindGenesis,
                    Ok(false) => {}
                    Err(code) => self.fail(code.as_bytes()),
                }
            }
            Process3dStoreInitializationPhase::CloneInitial => {
                let source = &self.envelope.as_ref().expect("Process3d envelope retained").vcs.genesis.snapshot();
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
                    match self.runtime.as_mut().expect("retained initializer runtime").adopt_current_owned(initial, std::sync::Arc::new(Process3dSnapshotRetirementFactory)) {
                        Ok(()) => self.phase = self.resume_phase.take().expect("retained mutation resume phase"),
                        Err(initial) => {
                            *self.active = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&Process3dSnapshotRetirementFactory, initial));
                            self.fail(b"initializer-owned-workspace-adoption");
                        }
                    }
                }
                return semio_framework_job::StepOutcome::Yield;
            }
            Process3dStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("Process3d history retained");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = Process3dStoreInitializationPhase::FoldSupersessions { transition: 0 };
                    return semio_framework_job::StepOutcome::Yield;
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
            Process3dStoreInitializationPhase::FindApplied { position } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self
                        .envelope
                        .as_ref()
                        .and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.as_ref()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| &checkpoint.id)))
                        .and_then(|id| process3d_copy_string(id).ok());
                    self.runtime.as_mut().expect("Process3d runtime retained").set_current_checkpoint_id(checkpoint);
                    self.phase = Process3dStoreInitializationPhase::FindRedo { position: 0 };
                    return semio_framework_job::StepOutcome::Yield;
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
            Process3dStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let needs_workspace = {
                    let envelope = self.envelope.as_ref().expect("retained initializer envelope");
                    let runtime = self.runtime.as_ref().expect("retained initializer runtime");
                    envelope.vcs.edits.get(edit).and_then(|entry| runtime.effective_forward(entry, mutation, &envelope.schema)).is_some_and(|effective| effective.operation().is_some())
                };
                if needs_workspace && self.runtime.as_mut().expect("retained initializer runtime").current_mut().is_none() {
                    self.resume_phase = Some(self.phase);
                    let capacity = self.envelope.as_ref().expect("retained genesis").vcs.genesis.snapshot().workshop.machines.len() + self.machine_growth;
                    *self.clone_cursor = Some(Process3dSnapshotCopyCursor::new(capacity));
                    self.phase = Process3dStoreInitializationPhase::CloneInitial;
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                let operation = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).and_then(|entry| entry.forwards.get(mutation));
                let Some(operation) = operation else {
                    self.phase = Process3dStoreInitializationPhase::CommitApplied { position, edit };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.as_ref().expect("Process3d envelope remains retained while its forwards fold");
                let entry = envelope.vcs.edits.get(edit).expect("Process3d applied edit remains retained");
                let effective = self.runtime.as_ref().and_then(|runtime| runtime.effective_forward(entry, mutation, &envelope.schema)).expect("Process3d applied forward remains retained");
                if effective.operation().is_none() {
                    drop(effective);
                    self.phase = Process3dStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                    return semio_framework_job::StepOutcome::Yield;
                }
                let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("Process3d runtime current retained");
                let applied = effective.operation().map(|operation| process3d_apply_retained_mutation(current, operation));
                drop(effective);
                match applied {
                    Some(Ok(retired)) => {
                        *self.active = retired;
                        self.phase = Process3dStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                    }
                    None => self.phase = Process3dStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 },
                    Some(Err(code)) => self.fail(code.as_bytes()),
                }
            }
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
            Process3dStoreInitializationPhase::FindRedo { position } => {
                let Some(id) = self.redo_id(position) else {
                    self.edit_index.clear();
                    self.phase = Process3dStoreInitializationPhase::BuildCandidate;
                    return semio_framework_job::StepOutcome::Yield;
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
            Process3dStoreInitializationPhase::BuildCandidate => {
                let authoritative = process3d_validate_publication_authority(self.operation, self.generation);
                let publication_fresh =
                    cx.operation() == self.operation && cx.generation() == self.generation && authoritative == Ok((self.base_revision, self.parent_revision)) && self.base_revision == self.parent_revision && self.parent_revision == self.generation.0;
                let Some(candidate_generation) = self.parent_revision.checked_add(1) else {
                    self.fail(b"process3d-store.initializer-generation-exhausted");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if !publication_fresh {
                    self.fail(b"process3d-store.initializer-parent-stale-aba");
                    return semio_framework_job::StepOutcome::Yield;
                }
                let envelope = self.envelope.take().expect("Process3d envelope retained until publication");
                let runtime = self.runtime.take().expect("Process3d runtime retained until publication");
                *self.candidate = Some(store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, process3d_document_store_owners()));
                self.phase = Process3dStoreInitializationPhase::Complete;
                return semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                });
            }
            Process3dStoreInitializationPhase::RetireCancelled | Process3dStoreInitializationPhase::RetireFault => match self.pump_terminal_retirement(PROCESS3D_OWNER_BYTES) {
                Ok(false) => return semio_framework_job::StepOutcome::Yield,
                Ok(true) => {
                    process3d_release_app_publication_authority(self.operation);
                    self.terminal_handoff = true;
                    if self.phase == Process3dStoreInitializationPhase::RetireCancelled {
                        self.phase = Process3dStoreInitializationPhase::Cancelled;
                        return semio_framework_job::StepOutcome::Cancelled;
                    }
                    self.phase = Process3dStoreInitializationPhase::Fault;
                    let bytes = self.fault.take().unwrap_or_else(|| process3d_fault_detail(b"process3d-store.initializer-fault"));
                    let detail = cx.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, &bytes).unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                    return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail });
                }
                Err(error) => self.fault = Some(error.into_message().into_bytes()),
            },
            Process3dStoreInitializationPhase::Complete => {
                return semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                });
            }
            Process3dStoreInitializationPhase::Cancelled => return semio_framework_job::StepOutcome::Cancelled,
            Process3dStoreInitializationPhase::Fault => {
                let bytes = self.fault.as_deref().unwrap_or(b"process3d-store.initializer-fault");
                let detail = cx.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, bytes).unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail });
            }
        }
        cx.consume_fuel(1);
        semio_framework_job::StepOutcome::Yield
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

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, semio_framework::Fault> {
        self.begin_close();
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        match self.pump_terminal_retirement(maximum_bytes.min(PROCESS3D_OWNER_BYTES)) {
            Ok(false) => Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }),
            Ok(true) => {
                process3d_release_app_publication_authority(self.operation);
                self.terminal_handoff = true;
                Ok(semio_framework_plugin::PluginCloseStep::Complete)
            }
            Err(error) => Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("artifact-store.initializer-close"), format!("Process3d initializer close failed: {}", error.into_message()))),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}

const PROCESS3D_OWNER_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

const PROCESS3D_MAXIMUM_DOMAIN_ITEMS: usize = 8_192;

const PROCESS3D_MAXIMUM_DOMAIN_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

pub const PROCESS3D_MOUNTED_OUTPUT_CHANNELS: usize = 4;

pub const PROCESS3D_MOUNTED_CONTROL_CREDITS: usize = 1;

const PROCESS3D_PUBLICATION_SLOTS: usize = 4;

fn process3d_publication_key(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> semio_framework_job::FixedOperationKey {
    semio_framework_job::FixedOperationKey::new(operation, generation)
}

/// 🛣️ The ONE lane every test that admits into the process-global publication registry takes first.
/// The lease table is a four-slot DIRECT-MAPPED registry shared by the whole test binary, so two
/// laws admitting concurrently (the suite runs multi-threaded) collide on a slot and report
/// `process3d-publication.saturated` — a scheduling artifact of the fixture, never the property under
/// test. Poisoning is absorbed: a lane is a scheduling device, and the panic that poisoned it is
/// already the failure being reported.
///
/// 🧹️ Taking the lane also DRAINS whatever the previous lane holder left behind. A law that panics
/// between its `admit` and its `release` (the exact shape of a failing assertion) strands a lease in
/// a four-slot table that nothing can enumerate, so the NEXT laws report
/// `process3d-publication.saturated` and one real defect is reported as three or four. Every host
/// admission in this binary happens under this lane (`🔬️retained-laws`, the editor's production
/// envelope fixture, the mounted registry laws), so draining here can never take a lease a
/// concurrently running law still owns.
#[cfg(test)]
pub fn process3d_publication_authority_lane() -> std::sync::MutexGuard<'static, ()> {
    static LANE: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let guard = LANE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    // 🔒️ Leases BEFORE admissions — the same order `process3d_admit_publication_authority` and
    // `process3d_release_publication_authority` take them in, so the drain can never invert a lock pair.
    if let Ok(mut leases) = process3d_publication_leases().try_lock() {
        let mut admitted = process3d_publication_test_admissions().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for key in admitted.drain(..) {
            leases.take(key);
        }
    }
    guard
}

/// 🧾️ Every key this test binary admitted into the process-global host table and has not released —
/// the enumeration `FixedOperationRegistry` does not offer, kept only so the lane above can drain a
/// stranded lease.
#[cfg(test)]
fn process3d_publication_test_admissions() -> &'static std::sync::Mutex<Vec<semio_framework_job::FixedOperationKey>> {
    static ADMITTED: std::sync::OnceLock<std::sync::Mutex<Vec<semio_framework_job::FixedOperationKey>>> = std::sync::OnceLock::new();
    ADMITTED.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

/// 🧮️ Domain item, output page, and control limits admitted for one publication.
#[derive(Clone, Copy)]
pub struct Process3dPublicationLimits {
    pub maximum_items: usize,
    pub maximum_output_pages: usize,
    pub maximum_controls: usize,
}

/// 🔐️ Releases the app-admitted lease of `operation` (a host-admitted one is the host's to release).
pub fn process3d_release_app_publication_authority(operation: semio_framework_job::OperationId) -> bool {
    let Ok(mut app) = process3d_app_publication_lease().try_lock() else { return false };
    if app.as_ref().is_none_or(|(_, lease)| lease.operation != operation.0) {
        return false;
    }
    app.take().is_some()
}

pub fn process3d_refresh_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_revision: u64) -> Result<(), &'static str> {
    let key = process3d_publication_key(operation, generation);
    let mut leases = process3d_publication_leases().try_lock().map_err(|_| "process3d-publication.contended")?;
    if let Some(lease) = leases.get_mut(key) {
        lease.live_revision = live_revision;
        return Ok(());
    }
    drop(leases);
    let mut app = process3d_app_publication_lease().try_lock().map_err(|_| "process3d-publication.contended")?;
    let Some((_, lease)) = app.as_mut().filter(|(held, _)| *held == key) else { return Err("process3d-publication.stale-authority") };
    lease.live_revision = live_revision;
    Ok(())
}

pub fn process3d_validate_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<(u64, u64), &'static str> {
    let lease = process3d_publication_lease_by_key(process3d_publication_key(operation, generation))?.ok_or("process3d-publication.stale-authority")?;
    if lease.generation != generation.0 || lease.live_revision != generation.0 || lease.base_revision != lease.live_revision || lease.parent_revision != lease.base_revision {
        return Err("process3d-publication.stale-aba-parent");
    }
    Ok((lease.base_revision, lease.parent_revision))
}

/// 🔐️ Fail-closed Process3d authority used by the shared atomic replacement branch.
pub fn process3d_validate_atomic_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), &'static str> {
    let lease = process3d_publication_lease_by_operation(operation)?.ok_or("process3d-publication.authority-missing")?;
    #[cfg(test)]
    let lease = {
        let mut lease = lease;
        let mut hostiles = process3d_publication_hostiles().try_lock().map_err(|_| "process3d-publication.hostile-contended")?;
        if let Some(hostile) = hostiles.iter_mut().flatten().find(|value| value.operation == operation.0) {
            hostile.observed = Some(match hostile.hostile {
                Process3dPublicationHostile::Missing => "process3d-publication.authority-missing",
                Process3dPublicationHostile::WrongOperation => "process3d-publication.wrong-operation",
                Process3dPublicationHostile::WrongGeneration => "process3d-publication.wrong-generation",
                Process3dPublicationHostile::WrongBase => "process3d-publication.wrong-base",
                Process3dPublicationHostile::WrongParent => "process3d-publication.wrong-parent",
            });
            match hostile.hostile {
                Process3dPublicationHostile::Missing => return Err("process3d-publication.authority-missing"),
                Process3dPublicationHostile::WrongOperation => lease.operation = lease.operation.wrapping_add(1),
                Process3dPublicationHostile::WrongGeneration => lease.generation = lease.generation.wrapping_add(1),
                Process3dPublicationHostile::WrongBase => lease.base_revision = lease.base_revision.wrapping_add(1),
                Process3dPublicationHostile::WrongParent => lease.parent_revision = lease.parent_revision.wrapping_add(1),
            }
        }
        lease
    };
    process3d_validate_atomic_lease(lease, operation, generation, live_generation)
}

pub fn process3d_publication_item_credit(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<usize, &'static str> {
    let lease = process3d_publication_lease_by_key(process3d_publication_key(operation, generation))?.ok_or("process3d-publication.stale-authority")?;
    if lease.maximum_output_pages != PROCESS3D_MOUNTED_OUTPUT_CHANNELS || lease.maximum_controls != PROCESS3D_MOUNTED_CONTROL_CREDITS {
        return Err("process3d-publication.domain-credits-lost");
    }
    Ok(lease.maximum_items)
}

pub fn process3d_release_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> bool {
    let Ok(mut leases) = process3d_publication_leases().try_lock() else { return false };
    let key = process3d_publication_key(operation, generation);
    #[cfg(test)]
    process3d_publication_test_admissions().lock().unwrap_or_else(std::sync::PoisonError::into_inner).retain(|held| *held != key);
    leases.take(key).is_some()
}

pub struct Process3dSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<Process3dSnapshot> for Process3dSnapshotRetirementFactory {
    fn retire_owned(&self, value: Process3dSnapshot) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(Process3dOwnedRetirement::snapshot(value))
    }
}

impl store::SnapshotRetirementFactory<Process3dSnapshot> for Process3dSnapshotRetirementFactory {
    fn retire(&self, snapshot: std::sync::Arc<Process3dSnapshot>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(Process3dSnapshotRootRetirement { owner: std::mem::ManuallyDrop::new(Some(snapshot)), retirement: std::mem::ManuallyDrop::new(None), terminal: false })
    }
}

pub struct Process3dMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<Process3dMutation> for Process3dMutationRetirementFactory {
    fn retire_owned(&self, value: Process3dMutation) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(Process3dOwnedRetirement::mutation(value))
    }
}

fn process3d_copy_string(source: &str) -> Result<String, &'static str> {
    if source.len() > PROCESS3D_OWNER_BYTES {
        return Err("process3d-owner.string-capacity");
    }
    let mut value = String::with_capacity(source.len());
    if value.capacity() > PROCESS3D_OWNER_BYTES {
        return Err("process3d-owner.string-observed-capacity");
    }
    value.push_str(source);
    Ok(value)
}

struct Process3dOwnerCensusCursor {
    phase: Process3dCensusPhase,
    totals: Process3dOwnerTotals,
}

impl Process3dOwnerCensusCursor {
    fn new() -> Self {
        Self { phase: Process3dCensusPhase::Root, totals: Process3dOwnerTotals::default() }
    }

    fn string_bytes(strings: &[&String]) -> Result<usize, &'static str> {
        let mut bytes = 0usize;
        for value in strings {
            bytes = bytes.checked_add(value.capacity()).ok_or("process3d-owner.bytes-overflow")?;
        }
        Ok(bytes)
    }

    fn machine(&mut self, machine: &WorkshopMachine) -> Result<(), &'static str> {
        let mut items = 4usize;
        let mut bytes = Self::string_bytes(&[&machine.id, &machine.label, &machine.icon_id])?;
        if let Some(catalog_id) = &machine.catalog_id {
            items += 1;
            bytes = bytes.checked_add(catalog_id.capacity()).ok_or("process3d-owner.bytes-overflow")?;
        }
        bytes = bytes.checked_add(machine.capabilities.capacity().saturating_mul(size_of::<Capability>())).ok_or("process3d-owner.bytes-overflow")?;
        for capability in &machine.capabilities {
            items = items.checked_add(4 + capability.parameters.len() * 3 + capability.rules.len() * 2).ok_or("process3d-owner.items-overflow")?;
            bytes = bytes.checked_add(Self::string_bytes(&[&capability.id, &capability.label, &capability.icon_id])?).ok_or("process3d-owner.bytes-overflow")?;
            bytes = bytes.checked_add(capability.parameters.capacity().saturating_mul(size_of::<CapabilityParameter>())).ok_or("process3d-owner.bytes-overflow")?;
            bytes = bytes.checked_add(capability.rules.capacity().saturating_mul(size_of::<CapabilityRule>())).ok_or("process3d-owner.bytes-overflow")?;
            for parameter in &capability.parameters {
                bytes = bytes.checked_add(Self::string_bytes(&[&parameter.id, &parameter.label])?).ok_or("process3d-owner.bytes-overflow")?;
            }
            for rule in &capability.rules {
                let parameter = match rule {
                    CapabilityRule::Min { parameter, .. } | CapabilityRule::Max { parameter, .. } => parameter,
                };
                bytes = bytes.checked_add(parameter.capacity()).ok_or("process3d-owner.bytes-overflow")?;
            }
        }
        self.totals.admit(items, bytes, 8)
    }

    fn step(&mut self, source: &Process3dSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(false);
        }
        match self.phase {
            Process3dCensusPhase::Root => {
                let bytes = Self::string_bytes(&[&source.stock_id, &source.stock_label])?
                    .checked_add(source.workshop.machines.capacity().saturating_mul(size_of::<WorkshopMachine>()))
                    .and_then(|value| value.checked_add(source.step_payloads.capacity().saturating_mul(size_of::<ProcessStep>())))
                    .and_then(|value| value.checked_add(source.tool_solids.capacity().saturating_mul(size_of_val(&process3d_empty_child::<semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot>()))))
                    .ok_or("process3d-owner.bytes-overflow")?;
                self.totals.admit(10, bytes, 4)?;
                self.totals.output_pages = PROCESS3D_MAXIMUM_OUTPUT_PAGES;
                self.totals.controls = PROCESS3D_RETAINED_STACK_CAPACITY;
                self.phase = Process3dCensusPhase::Machines(0);
            }
            Process3dCensusPhase::Machines(index) => {
                if let Some(machine) = source.workshop.machines.get(index) {
                    self.machine(machine)?;
                    self.phase = Process3dCensusPhase::Machines(index + 1);
                } else {
                    self.phase = Process3dCensusPhase::Steps(0);
                }
            }
            Process3dCensusPhase::Steps(index) => {
                if let Some(step) = source.step_payloads.get(index) {
                    let mut bytes = Self::string_bytes(&[&step.id, &step.label])?;
                    if let Some(origin) = &step.origin {
                        bytes = bytes.checked_add(Self::string_bytes(&[&origin.machine_id, &origin.capability_id])?).ok_or("process3d-owner.bytes-overflow")?;
                    }
                    self.totals.admit(6, bytes, 7)?;
                    self.phase = Process3dCensusPhase::Steps(index + 1);
                } else {
                    self.phase = Process3dCensusPhase::Tools(0);
                }
            }
            Process3dCensusPhase::Tools(index) => {
                if let Some(child) = source.tool_solids.get(index) {
                    let bytes = Self::string_bytes(&[&child.child_id, &child.target.artifact_id, &child.target.dialect.artifact_kind, &child.target.dialect.standard, &child.target.dialect.subset])?;
                    self.totals.admit(5, bytes, 5)?;
                    self.phase = Process3dCensusPhase::Tools(index + 1);
                } else {
                    self.phase = Process3dCensusPhase::Complete;
                }
            }
            Process3dCensusPhase::Complete => return Ok(true),
        }
        cx.consume_fuel(1);
        Ok(self.phase == Process3dCensusPhase::Complete)
    }
}

struct Process3dSnapshotCopyCursor {
    phase: Process3dCopyPhase,
    machine_capacity: usize,
    candidate: std::mem::ManuallyDrop<Option<Process3dSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    terminal_handoff: bool,
}

impl Process3dSnapshotCopyCursor {
    fn new(machine_capacity: usize) -> Self {
        Self { phase: Process3dCopyPhase::Shell, machine_capacity, candidate: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None), terminal_handoff: false }
    }

    fn step(&mut self, source: &Process3dSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(false);
        }
        match self.phase {
            Process3dCopyPhase::Shell => {
                let mut machines = Vec::with_capacity(self.machine_capacity);
                let step_payloads = Vec::with_capacity(source.step_payloads.len());
                let tool_solids = Vec::with_capacity(source.tool_solids.len());
                if machines.capacity() > PROCESS3D_MAXIMUM_DOMAIN_ITEMS || step_payloads.capacity() > PROCESS3D_MAXIMUM_DOMAIN_ITEMS || tool_solids.capacity() > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
                    return Err("process3d-owner.clone-observed-capacity");
                }
                machines.clear();
                *self.candidate = Some(Process3dSnapshot {
                    workshop: crate::Workshop { machines },
                    stock_id: process3d_copy_string(&source.stock_id)?,
                    stock_label: process3d_copy_string(&source.stock_label)?,
                    stock_pose: process3d_copy_pose(&source.stock_pose),
                    stock_payload: process3d_copy_stock(&source.stock_payload)?,
                    stock_solid: process3d_copy_child(&source.stock_solid)?,
                    steps: process3d_copy_child(&source.steps)?,
                    step_payloads,
                    tool_solids,
                });
                self.phase = Process3dCopyPhase::Machines(0);
            }
            Process3dCopyPhase::Machines(index) => {
                if let Some(machine) = source.workshop.machines.get(index) {
                    self.candidate.as_mut().expect("Process3d clone shell remains retained").workshop.machines.push(process3d_copy_machine(machine)?);
                    self.phase = Process3dCopyPhase::Machines(index + 1);
                } else {
                    self.phase = Process3dCopyPhase::Steps(0);
                }
            }
            Process3dCopyPhase::Steps(index) => {
                if let Some(step) = source.step_payloads.get(index) {
                    self.candidate.as_mut().expect("Process3d clone shell remains retained").step_payloads.push(process3d_copy_step(step)?);
                    self.phase = Process3dCopyPhase::Steps(index + 1);
                } else {
                    self.phase = Process3dCopyPhase::Tools(0);
                }
            }
            Process3dCopyPhase::Tools(index) => {
                if let Some(child) = source.tool_solids.get(index) {
                    self.candidate.as_mut().expect("Process3d clone shell remains retained").tool_solids.push(process3d_copy_child(child)?);
                    self.phase = Process3dCopyPhase::Tools(index + 1);
                } else {
                    self.phase = Process3dCopyPhase::Complete;
                }
            }
            Process3dCopyPhase::Complete => return Ok(true),
        }
        cx.consume_fuel(1);
        Ok(self.phase == Process3dCopyPhase::Complete)
    }

    fn take(&mut self) -> Option<Process3dSnapshot> {
        if self.phase != Process3dCopyPhase::Complete || self.terminal_handoff {
            return None;
        }
        let value = self.candidate.take()?;
        self.terminal_handoff = true;
        Some(value)
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(candidate) = self.candidate.take() {
                *self.retirement = Some(Box::new(Process3dOwnedRetirement::snapshot(candidate)));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.terminal_handoff = true;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let retirement = self.retirement.as_mut().expect("Process3d clone retirement remains retained");
        match retirement.close_step(1, maximum_bytes)? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                self.terminal_handoff = true;
                Ok(store::SnapshotRetirementStep::Complete)
            }
            store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d clone retirement false terminal")),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_handoff && self.candidate.is_none() && self.retirement.is_none()
    }
}

impl Drop for Process3dSnapshotCopyCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Process3d snapshot copy reached Drop before candidate handoff or retained close");
    }
}

fn process3d_fault_detail(code: &[u8]) -> Vec<u8> {
    assert!(code.len() <= PROCESS3D_OWNER_BYTES, "Process3d fault detail exceeds its fixed admission");
    let mut detail = Vec::new();
    detail.try_reserve_exact(code.len()).expect("Process3d fixed fault detail was pre-admitted");
    detail.extend_from_slice(code);
    detail
}

fn process3d_apply_retained_mutation(snapshot: &mut Process3dSnapshot, mutation: &Process3dMutation) -> Result<Option<Box<dyn store::ErasedSnapshotRetirement>>, &'static str> {
    use Process3dMutation::*;
    let retired = match mutation {
        CreateStep(_) | DeleteStep(_) | RenameStep(_) | ChangeStepEnabled(_) | ChangeStepOrigin(_) | ReplaceStepMeasure(_) | ReorderSteps(_) => None,
        CreateMachine(value) => {
            if snapshot.workshop.machines.iter().any(|machine| machine.id == value.machine.id) {
                return Err("process3d-store.duplicate-machine");
            }
            if snapshot.workshop.machines.len() == snapshot.workshop.machines.capacity() {
                return Err("process3d-store.machine-capacity");
            }
            snapshot.workshop.machines.push(process3d_copy_machine(&value.machine)?);
            None
        }
        DeleteMachine(value) => {
            let index = snapshot.workshop.machines.iter().position(|machine| machine.id == value.id).ok_or("process3d-store.machine-missing")?;
            let old = snapshot.workshop.machines.remove(index);
            Some(Box::new(Process3dOwnedRetirement::owner(Process3dRetirementOwner::Machine { value: old, phase: 0 })) as Box<dyn store::ErasedSnapshotRetirement>)
        }
        RenameMachine(value) => {
            let machine = snapshot.workshop.machines.iter_mut().find(|machine| machine.id == value.id).ok_or("process3d-store.machine-missing")?;
            let old = std::mem::replace(&mut machine.label, process3d_copy_string(&value.new_label)?);
            Some(Box::new(Process3dOwnedRetirement::owner(Process3dRetirementStack::one_string(old))) as Box<dyn store::ErasedSnapshotRetirement>)
        }
        ChangeMachineIcon(value) => {
            let machine = snapshot.workshop.machines.iter_mut().find(|machine| machine.id == value.id).ok_or("process3d-store.machine-missing")?;
            let old = std::mem::replace(&mut machine.icon_id, process3d_copy_string(&value.new_icon_id)?);
            Some(Box::new(Process3dOwnedRetirement::owner(Process3dRetirementStack::one_string(old))) as Box<dyn store::ErasedSnapshotRetirement>)
        }
        ReplaceMachineCapabilities(value) => {
            let machine = snapshot.workshop.machines.iter_mut().find(|machine| machine.id == value.id).ok_or("process3d-store.machine-missing")?;
            let mut next = Vec::with_capacity(value.new_capabilities.len());
            if next.capacity() > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
                return Err("process3d-store.capability-capacity");
            }
            for capability in &value.new_capabilities {
                next.push(process3d_copy_capability(capability)?);
            }
            let old = std::mem::replace(&mut machine.capabilities, next);
            Some(Box::new(Process3dOwnedRetirement::owner(Process3dRetirementOwner::Capabilities { values: old })) as Box<dyn store::ErasedSnapshotRetirement>)
        }
        MoveStock(value) => {
            if !value.new_pose.position.iter().chain(value.new_pose.axis.iter()).chain(std::iter::once(&value.new_pose.angle)).all(|scalar| scalar.is_finite()) {
                return Err("process3d-store.stock-pose-nonfinite");
            }
            snapshot.stock_pose = process3d_copy_pose(&value.new_pose);
            None
        }
        ChangeStockLabel(value) => {
            let old = std::mem::replace(&mut snapshot.stock_label, process3d_copy_string(&value.new_label)?);
            Some(Box::new(Process3dOwnedRetirement::owner(Process3dRetirementStack::one_string(old))) as Box<dyn store::ErasedSnapshotRetirement>)
        }
        ReplaceStockSolid(value) => {
            let old = std::mem::replace(&mut snapshot.stock_solid, process3d_copy_child(&value.new_solid)?);
            Some(Box::new(Process3dOwnedRetirement::owner(Process3dRetirementOwner::Child { value: Process3dChildParts::from_child(old), phase: 0 })) as Box<dyn store::ErasedSnapshotRetirement>)
        }
    };
    Ok(retired)
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
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize },
    CommitRedo { position: usize, edit: usize },
    BuildCandidate,
    Complete,
    RetireCancelled,
    RetireFault,
    Cancelled,
    Fault,
}

impl Process3dStoreInitializationAuthority {
    fn new(envelope: store::ArtifactEnvelope<Process3dSnapshot, Process3dMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Self {
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
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            active_terminal: false,
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            clone_cursor: std::mem::ManuallyDrop::new(None),
            census: std::mem::ManuallyDrop::new(Some(Process3dOwnerCensusCursor::new())),
            candidate_disposer: std::mem::ManuallyDrop::new(None),
            edit_index: store::ArtifactStoreInitializationEditIndex::default(),
            resume_phase: None,
            phase: Process3dStoreInitializationPhase::ValidateEnvelope,
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
        self.fault = Some(process3d_fault_detail(code));
        self.phase = Process3dStoreInitializationPhase::RetireFault;
    }

    fn pump_active(&mut self) -> Result<bool, semio_framework_value::ValueError> {
        if self.active_terminal {
            drop(self.active.take());
            self.active_terminal = false;
            return Ok(true);
        }
        let Some(active) = self.active.as_mut() else { return Ok(false) };
        match active.close_step(1, PROCESS3D_OWNER_BYTES)? {
            store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => self.active_terminal = true,
            store::SnapshotRetirementStep::Complete => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d initializer active false terminal")),
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= PROCESS3D_OWNER_BYTES => {}
            store::SnapshotRetirementStep::Pending { .. } => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d initializer active exceeded grant")),
            store::SnapshotRetirementStep::Blocked => {}
        }
        Ok(true)
    }

    fn pump_terminal_retirement(&mut self, maximum_bytes: usize) -> Result<bool, semio_framework_value::ValueError> {
        if self.pump_active()? {
            return Ok(false);
        }
        if let Some(candidate) = self.candidate.as_mut() {
            use semio_framework_plugin::ArtifactOwnedDisposer;
            if self.candidate_disposer.is_none() {
                *self.candidate_disposer = Some(semio_framework_plugin::ArtifactDocumentStoreDisposer::new());
                return Ok(false);
            }
            let disposer = self.candidate_disposer.as_mut().expect("Process3d candidate disposer retained");
            return match disposer.close_step(candidate, 1, maximum_bytes).map_err(|_| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d candidate disposer fault"))? {
                semio_framework_plugin::PluginCloseStep::Complete if disposer.terminal_is_empty(candidate) => {
                    *self.candidate_disposer = None;
                    drop(self.candidate.take());
                    Ok(false)
                }
                semio_framework_plugin::PluginCloseStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d candidate disposer false terminal")),
                _ => Ok(false),
            };
        }
        if let Some(runtime) = self.runtime.as_mut() {
            return match runtime.close_step(&Process3dSnapshotRetirementFactory, 1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    drop(self.runtime.take());
                    Ok(false)
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d runtime false terminal")),
                _ => Ok(false),
            };
        }
        if let Some(cursor) = self.clone_cursor.as_mut() {
            return match cursor.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if cursor.terminal_is_empty() => {
                    drop(self.clone_cursor.take());
                    Ok(false)
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d clone false terminal")),
                _ => Ok(false),
            };
        }
        *self.census = None;
        if self.envelope_retirement.is_none() {
            if let Some(envelope) = self.envelope.take() {
                *self.envelope_retirement = Some(process3d_envelope_decode_owner_bundle().retire_envelope(envelope));
                return Ok(false);
            }
        }
        if let Some(retirement) = self.envelope_retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.envelope_retirement.take());
                    Ok(false)
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d envelope retirement false terminal")),
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
            && self.clone_cursor.is_none()
            && self.census.is_none()
            && self.candidate_disposer.is_none()
    }
}

impl Drop for Process3dStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty_inner(), "Process3d initializer reached Drop before candidate handoff or terminal-empty close");
    }
}

pub fn process3d_document_store_initialization_job(
    envelope: store::ArtifactEnvelope<Process3dSnapshot, Process3dMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    actor: protocol::ActorId,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<Process3dSnapshot, Process3dMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(Process3dStoreInitializationAuthority::new(envelope, operation, generation, actor)))
}

const PROCESS3D_RETAINED_STACK_CAPACITY: usize = 64;

const PROCESS3D_MAXIMUM_OUTPUT_PAGES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES;

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Process3dPublicationHostile {
    Missing,
    WrongOperation,
    WrongGeneration,
    WrongBase,
    WrongParent,
}

#[cfg(test)]
fn process3d_publication_hostiles() -> &'static std::sync::Mutex<[Option<Process3dPublicationHostileLease>; PROCESS3D_PUBLICATION_SLOTS]> {
    static HOSTILES: std::sync::OnceLock<std::sync::Mutex<[Option<Process3dPublicationHostileLease>; PROCESS3D_PUBLICATION_SLOTS]>> = std::sync::OnceLock::new();
    HOSTILES.get_or_init(|| std::sync::Mutex::new([None; PROCESS3D_PUBLICATION_SLOTS]))
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Process3dOwnerTotals {
    pub items: usize,
    pub bytes: usize,
    pub output_pages: usize,
    pub controls: usize,
    pub combined_depth: usize,
}

impl Process3dOwnerTotals {
    fn admit(&mut self, items: usize, bytes: usize, depth: usize) -> Result<(), &'static str> {
        self.items = self.items.checked_add(items).ok_or("process3d-owner.items-overflow")?;
        self.bytes = self.bytes.checked_add(bytes).ok_or("process3d-owner.bytes-overflow")?;
        self.combined_depth = self.combined_depth.max(depth);
        if self.items > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
            return Err("process3d-owner.items-capacity");
        }
        if self.bytes > PROCESS3D_MAXIMUM_DOMAIN_BYTES {
            return Err("process3d-owner.bytes-capacity");
        }
        if self.combined_depth >= PROCESS3D_RETAINED_STACK_CAPACITY {
            return Err("process3d-owner.combined-depth");
        }
        Ok(())
    }
}

struct Process3dChildParts {
    strings: [Option<String>; 5],
}

impl Process3dChildParts {
    fn from_child<S>(child: store::ArtifactChild<S>) -> Self {
        Self { strings: [Some(child.child_id), Some(child.target.artifact_id), Some(child.target.dialect.artifact_kind), Some(child.target.dialect.standard), Some(child.target.dialect.subset)] }
    }
}

fn process3d_empty_child<S>() -> store::ArtifactChild<S> {
    store::ArtifactChild::new(String::new(), semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } })
}

enum Process3dRetirementOwner {
    Snapshot { value: Process3dSnapshot, phase: u8 },
    Machine { value: WorkshopMachine, phase: u8 },
    Capability { value: Capability, phase: u8 },
    Parameter { value: CapabilityParameter },
    Step { value: ProcessStep, phase: u8 },
    Origin { value: StepOrigin },
    Stock { value: Stock, phase: u8 },
    Measure { value: ProcessMeasure },
    Solid { value: WorkingSolid },
    Child { value: Process3dChildParts, phase: usize },
    Strings { values: [Option<String>; 6], phase: usize },
    MutationFields { value: Process3dMutationFields, phase: u8 },
    Capabilities { values: Vec<Capability> },
}

struct Process3dRetirementStack {
    slots: std::mem::ManuallyDrop<[Option<Process3dRetirementOwner>; PROCESS3D_RETAINED_STACK_CAPACITY]>,
    len: usize,
}

impl Process3dRetirementStack {
    fn new(owner: Process3dRetirementOwner) -> Self {
        let mut slots = std::array::from_fn(|_| None);
        slots[0] = Some(owner);
        Self { slots: std::mem::ManuallyDrop::new(slots), len: 1 }
    }

    fn push(&mut self, owner: Process3dRetirementOwner) -> Result<(), semio_framework_value::ValueError> {
        if self.len >= PROCESS3D_RETAINED_STACK_CAPACITY {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d combined retirement depth exceeded its fixed authority"));
        }
        self.slots[self.len] = Some(owner);
        self.len += 1;
        Ok(())
    }

    fn string(values: [Option<String>; 6]) -> Process3dRetirementOwner {
        Process3dRetirementOwner::Strings { values, phase: 0 }
    }

    fn one_string(value: String) -> Process3dRetirementOwner {
        Self::string([Some(value), None, None, None, None, None])
    }

    fn release_string(value: String, maximum_bytes: usize) -> Result<(usize, usize), semio_framework_value::ValueError> {
        let bytes = value.capacity();
        if bytes > maximum_bytes {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d string owner exceeds one close byte grant"));
        }
        drop(value);
        Ok((1, bytes))
    }

    fn pop_owner(&mut self) -> Option<Process3dRetirementOwner> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        self.slots[self.len].take()
    }

    fn advance(&mut self, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        let Some(owner) = self.pop_owner() else { return Ok(store::SnapshotRetirementStep::Complete) };
        let mut parent = None;
        let mut child = None;
        let mut released_items = 0;
        let mut released_bytes = 0;
        match owner {
            Process3dRetirementOwner::Snapshot { mut value, phase } => match phase {
                0 if !value.tool_solids.is_empty() => {
                    child = value.tool_solids.pop().map(Process3dChildParts::from_child).map(|value| Process3dRetirementOwner::Child { value, phase: 0 });
                    parent = Some(Process3dRetirementOwner::Snapshot { value, phase });
                }
                0 => {
                    let backing = std::mem::take(&mut value.tool_solids);
                    released_bytes = backing.capacity().saturating_mul(size_of_val(&process3d_empty_child::<semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot>()));
                    drop(backing);
                    released_items = 1;
                    parent = Some(Process3dRetirementOwner::Snapshot { value, phase: 1 });
                }
                1 if !value.step_payloads.is_empty() => {
                    child = value.step_payloads.pop().map(|value| Process3dRetirementOwner::Step { value, phase: 0 });
                    parent = Some(Process3dRetirementOwner::Snapshot { value, phase });
                }
                1 => {
                    let backing = std::mem::take(&mut value.step_payloads);
                    released_bytes = backing.capacity().saturating_mul(size_of::<ProcessStep>());
                    drop(backing);
                    released_items = 1;
                    parent = Some(Process3dRetirementOwner::Snapshot { value, phase: 2 });
                }
                2 if !value.workshop.machines.is_empty() => {
                    child = value.workshop.machines.pop().map(|value| Process3dRetirementOwner::Machine { value, phase: 0 });
                    parent = Some(Process3dRetirementOwner::Snapshot { value, phase });
                }
                2 => {
                    let backing = std::mem::take(&mut value.workshop.machines);
                    released_bytes = backing.capacity().saturating_mul(size_of::<WorkshopMachine>());
                    drop(backing);
                    released_items = 1;
                    parent = Some(Process3dRetirementOwner::Snapshot { value, phase: 3 });
                }
                3 => {
                    child = Some(Self::string([Some(std::mem::take(&mut value.stock_id)), Some(std::mem::take(&mut value.stock_label)), None, None, None, None]));
                    parent = Some(Process3dRetirementOwner::Snapshot { value, phase: 4 });
                }
                4 => {
                    child = Some(Process3dRetirementOwner::Stock { value: std::mem::take(&mut value.stock_payload), phase: 0 });
                    parent = Some(Process3dRetirementOwner::Snapshot { value, phase: 5 });
                }
                5 => {
                    child = Some(Process3dRetirementOwner::Child { value: Process3dChildParts::from_child(process3d_take_child(&mut value.stock_solid)), phase: 0 });
                    parent = Some(Process3dRetirementOwner::Snapshot { value, phase: 6 });
                }
                6 => {
                    child = Some(Process3dRetirementOwner::Child { value: Process3dChildParts::from_child(process3d_take_child(&mut value.steps)), phase: 0 });
                    parent = Some(Process3dRetirementOwner::Snapshot { value, phase: 7 });
                }
                _ => released_items = 1,
            },
            Process3dRetirementOwner::Machine { mut value, phase } => match phase {
                0 if !value.capabilities.is_empty() => {
                    child = value.capabilities.pop().map(|value| Process3dRetirementOwner::Capability { value, phase: 0 });
                    parent = Some(Process3dRetirementOwner::Machine { value, phase });
                }
                0 => {
                    let backing = std::mem::take(&mut value.capabilities);
                    released_bytes = backing.capacity().saturating_mul(size_of::<Capability>());
                    drop(backing);
                    released_items = 1;
                    parent = Some(Process3dRetirementOwner::Machine { value, phase: 1 });
                }
                _ => {
                    child = Some(Self::string([Some(value.id), Some(value.label), Some(value.icon_id), value.catalog_id, None, None]));
                    released_items = 1;
                }
            },
            Process3dRetirementOwner::Capability { mut value, phase } => match phase {
                0 if !value.rules.is_empty() => {
                    let rule = value.rules.pop().expect("Process3d retained rule exists");
                    let parameter = match rule {
                        CapabilityRule::Min { parameter, .. } | CapabilityRule::Max { parameter, .. } => parameter,
                    };
                    child = Some(Self::one_string(parameter));
                    parent = Some(Process3dRetirementOwner::Capability { value, phase });
                }
                0 => {
                    let backing = std::mem::take(&mut value.rules);
                    released_bytes = backing.capacity().saturating_mul(size_of::<CapabilityRule>());
                    drop(backing);
                    released_items = 1;
                    parent = Some(Process3dRetirementOwner::Capability { value, phase: 1 });
                }
                1 if !value.parameters.is_empty() => {
                    child = value.parameters.pop().map(|value| Process3dRetirementOwner::Parameter { value });
                    parent = Some(Process3dRetirementOwner::Capability { value, phase });
                }
                1 => {
                    let backing = std::mem::take(&mut value.parameters);
                    released_bytes = backing.capacity().saturating_mul(size_of::<CapabilityParameter>());
                    drop(backing);
                    released_items = 1;
                    parent = Some(Process3dRetirementOwner::Capability { value, phase: 2 });
                }
                2 => {
                    let strings = match value.recipe {
                        MeasureRecipe::DiscCut { diameter, kerf } => [Some(diameter), Some(kerf), None, None, None, None],
                        MeasureRecipe::BladeCut { kerf, length, depth } => [Some(kerf), Some(length), Some(depth), None, None, None],
                        MeasureRecipe::PocketCut { diameter, depth } | MeasureRecipe::BoreDrill { radius: diameter, depth } => [Some(diameter), Some(depth), None, None, None, None],
                        MeasureRecipe::CylinderAttach { radius, length } => [Some(radius), Some(length), None, None, None, None],
                        MeasureRecipe::BoxAttach { width, depth, height } => [Some(width), Some(depth), Some(height), None, None, None],
                    };
                    child = Some(Self::string(strings));
                    parent = Some(Process3dRetirementOwner::Capability {
                        value: Capability { id: value.id, label: value.label, icon_id: value.icon_id, recipe: MeasureRecipe::DiscCut { diameter: String::new(), kerf: String::new() }, parameters: value.parameters, rules: value.rules },
                        phase: 3,
                    });
                }
                _ => {
                    child = Some(Self::string([Some(value.id), Some(value.label), Some(value.icon_id), None, None, None]));
                    released_items = 1;
                }
            },
            Process3dRetirementOwner::Parameter { value } => {
                child = Some(Self::string([Some(value.id), Some(value.label), None, None, None, None]));
                released_items = 1;
            }
            Process3dRetirementOwner::Step { mut value, phase } => match phase {
                0 => {
                    if let Some(origin) = value.origin.take() {
                        child = Some(Process3dRetirementOwner::Origin { value: origin });
                    }
                    parent = Some(Process3dRetirementOwner::Step { value, phase: 1 });
                }
                1 => {
                    let measure = std::mem::replace(&mut value.measure, ProcessMeasure::Drill { radius: 0.0, depth: 0.0, pose: Default::default() });
                    child = Some(Process3dRetirementOwner::Measure { value: measure });
                    parent = Some(Process3dRetirementOwner::Step { value, phase: 2 });
                }
                _ => {
                    child = Some(Self::string([Some(value.id), Some(value.label), None, None, None, None]));
                    released_items = 1;
                }
            },
            Process3dRetirementOwner::Origin { value } => {
                child = Some(Self::string([Some(value.machine_id), Some(value.capability_id), None, None, None, None]));
                released_items = 1;
            }
            Process3dRetirementOwner::Stock { mut value, phase } => match phase {
                0 => {
                    child = Some(Process3dRetirementOwner::Solid { value: std::mem::take(&mut value.solid) });
                    parent = Some(Process3dRetirementOwner::Stock { value, phase: 1 });
                }
                _ => {
                    child = Some(Self::string([Some(value.id), Some(value.label), None, None, None, None]));
                    released_items = 1;
                }
            },
            Process3dRetirementOwner::Measure { value } => match value {
                ProcessMeasure::Cut { tool, .. } => child = Some(Process3dRetirementOwner::Solid { value: tool }),
                ProcessMeasure::Attach { component, .. } => child = Some(Process3dRetirementOwner::Solid { value: component }),
                ProcessMeasure::Drill { .. } => released_items = 1,
            },
            Process3dRetirementOwner::Solid { value } => match value {
                WorkingSolid::ImportedMesh { mesh_url } => child = Some(Self::one_string(mesh_url)),
                WorkingSolid::ImportedSolid { solid_handle } => child = Some(Self::one_string(solid_handle)),
                WorkingSolid::Reference { reference_id } => child = Some(Self::one_string(reference_id)),
                WorkingSolid::Box { .. } | WorkingSolid::Cylinder { .. } | WorkingSolid::Sphere { .. } => released_items = 1,
            },
            Process3dRetirementOwner::Child { mut value, phase } => {
                if phase < value.strings.len() {
                    if let Some(string) = value.strings[phase].take() {
                        let released = Self::release_string(string, maximum_bytes)?;
                        released_items = released.0;
                        released_bytes = released.1;
                    }
                    parent = Some(Process3dRetirementOwner::Child { value, phase: phase + 1 });
                } else {
                    released_items = 1;
                }
            }
            Process3dRetirementOwner::Strings { mut values, phase } => {
                if phase < values.len() {
                    if let Some(string) = values[phase].take() {
                        let released = Self::release_string(string, maximum_bytes)?;
                        released_items = released.0;
                        released_bytes = released.1;
                    }
                    parent = Some(Process3dRetirementOwner::Strings { values, phase: phase + 1 });
                } else {
                    released_items = 1;
                }
            }
            Process3dRetirementOwner::MutationFields { mut value, phase } => match phase {
                0 if value.machine.is_some() => {
                    child = value.machine.take().map(|value| Process3dRetirementOwner::Machine { value, phase: 0 });
                    parent = Some(Process3dRetirementOwner::MutationFields { value, phase: 1 });
                }
                0 => parent = Some(Process3dRetirementOwner::MutationFields { value, phase: 1 }),
                1 if value.step.is_some() => {
                    child = value.step.take().map(|value| Process3dRetirementOwner::Step { value, phase: 0 });
                    parent = Some(Process3dRetirementOwner::MutationFields { value, phase: 2 });
                }
                1 => parent = Some(Process3dRetirementOwner::MutationFields { value, phase: 2 }),
                2 if value.capabilities.is_some() => {
                    child = value.capabilities.take().map(|values| Process3dRetirementOwner::Capabilities { values });
                    parent = Some(Process3dRetirementOwner::MutationFields { value, phase: 3 });
                }
                2 => parent = Some(Process3dRetirementOwner::MutationFields { value, phase: 3 }),
                3 if value.origin.is_some() => {
                    child = value.origin.take().map(|value| Process3dRetirementOwner::Origin { value });
                    parent = Some(Process3dRetirementOwner::MutationFields { value, phase: 4 });
                }
                3 => parent = Some(Process3dRetirementOwner::MutationFields { value, phase: 4 }),
                4 if value.measure.is_some() => {
                    child = value.measure.take().map(|value| Process3dRetirementOwner::Measure { value });
                    parent = Some(Process3dRetirementOwner::MutationFields { value, phase: 5 });
                }
                4 => parent = Some(Process3dRetirementOwner::MutationFields { value, phase: 5 }),
                5 if value.child.is_some() => {
                    child = value.child.take().map(|value| Process3dRetirementOwner::Child { value, phase: 0 });
                    parent = Some(Process3dRetirementOwner::MutationFields { value, phase: 6 });
                }
                5 => parent = Some(Process3dRetirementOwner::MutationFields { value, phase: 6 }),
                6 => {
                    child = Some(Self::string([value.strings[0].take(), value.strings[1].take(), None, None, None, None]));
                    parent = Some(Process3dRetirementOwner::MutationFields { value, phase: 7 });
                }
                _ if value.scalars > 0 => {
                    value.scalars -= 1;
                    released_items = 1;
                    parent = Some(Process3dRetirementOwner::MutationFields { value, phase });
                }
                _ => released_items = 1,
            },
            Process3dRetirementOwner::Capabilities { mut values } => {
                if let Some(value) = values.pop() {
                    child = Some(Process3dRetirementOwner::Capability { value, phase: 0 });
                    parent = Some(Process3dRetirementOwner::Capabilities { values });
                } else {
                    released_bytes = values.capacity().saturating_mul(size_of::<Capability>());
                    drop(values);
                    released_items = 1;
                }
            }
        }
        if released_bytes > maximum_bytes {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d owner exceeded exact close byte grant"));
        }
        if let Some(parent) = parent {
            self.push(parent)?;
        }
        if let Some(child) = child {
            self.push(child)?;
        }
        Ok(store::SnapshotRetirementStep::Pending { released_items, released_bytes })
    }

    fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.slots.iter().all(Option::is_none)
    }
}

impl Drop for Process3dRetirementStack {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Process3d fixed owner stack reached Drop before terminal-empty");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.slots) };
    }
}

pub struct Process3dOwnedRetirement {
    stack: std::mem::ManuallyDrop<Option<Process3dRetirementStack>>,
    terminal: bool,
}

impl Process3dOwnedRetirement {
    fn owner(value: Process3dRetirementOwner) -> Self {
        Self { stack: std::mem::ManuallyDrop::new(Some(Process3dRetirementStack::new(value))), terminal: false }
    }

    fn snapshot(value: Process3dSnapshot) -> Self {
        Self::owner(Process3dRetirementOwner::Snapshot { value, phase: 0 })
    }

    fn mutation(value: Process3dMutation) -> Self {
        let fields = Process3dMutationFields::from_mutation(value);
        Self::owner(Process3dRetirementOwner::MutationFields { value: fields, phase: 0 })
    }
}

impl store::ErasedSnapshotRetirement for Process3dOwnedRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let Some(stack) = self.stack.as_mut() else { return Ok(store::SnapshotRetirementStep::Complete) };
        if stack.terminal_is_empty() {
            drop(self.stack.take());
            self.terminal = true;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        stack.advance(maximum_bytes.min(PROCESS3D_OWNER_BYTES))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.stack.is_none()
    }
}

impl Drop for Process3dOwnedRetirement {
    fn drop(&mut self) {
        assert!(self.terminal && self.stack.is_none(), "Process3d owner reached ordinary Drop before retained terminal-empty");
    }
}

struct Process3dSnapshotRootRetirement {
    owner: std::mem::ManuallyDrop<Option<std::sync::Arc<Process3dSnapshot>>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    terminal: bool,
}

impl store::ErasedSnapshotRetirement for Process3dSnapshotRootRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    self.terminal = true;
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d snapshot root reported false terminal")),
                step => Ok(step),
            };
        }
        let Some(owner) = self.owner.take() else {
            self.terminal = true;
            return Ok(store::SnapshotRetirementStep::Complete);
        };
        match std::sync::Arc::try_unwrap(owner) {
            Ok(value) => {
                *self.retirement = Some(Box::new(Process3dOwnedRetirement::snapshot(value)));
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            Err(owner) => {
                *self.owner = Some(owner);
                Ok(store::SnapshotRetirementStep::Blocked)
            }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.owner.is_none() && self.retirement.is_none()
    }
}

impl Drop for Process3dSnapshotRootRetirement {
    fn drop(&mut self) {
        assert!(self.terminal && self.owner.is_none() && self.retirement.is_none(), "Process3d snapshot Arc reached Drop before retained terminal-empty");
    }
}

impl store::ArtifactEnvelopeSnapshotFieldAuthority<Process3dSnapshot> for Process3dSnapshotDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("process3d-envelope.snapshot-stale-authority", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("process3d-envelope.snapshot-cancelled", token.start));
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Process3dSnapshotDecodeState::AwaitToken {
            if !terminal || token.kind != store::OwnedSchemaTokenKind::String {
                return Err(self.diagnostic("process3d-envelope.snapshot-pack-must-be-scalar", token.start));
            }
            let span = token.end.checked_sub(token.start).and_then(|span| span.checked_sub(2)).ok_or_else(|| self.diagnostic("process3d-envelope.snapshot-pack-length", token.start))?;
            if span == 0 || span & 1 != 0 {
                return Err(self.diagnostic("process3d-envelope.snapshot-pack-odd-hex", token.start));
            }
            let expected = usize::try_from(span / 2).map_err(|_| self.diagnostic("process3d-envelope.snapshot-pack-length", token.start))?;
            let maximum_items = process3d_publication_item_credit(self.operation, self.generation).map_err(|_| self.diagnostic("process3d-envelope.snapshot-item-authority", token.start))?;
            *self.session = Some(crate::standards::v1::subsets::any::io::text::snapshot::process3d_mounted_pack_session(expected, maximum_items).map_err(|_| self.diagnostic("process3d-envelope.snapshot-pack-preflight", token.start))?);
            self.token = Some(token);
            self.state = Process3dSnapshotDecodeState::Ingest;
        }
        if self.state == Process3dSnapshotDecodeState::Ingest {
            let retained = self.token.ok_or_else(|| self.diagnostic("process3d-envelope.snapshot-token-owner", token.start))?;
            if retained != token {
                return Err(self.diagnostic("process3d-envelope.snapshot-token-replayed", token.start));
            }
            let offset = retained.start + self.relative as u64;
            if offset + 1 >= retained.end {
                if self.high.is_some() {
                    return Err(self.diagnostic("process3d-envelope.snapshot-pack-odd-hex", offset));
                }
                self.session().seal().map_err(|_| self.diagnostic("process3d-envelope.snapshot-pack-seal", retained.end))?;
                self.state = Process3dSnapshotDecodeState::Drive;
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            if self.reserve_pending("process3d-envelope.snapshot-source-allocation", offset, cx)? {
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            let mut byte = [0u8; 1];
            if source.copy_token_bytes(retained, self.relative, &mut byte) != 1 {
                return Err(self.diagnostic("process3d-envelope.snapshot-pack-source", offset));
            }
            let nibble = Self::nibble(byte[0]).ok_or_else(|| self.diagnostic("process3d-envelope.snapshot-pack-hex", offset))?;
            self.relative += 1;
            cx.consume_fuel(1);
            if let Some(high) = self.high.take() {
                self.session().admit_byte((high << 4) | nibble).map_err(|_| self.diagnostic("process3d-envelope.snapshot-pack-handback", offset))?;
            } else {
                self.high = Some(nibble);
            }
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Process3dSnapshotDecodeState::Drive {
            cx.set_stage("process3d-retained-canonical-pack");
            if self.reserve_pending("process3d-envelope.snapshot-retained-allocation", token.start, cx)? {
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            cx.consume_fuel(1);
            if !self.session().grant().map_err(|_| self.diagnostic("process3d-envelope.snapshot-structural-malformed", token.start))? {
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            let value = self.session().take().ok_or_else(|| self.diagnostic("process3d-envelope.snapshot-handoff-missing", token.start))?;
            *self.value = Some(value);
            self.session().request_cancel();
            self.state = Process3dSnapshotDecodeState::CloseSession;
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Process3dSnapshotDecodeState::CloseSession {
            cx.consume_fuel(1);
            let maximum_bytes = self.session().next_retained_release_allocation_bytes().unwrap_or(0);
            if matches!(self.session().close_step(1, maximum_bytes).map_err(|_| self.diagnostic("process3d-envelope.snapshot-session-close", token.start))?, store::mounted_pack_rt::RetainedTypedPackCloseStep::Pending { .. }) {
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            drop(self.session.take());
            self.token = None;
            self.state = Process3dSnapshotDecodeState::Ready;
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete);
        }
        Err(self.diagnostic("process3d-envelope.snapshot-token-replayed", token.start))
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::ArtifactEnvelopeSnapshotFieldTarget<Process3dSnapshot>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if self.state != Process3dSnapshotDecodeState::Ready {
            return Err(self.diagnostic("process3d-envelope.snapshot-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("process3d-envelope.snapshot-owner-missing", 0))?;
        target.publish_snapshot_reserved(reservation, value);
        self.state = Process3dSnapshotDecodeState::Published;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn next_close_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        if let Some(session) = self.session.as_ref() {
            return Ok(session.next_retained_release_allocation_bytes().unwrap_or(0));
        }
        Ok(usize::from(self.retirement.is_some()) * PROCESS3D_OWNER_BYTES)
    }

    fn maximum_close_byte_demand(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.session.is_some() {
            self.session().request_cancel();
            match self.session().close_step(1, maximum_bytes).map_err(|_| self.diagnostic("process3d-envelope.snapshot-session-close", 0))? {
                store::mounted_pack_rt::RetainedTypedPackCloseStep::Pending { released_items, released_bytes } => {
                    self.state = Process3dSnapshotDecodeState::Closing;
                    return Ok(store::SnapshotRetirementStep::Pending { released_items, released_bytes });
                }
                store::mounted_pack_rt::RetainedTypedPackCloseStep::Complete => {}
            }
            drop(self.session.take());
            self.token = None;
            self.state = Process3dSnapshotDecodeState::Closing;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&Process3dSnapshotRetirementFactory, value));
                self.state = Process3dSnapshotDecodeState::Closing;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.state = Process3dSnapshotDecodeState::Complete;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let path = self.path;
        let retirement = self.retirement.as_mut().expect("Process3d snapshot retirement retained");
        match retirement.close_step(1, maximum_bytes.min(PROCESS3D_OWNER_BYTES)).map_err(|_| store::OwnedSchemaDecodeDiagnostic { code: "process3d-envelope.snapshot-retirement-fault", offset: 0, line: 0, column: 0, path })? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                self.state = Process3dSnapshotDecodeState::Complete;
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            store::SnapshotRetirementStep::Complete => Err(self.diagnostic("process3d-envelope.snapshot-retirement-false-terminal", 0)),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.state, Process3dSnapshotDecodeState::Published | Process3dSnapshotDecodeState::Complete) && self.session.is_none() && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for Process3dSnapshotDecodeAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Process3d snapshot field reached Drop before publication or terminal-empty close");
    }
}

impl Process3dRetainedCapabilityCursor {
    fn take_recipe(&mut self) -> Result<MeasureRecipe, String> {
        let first = self.recipe_fields[0].take().unwrap_or_default();
        let second = self.recipe_fields[1].take().unwrap_or_default();
        let third = self.recipe_fields[2].take().unwrap_or_default();
        match self.recipe_tag {
            0 => Ok(MeasureRecipe::DiscCut { diameter: first, kerf: second }),
            1 => Ok(MeasureRecipe::BladeCut { kerf: first, length: second, depth: third }),
            2 => Ok(MeasureRecipe::PocketCut { diameter: first, depth: second }),
            3 => Ok(MeasureRecipe::BoreDrill { radius: first, depth: second }),
            4 => Ok(MeasureRecipe::CylinderAttach { radius: first, length: second }),
            5 => Ok(MeasureRecipe::BoxAttach { width: first, depth: second, height: third }),
            _ => Err("process3d retained capability recipe tag is invalid".into()),
        }
    }

    fn finish(&mut self) -> Capability {
        Capability {
            id: self.id.take().unwrap_or_default(),
            label: self.label.take().unwrap_or_default(),
            icon_id: self.icon_id.take().unwrap_or_default(),
            recipe: self.recipe.take().unwrap_or(MeasureRecipe::DiscCut { diameter: String::new(), kerf: String::new() }),
            parameters: std::mem::take(&mut self.parameters),
            rules: std::mem::take(&mut self.rules),
        }
    }

    fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<Capability>, String> {
        match self.phase {
            0 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.id = Some(value);
            }
            1 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.label = Some(value);
            }
            2 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.icon_id = Some(value);
            }
            3 => {
                self.recipe_tag = reader.read_u8().map_err(|error| error.to_string())?;
                if self.recipe_tag > 5 {
                    return Err("process3d retained capability recipe tag is invalid".into());
                }
            }
            4..=6 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.recipe_fields[(self.phase - 4) as usize] = Some(value);
            }
            7 => {
                self.recipe = Some(self.take_recipe()?);
                self.expected = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
                if self.expected > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
                    return Err("process3d retained capability parameter count exceeds capacity".into());
                }
                self.parameters.try_reserve_exact(self.expected).map_err(|_| "process3d retained capability parameter admission failed".to_string())?;
                self.index = 0;
                self.phase = if self.expected == 0 { 11 } else { 8 };
                return Ok(None);
            }
            8 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.parameter_id = Some(value);
            }
            9 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.parameter_label = Some(value);
            }
            10 => {
                let value = reader.read_f64_le().map_err(|error| error.to_string())?;
                self.parameters.push(CapabilityParameter { id: self.parameter_id.take().unwrap_or_default(), label: self.parameter_label.take().unwrap_or_default(), value });
                self.index += 1;
                self.phase = if self.index < self.expected { 8 } else { 11 };
                return Ok(None);
            }
            11 => {
                self.expected = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
                if self.expected > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
                    return Err("process3d retained capability rule count exceeds capacity".into());
                }
                self.rules.try_reserve_exact(self.expected).map_err(|_| "process3d retained capability rule admission failed".to_string())?;
                self.index = 0;
                if self.expected == 0 {
                    return Ok(Some(self.finish()));
                }
                self.phase = 12;
                return Ok(None);
            }
            12 => {
                self.rule_tag = reader.read_u8().map_err(|error| error.to_string())?;
                if self.rule_tag > 1 {
                    return Err("process3d retained capability rule tag is invalid".into());
                }
            }
            13 => {
                self.rule_quantity = Some(match reader.read_u8().map_err(|error| error.to_string())? {
                    0 => StockQuantity::Width,
                    1 => StockQuantity::Depth,
                    2 => StockQuantity::Height,
                    3 => StockQuantity::MaxDimension,
                    4 => StockQuantity::MinDimension,
                    _ => return Err("process3d retained stock quantity tag is invalid".into()),
                });
            }
            14 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.rule_parameter = Some(value);
            }
            15 => {
                let margin = reader.read_f64_le().map_err(|error| error.to_string())?;
                let quantity = self.rule_quantity.take().unwrap_or(StockQuantity::Width);
                let parameter = self.rule_parameter.take().unwrap_or_default();
                self.rules.push(if self.rule_tag == 0 { CapabilityRule::Min { quantity, parameter, margin } } else { CapabilityRule::Max { quantity, parameter, margin } });
                self.index += 1;
                if self.index == self.expected {
                    return Ok(Some(self.finish()));
                }
                self.phase = 12;
                return Ok(None);
            }
            _ => unreachable!("retained capability phase"),
        }
        self.phase += 1;
        Ok(None)
    }

    fn take_partial(&mut self) -> Capability {
        if self.string.has_partial() {
            let partial = self.string.take_partial();
            match self.phase {
                0 => self.id = Some(partial),
                1 => self.label = Some(partial),
                2 => self.icon_id = Some(partial),
                4..=6 => self.recipe_fields[(self.phase - 4) as usize] = Some(partial),
                8 => self.parameter_id = Some(partial),
                9 => self.parameter_label = Some(partial),
                14 => self.rule_parameter = Some(partial),
                _ => drop(partial),
            }
            self.string = Default::default();
        }
        if self.parameter_id.is_some() || self.parameter_label.is_some() {
            self.parameters.push(CapabilityParameter { id: self.parameter_id.take().unwrap_or_default(), label: self.parameter_label.take().unwrap_or_default(), value: 0.0 });
        }
        if let Some(parameter) = self.rule_parameter.take() {
            let quantity = self.rule_quantity.take().unwrap_or_default();
            self.rules.push(if self.rule_tag == 1 { CapabilityRule::Max { quantity, parameter, margin: 0.0 } } else { CapabilityRule::Min { quantity, parameter, margin: 0.0 } });
        }
        if self.recipe.is_none() {
            self.recipe = self.take_recipe().ok();
        }
        self.finish()
    }
}

impl Process3dRetainedMachineCursor {
    fn finish(&mut self) -> WorkshopMachine {
        WorkshopMachine {
            id: self.id.take().unwrap_or_default(),
            label: self.label.take().unwrap_or_default(),
            icon_id: self.icon_id.take().unwrap_or_default(),
            catalog_id: self.catalog_id.take(),
            capabilities: std::mem::take(&mut self.capabilities),
        }
    }

    fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<WorkshopMachine>, String> {
        match self.phase {
            0 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.id = Some(value);
            }
            1 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.label = Some(value);
            }
            2 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.icon_id = Some(value);
            }
            3 => match reader.read_u8().map_err(|error| error.to_string())? {
                0 => {
                    self.phase = 5;
                    return Ok(None);
                }
                1 => {}
                _ => return Err("process3d retained machine catalog tag is invalid".into()),
            },
            4 => {
                let Some(value) = self.string.step(reader)? else { return Ok(None) };
                self.string = Default::default();
                self.catalog_id = Some(value);
            }
            5 => {
                self.expected = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
                if self.expected > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
                    return Err("process3d retained machine capability count exceeds capacity".into());
                }
                self.capabilities.try_reserve_exact(self.expected).map_err(|_| "process3d retained machine capability admission failed".to_string())?;
                if self.expected == 0 {
                    return Ok(Some(self.finish()));
                }
                self.phase = 6;
                return Ok(None);
            }
            6 => {
                let cursor = self.capability.get_or_insert_with(Process3dRetainedCapabilityCursor::default);
                if let Some(capability) = cursor.step(reader)? {
                    self.capabilities.push(capability);
                    self.capability = None;
                    self.index += 1;
                    if self.index == self.expected {
                        return Ok(Some(self.finish()));
                    }
                }
                return Ok(None);
            }
            _ => unreachable!("retained machine phase"),
        }
        self.phase += 1;
        Ok(None)
    }

    fn take_partial(&mut self) -> WorkshopMachine {
        if self.string.has_partial() {
            let partial = self.string.take_partial();
            match self.phase {
                0 => self.id = Some(partial),
                1 => self.label = Some(partial),
                2 => self.icon_id = Some(partial),
                4 => self.catalog_id = Some(partial),
                _ => drop(partial),
            }
            self.string = Default::default();
        }
        if let Some(mut cursor) = self.capability.take() {
            self.capabilities.push(cursor.take_partial());
        }
        self.finish()
    }
}

impl Process3dRetainedMutationReader {
    fn new() -> Self {
        Self {
            phase: Process3dRetainedMutationPhase::Header,
            offset: 0,
            tag: 0,
            field: 0,
            index: 0,
            expected: 0,
            strings: [None, None, None],
            string: Default::default(),
            enabled: false,
            origin_tag: 0,
            step: None,
            measure: None,
            pose: None,
            child: None,
            machine: None,
            capability: None,
            capabilities: Vec::new(),
            value: std::mem::ManuallyDrop::new(None),
            terminal_handoff: false,
        }
    }

    fn complete(&mut self, bytes: &[u8], value: Process3dMutation) -> Result<(), protocol::ProtocolError> {
        if self.offset != bytes.len() {
            return Err(process3d_protocol_error("retained mutation has trailing bytes".into()));
        }
        *self.value = Some(value);
        self.phase = Process3dRetainedMutationPhase::Complete;
        Ok(())
    }

    fn string_step(&mut self, bytes: &[u8]) -> Result<Option<String>, protocol::ProtocolError> {
        let value = process3d_retained_string_step(bytes, &mut self.offset, &mut self.string)?;
        if value.is_some() {
            self.string = Default::default();
        }
        Ok(value)
    }

    fn structural_step(&mut self, bytes: &[u8]) -> Result<(), protocol::ProtocolError> {
        use crate::mutations::{
            change_machine_icon, change_step_enabled, change_step_origin, change_stock_label, create_machine, create_step, delete_machine, delete_step, move_stock, rename_machine, rename_step, reorder_steps,
            replace_machine_capabilities, replace_step_measure, replace_stock_solid,
        };
        match self.tag {
            0 => {
                if self.field == 0 {
                    self.index = process3d_retained_advance(bytes, &mut self.offset, |reader| reader.read_varint_u64().map(|value| value as usize).map_err(|error| error.to_string()))?;
                    self.field = 1;
                } else {
                    let cursor = self.step.get_or_insert_with(Default::default);
                    if let Some(step) = process3d_retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                        self.step = None;
                        self.complete(bytes, Process3dMutation::CreateStep(create_step::CreateStep { index: self.index, step }))?;
                    }
                }
            }
            1 | 8 | 13 => {
                let Some(value) = self.string_step(bytes)? else { return Ok(()) };
                self.complete(
                    bytes,
                    match self.tag {
                        1 => Process3dMutation::DeleteStep(delete_step::DeleteStep { id: value }),
                        8 => Process3dMutation::DeleteMachine(delete_machine::DeleteMachine { id: value }),
                        13 => Process3dMutation::ChangeStockLabel(change_stock_label::ChangeStockLabel { new_label: value }),
                        _ => unreachable!("retained simple string tag"),
                    },
                )?;
            }
            2 | 9 | 10 => {
                let Some(value) = self.string_step(bytes)? else { return Ok(()) };
                self.strings[self.field] = Some(value);
                self.field += 1;
                if self.field == 2 {
                    let first = self.strings[0].take().unwrap_or_default();
                    let second = self.strings[1].take().unwrap_or_default();
                    self.complete(
                        bytes,
                        match self.tag {
                            2 => Process3dMutation::RenameStep(rename_step::RenameStep { id: first, new_label: second }),
                            9 => Process3dMutation::RenameMachine(rename_machine::RenameMachine { id: first, new_label: second }),
                            10 => Process3dMutation::ChangeMachineIcon(change_machine_icon::ChangeMachineIcon { id: first, new_icon_id: second }),
                            _ => unreachable!("retained double string tag"),
                        },
                    )?;
                }
            }
            3 => {
                if self.field == 0 {
                    let Some(value) = self.string_step(bytes)? else { return Ok(()) };
                    self.strings[0] = Some(value);
                    self.field = 1;
                } else {
                    self.enabled = process3d_retained_advance(bytes, &mut self.offset, |reader| reader.read_u8().map(|value| value != 0).map_err(|error| error.to_string()))?;
                    let mutation = Process3dMutation::ChangeStepEnabled(change_step_enabled::ChangeStepEnabled { id: self.strings[0].take().unwrap_or_default(), new_enabled: self.enabled });
                    self.complete(bytes, mutation)?;
                }
            }
            4 => match self.field {
                0 => {
                    let Some(value) = self.string_step(bytes)? else { return Ok(()) };
                    self.strings[0] = Some(value);
                    self.field = 1;
                }
                1 => {
                    self.origin_tag = process3d_retained_advance(bytes, &mut self.offset, |reader| reader.read_u8().map_err(|error| error.to_string()))?;
                    match self.origin_tag {
                        0 => {
                            let mutation = Process3dMutation::ChangeStepOrigin(change_step_origin::ChangeStepOrigin { id: self.strings[0].take().unwrap_or_default(), new_origin: None });
                            self.complete(bytes, mutation)?;
                        }
                        1 => self.field = 2,
                        _ => return Err(process3d_protocol_error("retained step origin tag is invalid".into())),
                    }
                }
                2 | 3 => {
                    let Some(value) = self.string_step(bytes)? else { return Ok(()) };
                    self.strings[self.field - 1] = Some(value);
                    self.field += 1;
                    if self.field == 4 {
                        let id = self.strings[0].take().unwrap_or_default();
                        let machine_id = self.strings[1].take().unwrap_or_default();
                        let capability_id = self.strings[2].take().unwrap_or_default();
                        let mutation = Process3dMutation::ChangeStepOrigin(change_step_origin::ChangeStepOrigin { id, new_origin: Some(StepOrigin { machine_id, capability_id }) });
                        self.complete(bytes, mutation)?;
                    }
                }
                _ => unreachable!("retained origin field"),
            },
            5 => {
                if self.field == 0 {
                    let Some(value) = self.string_step(bytes)? else { return Ok(()) };
                    self.strings[0] = Some(value);
                    self.field = 1;
                } else {
                    let cursor = self.measure.get_or_insert_with(Default::default);
                    if let Some(new_measure) = process3d_retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                        self.measure = None;
                        let mutation = Process3dMutation::ReplaceStepMeasure(replace_step_measure::ReplaceStepMeasure { id: self.strings[0].take().unwrap_or_default(), new_measure });
                        self.complete(bytes, mutation)?;
                    }
                }
            }
            6 => {
                if self.field == 0 {
                    let Some(value) = self.string_step(bytes)? else { return Ok(()) };
                    self.strings[0] = Some(value);
                    self.field = 1;
                } else {
                    let to_index = process3d_retained_advance(bytes, &mut self.offset, |reader| reader.read_varint_u64().map(|value| value as usize).map_err(|error| error.to_string()))?;
                    let mutation = Process3dMutation::ReorderSteps(reorder_steps::ReorderSteps { id: self.strings[0].take().unwrap_or_default(), to_index });
                    self.complete(bytes, mutation)?;
                }
            }
            7 => {
                if self.field == 0 {
                    self.index = process3d_retained_advance(bytes, &mut self.offset, |reader| reader.read_varint_u64().map(|value| value as usize).map_err(|error| error.to_string()))?;
                    self.field = 1;
                } else {
                    let cursor = self.machine.get_or_insert_with(Default::default);
                    if let Some(machine) = process3d_retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                        self.machine = None;
                        self.complete(bytes, Process3dMutation::CreateMachine(create_machine::CreateMachine { index: self.index, machine }))?;
                    }
                }
            }
            11 => match self.field {
                0 => {
                    let Some(value) = self.string_step(bytes)? else { return Ok(()) };
                    self.strings[0] = Some(value);
                    self.field = 1;
                }
                1 => {
                    self.expected = process3d_retained_advance(bytes, &mut self.offset, |reader| reader.read_varint_u64().map(|value| value as usize).map_err(|error| error.to_string()))?;
                    if self.expected > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
                        return Err(protocol::ProtocolError::LimitExceeded("process3d retained capability count exceeds fixed catalog"));
                    }
                    self.capabilities.try_reserve_exact(self.expected).map_err(|_| process3d_protocol_error("retained capability admission failed".into()))?;
                    self.field = 2;
                    if self.expected == 0 {
                        let id = self.strings[0].take().unwrap_or_default();
                        let new_capabilities = std::mem::take(&mut self.capabilities);
                        let mutation = Process3dMutation::ReplaceMachineCapabilities(replace_machine_capabilities::ReplaceMachineCapabilities { id, new_capabilities });
                        self.complete(bytes, mutation)?;
                    }
                }
                _ => {
                    let cursor = self.capability.get_or_insert_with(Default::default);
                    if let Some(capability) = process3d_retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                        self.capabilities.push(capability);
                        self.capability = None;
                        self.index += 1;
                        if self.index == self.expected {
                            let id = self.strings[0].take().unwrap_or_default();
                            let new_capabilities = std::mem::take(&mut self.capabilities);
                            let mutation = Process3dMutation::ReplaceMachineCapabilities(replace_machine_capabilities::ReplaceMachineCapabilities { id, new_capabilities });
                            self.complete(bytes, mutation)?;
                        }
                    }
                }
            },
            12 => {
                let cursor = self.pose.get_or_insert_with(Default::default);
                if let Some(new_pose) = process3d_retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                    self.pose = None;
                    self.complete(bytes, Process3dMutation::MoveStock(move_stock::MoveStock { new_pose }))?;
                }
            }
            14 => {
                let cursor = self.child.get_or_insert_with(Default::default);
                if let Some(new_solid) = process3d_retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                    self.child = None;
                    self.complete(bytes, Process3dMutation::ReplaceStockSolid(replace_stock_solid::ReplaceStockSolid { new_solid }))?;
                }
            }
            _ => return Err(process3d_protocol_error("retained mutation tag is invalid".into())),
        }
        Ok(())
    }

    fn step(&mut self, bytes: &[u8], cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, protocol::ProtocolError> {
        if cx.should_yield() || cx.fuel_remaining() == 0 || cx.is_cancelled() {
            return Ok(false);
        }
        match self.phase {
            Process3dRetainedMutationPhase::Header => {
                if bytes.first().copied() != Some(PROCESS3D_MUTATION_BINARY_FORMAT) || bytes.len() < 2 || bytes.len() > PROCESS3D_OWNER_BYTES {
                    return Err(process3d_protocol_error("invalid retained mutation header".into()));
                }
                self.offset = 1;
                self.phase = Process3dRetainedMutationPhase::Tag;
            }
            Process3dRetainedMutationPhase::Tag => {
                self.tag = process3d_retained_advance(bytes, &mut self.offset, |reader| reader.read_u8().map_err(|error| error.to_string()))?;
                if self.tag >= PROCESS3D_MUTATION_VARIANT_COUNT as u8 {
                    return Err(process3d_protocol_error("retained mutation tag is invalid".into()));
                }
                self.phase = Process3dRetainedMutationPhase::Structural;
            }
            Process3dRetainedMutationPhase::Structural => {
                self.structural_step(bytes)?;
            }
            Process3dRetainedMutationPhase::Complete => return Ok(true),
        }
        cx.consume_fuel(1);
        Ok(self.phase == Process3dRetainedMutationPhase::Complete)
    }

    fn take(&mut self) -> Option<Process3dMutation> {
        if self.phase != Process3dRetainedMutationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let value = self.value.take()?;
        self.terminal_handoff = true;
        Some(value)
    }

    fn take_partial(&mut self) -> Option<Process3dMutation> {
        use crate::mutations::{
            change_machine_icon, change_step_enabled, change_step_origin, change_stock_label, create_machine, create_step, delete_machine, delete_step, move_stock, rename_machine, rename_step, reorder_steps,
            replace_machine_capabilities, replace_step_measure, replace_stock_solid,
        };
        if self.string.has_partial() {
            let partial = self.string.take_partial();
            match self.tag {
                1 | 8 | 13 => self.strings[0] = Some(partial),
                2 | 9 | 10 => self.strings[self.field.min(1)] = Some(partial),
                3 | 5 | 6 | 11 if self.field == 0 => self.strings[0] = Some(partial),
                4 if self.field == 0 => self.strings[0] = Some(partial),
                4 if matches!(self.field, 2 | 3) => self.strings[self.field - 1] = Some(partial),
                _ => drop(partial),
            }
            self.string = Default::default();
        }
        let value = match self.tag {
            0 => Some(Process3dMutation::CreateStep(create_step::CreateStep {
                index: self.index,
                step: self
                    .step
                    .take()
                    .map_or(ProcessStep { id: String::new(), label: String::new(), enabled: false, origin: None, measure: ProcessMeasure::Drill { radius: 0.0, depth: 0.0, pose: Default::default() } }, |mut cursor| cursor.take_partial()),
            })),
            1 => Some(Process3dMutation::DeleteStep(delete_step::DeleteStep { id: self.strings[0].take().unwrap_or_default() })),
            2 => Some(Process3dMutation::RenameStep(rename_step::RenameStep { id: self.strings[0].take().unwrap_or_default(), new_label: self.strings[1].take().unwrap_or_default() })),
            3 => Some(Process3dMutation::ChangeStepEnabled(change_step_enabled::ChangeStepEnabled { id: self.strings[0].take().unwrap_or_default(), new_enabled: self.enabled })),
            4 => Some(Process3dMutation::ChangeStepOrigin(change_step_origin::ChangeStepOrigin {
                id: self.strings[0].take().unwrap_or_default(),
                new_origin: (self.origin_tag == 1).then(|| StepOrigin { machine_id: self.strings[1].take().unwrap_or_default(), capability_id: self.strings[2].take().unwrap_or_default() }),
            })),
            5 => Some(Process3dMutation::ReplaceStepMeasure(replace_step_measure::ReplaceStepMeasure {
                id: self.strings[0].take().unwrap_or_default(),
                new_measure: self.measure.take().map_or(ProcessMeasure::Drill { radius: 0.0, depth: 0.0, pose: Default::default() }, |mut cursor| cursor.take_partial()),
            })),
            6 => Some(Process3dMutation::ReorderSteps(reorder_steps::ReorderSteps { id: self.strings[0].take().unwrap_or_default(), to_index: self.index })),
            7 => Some(Process3dMutation::CreateMachine(create_machine::CreateMachine {
                index: self.index,
                machine: self.machine.take().map_or(WorkshopMachine { id: String::new(), label: String::new(), icon_id: String::new(), catalog_id: None, capabilities: Vec::new() }, |mut cursor| cursor.take_partial()),
            })),
            8 => Some(Process3dMutation::DeleteMachine(delete_machine::DeleteMachine { id: self.strings[0].take().unwrap_or_default() })),
            9 => Some(Process3dMutation::RenameMachine(rename_machine::RenameMachine { id: self.strings[0].take().unwrap_or_default(), new_label: self.strings[1].take().unwrap_or_default() })),
            10 => Some(Process3dMutation::ChangeMachineIcon(change_machine_icon::ChangeMachineIcon { id: self.strings[0].take().unwrap_or_default(), new_icon_id: self.strings[1].take().unwrap_or_default() })),
            11 => {
                if let Some(mut cursor) = self.capability.take() {
                    self.capabilities.push(cursor.take_partial());
                }
                Some(Process3dMutation::ReplaceMachineCapabilities(replace_machine_capabilities::ReplaceMachineCapabilities { id: self.strings[0].take().unwrap_or_default(), new_capabilities: std::mem::take(&mut self.capabilities) }))
            }
            12 => Some(Process3dMutation::MoveStock(move_stock::MoveStock { new_pose: self.pose.take().map(|mut cursor| cursor.take_partial()).unwrap_or_default() })),
            13 => Some(Process3dMutation::ChangeStockLabel(change_stock_label::ChangeStockLabel { new_label: self.strings[0].take().unwrap_or_default() })),
            14 => Some(Process3dMutation::ReplaceStockSolid(replace_stock_solid::ReplaceStockSolid {
                new_solid: self.child.take().map_or_else(
                    || store::ArtifactChild::new(String::new(), semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } }),
                    |mut cursor| cursor.take_partial(),
                ),
            })),
            _ => None,
        };
        for string in &mut self.strings {
            drop(string.take());
        }
        value
    }

    fn take_rejected(&mut self) -> Option<Process3dMutation> {
        if self.terminal_handoff {
            return None;
        }
        let value = self.value.take().or_else(|| self.take_partial());
        self.terminal_handoff = true;
        value
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_handoff
            && self.value.is_none()
            && self.strings.iter().all(Option::is_none)
            && self.string.terminal_is_empty()
            && self.step.is_none()
            && self.measure.is_none()
            && self.pose.is_none()
            && self.child.is_none()
            && self.machine.is_none()
            && self.capability.is_none()
            && self.capabilities.is_empty()
    }
}

impl Drop for Process3dRetainedMutationReader {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Process3d retained mutation reader reached Drop before handoff");
    }
}

struct Process3dMutationDecodeAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    state: Process3dMutationDecodeState,
    hex: std::mem::ManuallyDrop<Option<store::OwnedSchemaHexAuthority<PROCESS3D_OWNER_BYTES>>>,
    reader: std::mem::ManuallyDrop<Option<Process3dRetainedMutationReader>>,
    value: std::mem::ManuallyDrop<Option<Process3dMutation>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    retirement_terminal: bool,
}

impl Process3dMutationDecodeAuthority {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self {
            operation,
            generation,
            path,
            state: Process3dMutationDecodeState::AwaitToken,
            hex: std::mem::ManuallyDrop::new(None),
            reader: std::mem::ManuallyDrop::new(None),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            retirement_terminal: false,
        }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
    }
}

impl store::ArtifactEnvelopeMutationFieldAuthority<Process3dMutation> for Process3dMutationDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("process3d-envelope.mutation-stale-authority", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("process3d-envelope.mutation-cancelled", token.start));
        }
        if self.state == Process3dMutationDecodeState::AwaitToken {
            if !terminal {
                return Err(self.diagnostic("process3d-envelope.mutation-pack-must-be-scalar", token.start));
            }
            *self.hex = Some(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
            self.state = Process3dMutationDecodeState::Hex;
        }
        if self.state == Process3dMutationDecodeState::Hex {
            return match self.hex.as_mut().expect("Process3d mutation hex owner retained").step(source, cx) {
                store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
                store::OwnedSchemaHexStep::Complete => {
                    *self.reader = Some(Process3dRetainedMutationReader::new());
                    self.state = Process3dMutationDecodeState::Structural;
                    Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending)
                }
                store::OwnedSchemaHexStep::Cancelled => Err(self.diagnostic("process3d-envelope.mutation-pack-cancelled", token.start)),
                store::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
            };
        }
        if self.state != Process3dMutationDecodeState::Structural {
            return Err(self.diagnostic("process3d-envelope.mutation-token-replayed", token.start));
        }
        let bytes = self.hex.as_ref().and_then(store::OwnedSchemaHexAuthority::as_bytes).ok_or_else(|| self.diagnostic("process3d-envelope.mutation-backing-missing", token.start))?;
        let complete = self.reader.as_mut().expect("Process3d retained mutation reader exists").step(bytes, cx).map_err(|_| self.diagnostic("process3d-envelope.mutation-structural-malformed", token.start))?;
        if !complete {
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        let value = self.reader.as_mut().expect("Process3d retained mutation reader exists").take().ok_or_else(|| self.diagnostic("process3d-envelope.mutation-handoff-missing", token.start))?;
        drop(self.reader.take());
        if !self.hex.as_mut().expect("Process3d mutation hex owner retained").release() {
            return Err(self.diagnostic("process3d-envelope.mutation-backing-release", token.start));
        }
        drop(self.hex.take());
        *self.value = Some(value);
        self.state = Process3dMutationDecodeState::Ready;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::ArtifactEnvelopeMutationFieldTarget<Process3dMutation>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if self.state != Process3dMutationDecodeState::Ready {
            return Err(self.diagnostic("process3d-envelope.mutation-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("process3d-envelope.mutation-owner-missing", 0))?;
        target.publish_mutation_reserved(reservation, value);
        self.state = Process3dMutationDecodeState::Published;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.retirement_terminal {
            drop(self.retirement.take());
            self.retirement_terminal = false;
            self.state = Process3dMutationDecodeState::Complete;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(reader) = self.reader.as_mut() {
            if let Some(value) = reader.take_rejected() {
                *self.value = Some(value);
            }
            drop(self.reader.take());
            self.state = Process3dMutationDecodeState::Closing;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(hex) = self.hex.as_mut() {
            hex.cancel();
            drop(self.hex.take());
            self.state = Process3dMutationDecodeState::Closing;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&Process3dMutationRetirementFactory, value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.state = Process3dMutationDecodeState::Complete;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let path = self.path;
        let retirement = self.retirement.as_mut().expect("Process3d mutation retirement retained");
        match retirement.close_step(1, maximum_bytes.min(PROCESS3D_OWNER_BYTES)).map_err(|_| store::OwnedSchemaDecodeDiagnostic { code: "process3d-envelope.mutation-retirement-fault", offset: 0, line: 0, column: 0, path })? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                self.retirement_terminal = true;
                Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
            }
            store::SnapshotRetirementStep::Complete => Err(self.diagnostic("process3d-envelope.mutation-retirement-false-terminal", 0)),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.state, Process3dMutationDecodeState::Published | Process3dMutationDecodeState::Complete) && self.hex.is_none() && self.reader.is_none() && self.value.is_none() && self.retirement.is_none() && !self.retirement_terminal
    }
}

impl Drop for Process3dMutationDecodeAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Process3d mutation field reached Drop before publication or terminal-empty close");
    }
}

impl store::ArtifactEnvelopeOwnedFieldCatalog<Process3dSnapshot, Process3dMutation> for Process3dEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Result<Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<Process3dSnapshot, Process3dMutation>>, Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<Process3dSnapshot>>> {
        store::ArtifactEnvelopeFreshVcsAuthority::try_new(
            self.begin_snapshot(operation, generation, path),
            std::sync::Arc::new(Process3dSnapshotRetirementFactory),
            std::sync::Arc::new(Process3dMutationRetirementFactory),
            self.edit_history_decoder(),
        )
        .map(|authority| Box::new(authority) as Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<Process3dSnapshot, Process3dMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<Process3dSnapshot>> {
        Box::new(Process3dSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<Process3dMutation>> {
        Box::new(Process3dMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(Process3dRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<Process3dMutation>>> {
        store::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(Process3dMutationRetirementFactory))
    }
}

pub fn process3d_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<Process3dSnapshot, Process3dMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(Process3dEnvelopeOwnedFieldCatalog), std::sync::Arc::new(Process3dSnapshotRetirementFactory), std::sync::Arc::new(Process3dMutationRetirementFactory))
}

fn process3d_copy_pose(source: &Pose) -> Pose {
    Pose { position: source.position, axis: source.axis, angle: source.angle }
}

fn process3d_copy_solid(source: &WorkingSolid) -> Result<WorkingSolid, &'static str> {
    Ok(match source {
        WorkingSolid::Box { width, depth, height } => WorkingSolid::Box { width: *width, depth: *depth, height: *height },
        WorkingSolid::Cylinder { radius, height } => WorkingSolid::Cylinder { radius: *radius, height: *height },
        WorkingSolid::Sphere { radius } => WorkingSolid::Sphere { radius: *radius },
        WorkingSolid::ImportedMesh { mesh_url } => WorkingSolid::ImportedMesh { mesh_url: process3d_copy_string(mesh_url)? },
        WorkingSolid::ImportedSolid { solid_handle } => WorkingSolid::ImportedSolid { solid_handle: process3d_copy_string(solid_handle)? },
        WorkingSolid::Reference { reference_id } => WorkingSolid::Reference { reference_id: process3d_copy_string(reference_id)? },
    })
}

fn process3d_copy_origin(source: &StepOrigin) -> Result<StepOrigin, &'static str> {
    Ok(StepOrigin { machine_id: process3d_copy_string(&source.machine_id)?, capability_id: process3d_copy_string(&source.capability_id)? })
}

fn process3d_copy_step(source: &ProcessStep) -> Result<ProcessStep, &'static str> {
    Ok(ProcessStep {
        id: process3d_copy_string(&source.id)?,
        label: process3d_copy_string(&source.label)?,
        enabled: source.enabled,
        origin: source.origin.as_ref().map(process3d_copy_origin).transpose()?,
        measure: process3d_copy_measure(&source.measure)?,
    })
}

fn process3d_copy_recipe(source: &MeasureRecipe) -> Result<MeasureRecipe, &'static str> {
    Ok(match source {
        MeasureRecipe::DiscCut { diameter, kerf } => MeasureRecipe::DiscCut { diameter: process3d_copy_string(diameter)?, kerf: process3d_copy_string(kerf)? },
        MeasureRecipe::BladeCut { kerf, length, depth } => MeasureRecipe::BladeCut { kerf: process3d_copy_string(kerf)?, length: process3d_copy_string(length)?, depth: process3d_copy_string(depth)? },
        MeasureRecipe::PocketCut { diameter, depth } => MeasureRecipe::PocketCut { diameter: process3d_copy_string(diameter)?, depth: process3d_copy_string(depth)? },
        MeasureRecipe::BoreDrill { radius, depth } => MeasureRecipe::BoreDrill { radius: process3d_copy_string(radius)?, depth: process3d_copy_string(depth)? },
        MeasureRecipe::CylinderAttach { radius, length } => MeasureRecipe::CylinderAttach { radius: process3d_copy_string(radius)?, length: process3d_copy_string(length)? },
        MeasureRecipe::BoxAttach { width, depth, height } => MeasureRecipe::BoxAttach { width: process3d_copy_string(width)?, depth: process3d_copy_string(depth)?, height: process3d_copy_string(height)? },
    })
}

fn process3d_copy_capability(source: &Capability) -> Result<Capability, &'static str> {
    let mut parameters = Vec::with_capacity(source.parameters.len());
    if parameters.capacity() > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
        return Err("process3d-owner.parameter-observed-capacity");
    }
    for parameter in &source.parameters {
        parameters.push(CapabilityParameter { id: process3d_copy_string(&parameter.id)?, label: process3d_copy_string(&parameter.label)?, value: parameter.value });
    }
    let mut rules = Vec::with_capacity(source.rules.len());
    if rules.capacity() > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
        return Err("process3d-owner.rule-observed-capacity");
    }
    for rule in &source.rules {
        rules.push(match rule {
            CapabilityRule::Min { quantity, parameter, margin } => CapabilityRule::Min { quantity: *quantity, parameter: process3d_copy_string(parameter)?, margin: *margin },
            CapabilityRule::Max { quantity, parameter, margin } => CapabilityRule::Max { quantity: *quantity, parameter: process3d_copy_string(parameter)?, margin: *margin },
        });
    }
    Ok(Capability { id: process3d_copy_string(&source.id)?, label: process3d_copy_string(&source.label)?, icon_id: process3d_copy_string(&source.icon_id)?, recipe: process3d_copy_recipe(&source.recipe)?, parameters, rules })
}

fn process3d_copy_machine(source: &WorkshopMachine) -> Result<WorkshopMachine, &'static str> {
    let mut capabilities = Vec::with_capacity(source.capabilities.len());
    if capabilities.capacity() > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
        return Err("process3d-owner.capability-observed-capacity");
    }
    for capability in &source.capabilities {
        capabilities.push(process3d_copy_capability(capability)?);
    }
    Ok(WorkshopMachine {
        id: process3d_copy_string(&source.id)?,
        label: process3d_copy_string(&source.label)?,
        icon_id: process3d_copy_string(&source.icon_id)?,
        catalog_id: source.catalog_id.as_deref().map(process3d_copy_string).transpose()?,
        capabilities,
    })
}

fn process3d_copy_stock(source: &Stock) -> Result<Stock, &'static str> {
    Ok(Stock { id: process3d_copy_string(&source.id)?, label: process3d_copy_string(&source.label)?, solid: process3d_copy_solid(&source.solid)?, pose: process3d_copy_pose(&source.pose) })
}

fn process3d_copy_child<S>(source: &store::ArtifactChild<S>) -> Result<store::ArtifactChild<S>, &'static str> {
    Ok(store::ArtifactChild::new(
        process3d_copy_string(&source.child_id)?,
        semio_framework_artifact_reference::ArtifactRef {
            artifact_id: process3d_copy_string(&source.target.artifact_id)?,
            dialect: semio_framework_artifact_reference::ArtifactDialect {
                artifact_kind: process3d_copy_string(&source.target.dialect.artifact_kind)?,
                standard: process3d_copy_string(&source.target.dialect.standard)?,
                subset: process3d_copy_string(&source.target.dialect.subset)?,
            },
        },
    ))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dCensusPhase {
    Root,
    Machines(usize),
    Steps(usize),
    Tools(usize),
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dCopyPhase {
    Shell,
    Machines(usize),
    Steps(usize),
    Tools(usize),
    Complete,
}

const PROCESS3D_MUTATION_BINARY_FORMAT: u8 = 2;

fn process3d_protocol_error(detail: String) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "process3d mutation", offset: 0, detail }
}

const PROCESS3D_MUTATION_VARIANT_COUNT: usize = 15;

#[cfg(test)]
#[derive(Clone, Copy)]
struct Process3dPublicationHostileLease {
    operation: u64,
    hostile: Process3dPublicationHostile,
    observed: Option<&'static str>,
}

#[cfg(test)]
pub fn process3d_arm_publication_hostile(operation: semio_framework_job::OperationId, hostile: Process3dPublicationHostile) {
    let mut hostiles = process3d_publication_hostiles().try_lock().expect("Process3d hostile publication authority is uncontended");
    let slot = hostiles.iter_mut().find(|slot| slot.is_none()).expect("Process3d hostile publication authority has a fixed slot");
    *slot = Some(Process3dPublicationHostileLease { operation: operation.0, hostile, observed: None });
}

#[cfg(test)]
pub fn process3d_take_publication_hostile_observed(operation: semio_framework_job::OperationId) -> Option<&'static str> {
    let mut hostiles = process3d_publication_hostiles().try_lock().expect("Process3d hostile publication authority is uncontended");
    let slot = hostiles.iter_mut().find(|slot| slot.is_some_and(|value| value.operation == operation.0))?;
    slot.take()?.observed
}

fn process3d_take_child<S>(child: &mut store::ArtifactChild<S>) -> store::ArtifactChild<S> {
    std::mem::replace(child, process3d_empty_child())
}

struct Process3dMutationFields {
    strings: [Option<String>; 2],
    machine: Option<WorkshopMachine>,
    step: Option<ProcessStep>,
    capabilities: Option<Vec<Capability>>,
    origin: Option<StepOrigin>,
    measure: Option<ProcessMeasure>,
    child: Option<Process3dChildParts>,
    scalars: u8,
}

impl Process3dMutationFields {
    fn empty() -> Self {
        Self { strings: [None, None], machine: None, step: None, capabilities: None, origin: None, measure: None, child: None, scalars: 0 }
    }

    fn from_mutation(mutation: Process3dMutation) -> Self {
        use Process3dMutation::*;
        let mut fields = Self::empty();
        match mutation {
            CreateStep(value) => {
                fields.step = Some(value.step);
                fields.scalars = 1;
            }
            DeleteStep(value) => fields.strings[0] = Some(value.id),
            RenameStep(value) => fields.strings = [Some(value.id), Some(value.new_label)],
            ChangeStepEnabled(value) => {
                fields.strings[0] = Some(value.id);
                fields.scalars = 1;
            }
            ChangeStepOrigin(value) => {
                fields.strings[0] = Some(value.id);
                fields.origin = value.new_origin;
                fields.scalars = 1;
            }
            ReplaceStepMeasure(value) => {
                fields.strings[0] = Some(value.id);
                fields.measure = Some(value.new_measure);
            }
            ReorderSteps(value) => {
                fields.strings[0] = Some(value.id);
                fields.scalars = 1;
            }
            CreateMachine(value) => {
                fields.machine = Some(value.machine);
                fields.scalars = 1;
            }
            DeleteMachine(value) => fields.strings[0] = Some(value.id),
            RenameMachine(value) => fields.strings = [Some(value.id), Some(value.new_label)],
            ChangeMachineIcon(value) => fields.strings = [Some(value.id), Some(value.new_icon_id)],
            ReplaceMachineCapabilities(value) => {
                fields.strings[0] = Some(value.id);
                fields.capabilities = Some(value.new_capabilities);
            }
            MoveStock(_) => fields.scalars = 7,
            ChangeStockLabel(value) => fields.strings[0] = Some(value.new_label),
            ReplaceStockSolid(value) => fields.child = Some(Process3dChildParts::from_child(value.new_solid)),
        }
        fields
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dSnapshotDecodeState {
    AwaitToken,
    Ingest,
    Drive,
    CloseSession,
    Ready,
    Published,
    Closing,
    Complete,
}

/// 🧵️ The envelope's `snapshot` field: hex pack bytes streamed nibble by nibble into this
/// artifact's mounted canonical pack session, then the typed snapshot published exactly once.
struct Process3dSnapshotDecodeAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    state: Process3dSnapshotDecodeState,
    token: Option<store::OwnedSchemaToken>,
    relative: usize,
    high: Option<u8>,
    session: std::mem::ManuallyDrop<Option<crate::standards::v1::subsets::any::io::binary::snapshot::Process3dMountedPackSession>>,
    value: std::mem::ManuallyDrop<Option<Process3dSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl Process3dSnapshotDecodeAuthority {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self {
            operation,
            generation,
            path,
            state: Process3dSnapshotDecodeState::AwaitToken,
            token: None,
            relative: 1,
            high: None,
            session: std::mem::ManuallyDrop::new(None),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
        }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
    }

    fn session(&mut self) -> &mut crate::standards::v1::subsets::any::io::binary::snapshot::Process3dMountedPackSession {
        self.session.as_mut().expect("Process3d mounted pack session retained")
    }

    fn nibble(value: u8) -> Option<u8> {
        match value {
            b'0'..=b'9' => Some(value - b'0'),
            b'a'..=b'f' => Some(value - b'a' + 10),
            b'A'..=b'F' => Some(value - b'A' + 10),
            _ => None,
        }
    }

    fn reserve_pending(&mut self, code: &'static str, offset: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, store::OwnedSchemaDecodeDiagnostic> {
        let Some(exact) = self.session().next_retained_allocation_bytes().map_err(|_| self.diagnostic(code, offset))? else { return Ok(false) };
        let step = self.session().reserve_retained_allocation(exact).map_err(|_| self.diagnostic(code, offset))?;
        if !step.progressed {
            return Err(self.diagnostic("process3d-envelope.snapshot-allocation-stalled", offset));
        }
        cx.consume_fuel(1);
        Ok(true)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dRetainedMutationPhase {
    Header,
    Tag,
    Structural,
    Complete,
}

fn process3d_retained_advance<T>(bytes: &[u8], offset: &mut usize, read: impl FnOnce(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<T, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes.get(*offset..).ok_or_else(|| process3d_protocol_error("retained mutation cursor escaped its backing".into()))?);
    let value = read(&mut reader).map_err(process3d_protocol_error)?;
    *offset = offset.checked_add(reader.position()).ok_or_else(|| process3d_protocol_error("retained mutation cursor overflow".into()))?;
    Ok(value)
}

fn process3d_retained_string_step(bytes: &[u8], offset: &mut usize, cursor: &mut Process3dRetainedStringCursor) -> Result<Option<String>, protocol::ProtocolError> {
    process3d_retained_advance(bytes, offset, |reader| cursor.step(reader))
}

#[derive(Default)]
struct Process3dRetainedCapabilityCursor {
    phase: u8,
    string: Process3dRetainedStringCursor,
    id: Option<String>,
    label: Option<String>,
    icon_id: Option<String>,
    recipe_tag: u8,
    recipe_fields: [Option<String>; 3],
    recipe: Option<MeasureRecipe>,
    expected: usize,
    index: usize,
    parameter_id: Option<String>,
    parameter_label: Option<String>,
    parameters: Vec<CapabilityParameter>,
    rule_tag: u8,
    rule_quantity: Option<StockQuantity>,
    rule_parameter: Option<String>,
    rules: Vec<CapabilityRule>,
}

#[derive(Default)]
struct Process3dRetainedMachineCursor {
    phase: u8,
    string: Process3dRetainedStringCursor,
    id: Option<String>,
    label: Option<String>,
    icon_id: Option<String>,
    catalog_id: Option<String>,
    expected: usize,
    index: usize,
    capabilities: Vec<Capability>,
    capability: Option<Process3dRetainedCapabilityCursor>,
}

struct Process3dRetainedMutationReader {
    phase: Process3dRetainedMutationPhase,
    offset: usize,
    tag: u8,
    field: usize,
    index: usize,
    expected: usize,
    strings: [Option<String>; 3],
    string: Process3dRetainedStringCursor,
    enabled: bool,
    origin_tag: u8,
    step: Option<Process3dRetainedStepCursor>,
    measure: Option<Process3dRetainedMeasureCursor>,
    pose: Option<Process3dRetainedPoseCursor>,
    child: Option<Process3dRetainedChildCursor>,
    machine: Option<Process3dRetainedMachineCursor>,
    capability: Option<Process3dRetainedCapabilityCursor>,
    capabilities: Vec<Capability>,
    value: std::mem::ManuallyDrop<Option<Process3dMutation>>,
    terminal_handoff: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dMutationDecodeState {
    AwaitToken,
    Hex,
    Structural,
    Ready,
    Published,
    Closing,
    Complete,
}

struct Process3dRejectedConflictAuthority {
    terminal: bool,
}

impl store::ArtifactEnvelopeSprConflictAuthority for Process3dRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "process3d-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })
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

pub struct Process3dEnvelopeOwnedFieldCatalog;

fn process3d_copy_measure(source: &ProcessMeasure) -> Result<ProcessMeasure, &'static str> {
    Ok(match source {
        ProcessMeasure::Cut { tool, pose } => ProcessMeasure::Cut { tool: process3d_copy_solid(tool)?, pose: process3d_copy_pose(pose) },
        ProcessMeasure::Drill { radius, depth, pose } => ProcessMeasure::Drill { radius: *radius, depth: *depth, pose: process3d_copy_pose(pose) },
        ProcessMeasure::Attach { component, pose } => ProcessMeasure::Attach { component: process3d_copy_solid(component)?, pose: process3d_copy_pose(pose) },
    })
}

#[derive(Default)]
struct Process3dRetainedPoseCursor {
    index: usize,
    values: [f64; 7],
}

impl Process3dRetainedPoseCursor {
    fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<Pose>, String> {
        self.values[self.index] = reader.read_f64_le().map_err(|error| error.to_string())?;
        self.index += 1;
        Ok((self.index == self.values.len()).then(|| Pose { position: [self.values[0], self.values[1], self.values[2]], axis: [self.values[3], self.values[4], self.values[5]], angle: self.values[6] }))
    }

    fn take_partial(&mut self) -> Pose {
        self.index = 0;
        Pose { position: [self.values[0], self.values[1], self.values[2]], axis: [self.values[3], self.values[4], self.values[5]], angle: self.values[6] }
    }
}

#[derive(Default)]
struct Process3dRetainedChildCursor {
    phase: u8,
    child_id: Option<String>,
    target: Option<semio_framework_artifact_reference::ArtifactRef>,
    string: Process3dRetainedStringCursor,
}

impl Process3dRetainedChildCursor {
    fn step<S>(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<store::ArtifactChild<S>>, String> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        if self.phase == 0 {
            let Some(child_id) = self.string.step(reader)? else { return Ok(None) };
            self.string = Process3dRetainedStringCursor::default();
            self.child_id = Some(child_id);
            self.phase = 1;
            return Ok(None);
        }
        let Some(uri) = self.string.step(reader)? else { return Ok(None) };
        self.string = Process3dRetainedStringCursor::default();
        let target = semio_framework_artifact_reference::ArtifactRef::parse_uri(&uri)?;
        self.target = Some(target);
        Ok(Some(store::ArtifactChild::new(self.child_id.take().unwrap_or_default(), self.target.take().expect("Process3d retained child target exists"))))
    }

    fn take_partial<S>(&mut self) -> store::ArtifactChild<S> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        if self.phase == 0 && self.child_id.is_none() {
            self.child_id = Some(self.string.take_partial());
        } else if self.phase == 1 && self.target.is_none() {
            self.target = semio_framework_artifact_reference::ArtifactRef::parse_uri(&self.string.take_partial()).ok();
        }
        store::ArtifactChild::new(
            self.child_id.take().unwrap_or_default(),
            self.target.take().unwrap_or(semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } }),
        )
    }
}

struct Process3dRetainedMeasureCursor {
    phase: Process3dRetainedMeasurePhase,
    tag: u8,
    radius: f64,
    depth: f64,
    solid: Option<WorkingSolid>,
    solid_cursor: Process3dRetainedSolidCursor,
    pose_cursor: Process3dRetainedPoseCursor,
}

impl Default for Process3dRetainedMeasureCursor {
    fn default() -> Self {
        Self { phase: Process3dRetainedMeasurePhase::Tag, tag: 0, radius: 0.0, depth: 0.0, solid: None, solid_cursor: Process3dRetainedSolidCursor::default(), pose_cursor: Process3dRetainedPoseCursor::default() }
    }
}

impl Process3dRetainedMeasureCursor {
    fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<ProcessMeasure>, String> {
        match self.phase {
            Process3dRetainedMeasurePhase::Tag => {
                self.tag = reader.read_u8().map_err(|error| error.to_string())?;
                self.phase = match self.tag {
                    0 | 2 => Process3dRetainedMeasurePhase::Solid,
                    1 => Process3dRetainedMeasurePhase::Radius,
                    _ => return Err("process3d retained measure tag is invalid".into()),
                };
            }
            Process3dRetainedMeasurePhase::Solid => {
                if let Some(solid) = self.solid_cursor.step(reader)? {
                    self.solid = Some(solid);
                    self.phase = Process3dRetainedMeasurePhase::Pose;
                }
            }
            Process3dRetainedMeasurePhase::Radius => {
                self.radius = reader.read_f64_le().map_err(|error| error.to_string())?;
                self.phase = Process3dRetainedMeasurePhase::Depth;
            }
            Process3dRetainedMeasurePhase::Depth => {
                self.depth = reader.read_f64_le().map_err(|error| error.to_string())?;
                self.phase = Process3dRetainedMeasurePhase::Pose;
            }
            Process3dRetainedMeasurePhase::Pose => {
                if let Some(pose) = self.pose_cursor.step(reader)? {
                    let solid = self.solid.take();
                    return Ok(Some(match self.tag {
                        0 => ProcessMeasure::Cut { tool: solid.unwrap_or(WorkingSolid::Sphere { radius: 0.0 }), pose },
                        1 => ProcessMeasure::Drill { radius: self.radius, depth: self.depth, pose },
                        2 => ProcessMeasure::Attach { component: solid.unwrap_or(WorkingSolid::Sphere { radius: 0.0 }), pose },
                        _ => unreachable!("validated retained measure tag"),
                    }));
                }
            }
        }
        Ok(None)
    }

    fn take_partial(&mut self) -> ProcessMeasure {
        let pose = self.pose_cursor.take_partial();
        let solid = self.solid.take().unwrap_or_else(|| self.solid_cursor.take_partial());
        match self.tag {
            1 => ProcessMeasure::Drill { radius: self.radius, depth: self.depth, pose },
            2 => ProcessMeasure::Attach { component: solid, pose },
            _ => ProcessMeasure::Cut { tool: solid, pose },
        }
    }
}

struct Process3dRetainedStepCursor {
    phase: Process3dRetainedStepPhase,
    id: Option<String>,
    label: Option<String>,
    enabled: bool,
    origin_machine: Option<String>,
    origin_capability: Option<String>,
    string: Process3dRetainedStringCursor,
    measure_cursor: Process3dRetainedMeasureCursor,
}

impl Default for Process3dRetainedStepCursor {
    fn default() -> Self {
        Self { phase: Process3dRetainedStepPhase::Id, id: None, label: None, enabled: false, origin_machine: None, origin_capability: None, string: Process3dRetainedStringCursor::default(), measure_cursor: Process3dRetainedMeasureCursor::default() }
    }
}

impl Process3dRetainedStepCursor {
    fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<ProcessStep>, String> {
        match self.phase {
            Process3dRetainedStepPhase::Id => {
                let Some(id) = self.string.step(reader)? else { return Ok(None) };
                self.string = Process3dRetainedStringCursor::default();
                self.id = Some(id);
                self.phase = Process3dRetainedStepPhase::Label;
            }
            Process3dRetainedStepPhase::Label => {
                let Some(label) = self.string.step(reader)? else { return Ok(None) };
                self.string = Process3dRetainedStringCursor::default();
                self.label = Some(label);
                self.phase = Process3dRetainedStepPhase::Enabled;
            }
            Process3dRetainedStepPhase::Enabled => {
                self.enabled = reader.read_u8().map_err(|error| error.to_string())? != 0;
                self.phase = Process3dRetainedStepPhase::OriginTag;
            }
            Process3dRetainedStepPhase::OriginTag => {
                self.phase = match reader.read_u8().map_err(|error| error.to_string())? {
                    0 => Process3dRetainedStepPhase::Measure,
                    1 => Process3dRetainedStepPhase::OriginMachine,
                    _ => return Err("process3d retained origin tag is invalid".into()),
                };
            }
            Process3dRetainedStepPhase::OriginMachine => {
                let Some(machine) = self.string.step(reader)? else { return Ok(None) };
                self.string = Process3dRetainedStringCursor::default();
                self.origin_machine = Some(machine);
                self.phase = Process3dRetainedStepPhase::OriginCapability;
            }
            Process3dRetainedStepPhase::OriginCapability => {
                let Some(capability) = self.string.step(reader)? else { return Ok(None) };
                self.string = Process3dRetainedStringCursor::default();
                self.origin_capability = Some(capability);
                self.phase = Process3dRetainedStepPhase::Measure;
            }
            Process3dRetainedStepPhase::Measure => {
                if let Some(measure) = self.measure_cursor.step(reader)? {
                    return Ok(Some(ProcessStep {
                        id: self.id.take().unwrap_or_default(),
                        label: self.label.take().unwrap_or_default(),
                        enabled: self.enabled,
                        origin: self.origin_machine.take().map(|machine_id| StepOrigin { machine_id, capability_id: self.origin_capability.take().unwrap_or_default() }),
                        measure,
                    }));
                }
            }
        }
        Ok(None)
    }

    fn take_partial(&mut self) -> ProcessStep {
        let partial = self.string.take_partial();
        match self.phase {
            Process3dRetainedStepPhase::Id if self.id.is_none() => self.id = Some(partial),
            Process3dRetainedStepPhase::Label if self.label.is_none() => self.label = Some(partial),
            Process3dRetainedStepPhase::OriginMachine if self.origin_machine.is_none() => self.origin_machine = Some(partial),
            Process3dRetainedStepPhase::OriginCapability if self.origin_capability.is_none() => self.origin_capability = Some(partial),
            _ => drop(partial),
        }
        ProcessStep {
            id: self.id.take().unwrap_or_default(),
            label: self.label.take().unwrap_or_default(),
            enabled: self.enabled,
            origin: self.origin_machine.take().map(|machine_id| StepOrigin { machine_id, capability_id: self.origin_capability.take().unwrap_or_default() }),
            measure: self.measure_cursor.take_partial(),
        }
    }
}

struct Process3dRetainedStringCursor {
    length: usize,
    shift: u32,
    remaining: Option<usize>,
    bytes: Vec<u8>,
    maximum_bytes: usize,
}

impl Default for Process3dRetainedStringCursor {
    fn default() -> Self {
        Self { length: 0, shift: 0, remaining: None, bytes: Vec::new(), maximum_bytes: store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES }
    }
}

impl Process3dRetainedStringCursor {
    #[cfg(test)]
    fn with_maximum_bytes(maximum_bytes: usize) -> Self {
        let mut cursor = Self::default();
        cursor.maximum_bytes = maximum_bytes;
        cursor
    }

    fn finish(&mut self) -> Result<String, String> {
        self.length = 0;
        self.shift = 0;
        self.remaining = None;
        String::from_utf8(std::mem::take(&mut self.bytes)).map_err(|error| error.to_string())
    }

    fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
        if let Some(remaining) = self.remaining {
            let byte = reader.read_u8().map_err(|error| error.to_string())?;
            self.bytes.push(byte);
            let remaining = remaining - 1;
            self.remaining = Some(remaining);
            return if remaining == 0 { Ok(Some(self.finish()?)) } else { Ok(None) };
        }
        let byte = reader.read_u8().map_err(|error| error.to_string())?;
        let payload = usize::from(byte & 0x7f);
        self.length = self.length.checked_add(payload.checked_shl(self.shift).ok_or_else(|| "process3d retained string length overflow".to_string())?).ok_or_else(|| "process3d retained string length overflow".to_string())?;
        if byte & 0x80 != 0 {
            self.shift = self.shift.checked_add(7).filter(|shift| *shift < usize::BITS).ok_or_else(|| "process3d retained string length overflow".to_string())?;
            return Ok(None);
        }
        if self.length > self.maximum_bytes {
            return Err("process3d retained string exceeded its fixed byte credit".into());
        }
        self.bytes.try_reserve_exact(self.length).map_err(|_| "process3d retained string admission failed".to_string())?;
        self.remaining = Some(self.length);
        if self.length == 0 {
            Ok(Some(self.finish()?))
        } else {
            Ok(None)
        }
    }

    fn take_partial(&mut self) -> String {
        self.finish().unwrap_or_default()
    }

    fn terminal_is_empty(&self) -> bool {
        self.length == 0 && self.shift == 0 && self.remaining.is_none() && self.bytes.is_empty()
    }

    fn has_partial(&self) -> bool {
        self.length != 0 || self.shift != 0 || self.remaining.is_some() || !self.bytes.is_empty()
    }
}

impl Drop for Process3dRetainedStringCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Process3d retained string cursor reached Drop before exact handback");
    }
}

#[derive(Default)]
struct Process3dRetainedSolidCursor {
    tag: Option<u8>,
    index: usize,
    values: [f64; 3],
    text: Option<String>,
    string: Process3dRetainedStringCursor,
}

impl Process3dRetainedSolidCursor {
    fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<WorkingSolid>, String> {
        let Some(tag) = self.tag else {
            let tag = reader.read_u8().map_err(|error| error.to_string())?;
            if tag > 5 {
                return Err("process3d retained solid tag is invalid".into());
            }
            self.tag = Some(tag);
            return Ok(None);
        };
        let scalar_count = match tag {
            0 => 3,
            1 => 2,
            2 => 1,
            _ => 0,
        };
        if self.index < scalar_count {
            self.values[self.index] = reader.read_f64_le().map_err(|error| error.to_string())?;
            self.index += 1;
            if self.index < scalar_count {
                return Ok(None);
            }
        } else if tag >= 3 {
            let Some(text) = self.string.step(reader)? else { return Ok(None) };
            self.string = Process3dRetainedStringCursor::default();
            self.text = Some(text);
        }
        Ok(Some(self.take_partial()))
    }

    fn take_partial(&mut self) -> WorkingSolid {
        let tag = self.tag.take().unwrap_or(0);
        self.index = 0;
        if tag >= 3 && self.text.is_none() {
            self.text = Some(self.string.take_partial());
        }
        match tag {
            0 => WorkingSolid::Box { width: self.values[0], depth: self.values[1], height: self.values[2] },
            1 => WorkingSolid::Cylinder { radius: self.values[0], height: self.values[1] },
            2 => WorkingSolid::Sphere { radius: self.values[0] },
            3 => WorkingSolid::ImportedMesh { mesh_url: self.text.take().unwrap_or_default() },
            4 => WorkingSolid::ImportedSolid { solid_handle: self.text.take().unwrap_or_default() },
            5 => WorkingSolid::Reference { reference_id: self.text.take().unwrap_or_default() },
            _ => unreachable!("validated retained solid tag"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dRetainedMeasurePhase {
    Tag,
    Solid,
    Radius,
    Depth,
    Pose,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dRetainedStepPhase {
    Id,
    Label,
    Enabled,
    OriginTag,
    OriginMachine,
    OriginCapability,
    Measure,
}
