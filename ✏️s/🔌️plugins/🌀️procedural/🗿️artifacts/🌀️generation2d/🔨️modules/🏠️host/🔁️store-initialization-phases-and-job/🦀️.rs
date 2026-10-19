//! 🔁️ Store initialization phases and job: the initialization phases, the authority stepping, the document store initialization job, the publication hostiles and the hostile lease with its arming. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Generation2dStoreInitializationPhase {
    BindGenesis,
    ValidateEnvelope,
    ValidateEdit { index: usize },
    CensusHistory { edit: usize, mutation: usize },
    CopyInitial,
    AdoptWorkspace,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FoldSupersessions { transition: usize },
    FindApplied { position: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize },
    CommitRedo { position: usize, edit: usize },
    BuildOwners,
    BuildCandidate,
    Complete,
    RetireCancelled,
    RetireFault,
    Cancelled,
    Fault,
}

impl Generation2dStoreInitializationAuthority {
    pub(super) fn new(envelope: store::os_store::store_schema::ArtifactEnvelope<Generation2dSnapshot, Generation2dMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Self {
        if generation2d_validate_publication_authority(operation, generation).is_err() {
            let _ = generation2d_admit_app_publication_authority(operation, generation);
        }
        let (base_revision, parent_revision) = generation2d_validate_publication_authority(operation, generation).unwrap_or((u64::MAX, u64::MAX));
        Self {
            actor,
            operation,
            generation,
            base_revision,
            parent_revision,
            history_items: 0,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            copy: std::mem::ManuallyDrop::new(None),
            runtime: std::mem::ManuallyDrop::new(None),
            owners: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            displaced: std::mem::ManuallyDrop::new(None),
            rejected: std::mem::ManuallyDrop::new(None),
            candidate_disposer: std::mem::ManuallyDrop::new(None),
            closer: std::mem::ManuallyDrop::new(None),
            snapshot_factory: std::mem::ManuallyDrop::new(Some(std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation2dSnapshot>::default()))),
            factory_close: std::mem::ManuallyDrop::new(None),
            publication: semio_framework_job_publication::RetainedJobPublication::new(),
            edit_index: store::ArtifactStoreInitializationEditIndex::default(),
            resume_phase: None,
            phase: Generation2dStoreInitializationPhase::ValidateEnvelope,
            cancel_requested: false,
            fault: None,
            terminal_handoff: false,
        }
    }

    pub(super) fn fail(&mut self, code: &'static [u8]) {
        let mut value = Vec::new();
        if value.try_reserve_exact(code.len()).is_ok() {
            value.extend_from_slice(code);
        }
        self.fault = Some(value);
        self.phase = Generation2dStoreInitializationPhase::RetireFault;
    }

    pub(super) fn applied_id(&self, position: usize) -> Option<&str> {
        let envelope = self.envelope.as_ref()?;
        match &envelope.cursor {
            Some(cursor) => cursor.applied_edit_ids.get(position).map(String::as_str),
            None => envelope.vcs.edits.get(position).map(|edit| edit.id.as_str()),
        }
    }

    pub(super) fn redo_id(&self, position: usize) -> Option<&str> {
        self.envelope.as_ref()?.cursor.as_ref()?.redo_edit_ids.get(position).map(String::as_str)
    }

    pub(super) fn pump_owned(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_value::ValueError> {
        if self.active.is_some() {
            let step = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, cx.retained_grant())?;
            cx.consume_retained(step.progress())?;
            return Ok(true);
        }
        if self.displaced.is_some() {
            let step = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.displaced, &mut self.active, cx.retained_grant())?;
            cx.consume_retained(step.progress())?;
            return Ok(true);
        }
        if self.rejected.is_some() {
            let step = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.rejected, &mut self.active, cx.retained_grant())?;
            cx.consume_retained(step.progress())?;
            return Ok(true);
        }
        Ok(false)
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        use semio_framework_plugin::ArtifactOwnedDisposer;
        use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> {
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "generation2d initializer close depth overflow"))?;
            Ok(demand)
        };
        if !self.publication.terminal_is_empty() {
            return self.publication.retirement_demands();
        }
        if let Some(owner) = self.active.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body);
        }
        if self.displaced.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&*self.displaced);
        }
        if self.rejected.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&*self.rejected);
        }
        if let Some(candidate) = self.candidate.as_ref() {
            return match self.candidate_disposer.as_ref() {
                Some(disposer) => nested(disposer.retirement_demands(candidate, body)?),
                None => Ok(RetirementDemand { depth: 1, ..Default::default() }),
            };
        }
        if let Some(runtime) = self.runtime.as_ref() {
            return nested(runtime.initialization_retirement_demands(body)?);
        }
        if let Some(owners) = self.owners.as_ref() {
            return if owners.constructor_is_complete() && owners.uninstalled_owners_terminal_is_empty() { Ok(RetirementDemand { depth: 1, ..Default::default() }) } else if owners.constructor_is_complete() { nested(owners.uninstalled_owners_demands(body)?) } else { nested(owners.constructor_demands(body)?) };
        }
        if let Some(copy) = self.copy.as_ref() {
            return copy.close_demands();
        }
        if let Some(closer) = self.closer.as_ref() {
            return nested(closer.retirement_demands(body)?);
        }
        if self.envelope.is_some() {
            return Ok(RetirementDemand { depth: 1, ..Default::default() });
        }
        if let Some(factory) = self.factory_close.as_ref() {
            return nested(factory.demands(body)?);
        }
        if self.snapshot_factory.is_some() {
            return Ok(RetirementDemand { copy_bytes: size_of::<std::sync::Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() });
        }
        Ok(Default::default())
    }

    pub(super) fn close_turn(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneProgress, semio_framework_value::ValueError> {
        use semio_framework_plugin::ArtifactOwnedDisposer;
        use semio_framework_value::{retained_clone::{admit_retained_clone_close, RetainedCloneGrant, RetainedCloneProgress}, FactoryAuthority, ValueError, ValueRefusalKind};
        let item = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if demand == Default::default() {
            return Ok(Default::default());
        }
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "generation2d initializer close exceeds its admitted depth"));
        }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(Default::default());
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if !self.publication.terminal_is_empty() {
            return self.publication.close_step(child).map(|step| step.progress());
        }
        if self.active.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, grant).map(|step| step.progress());
        }
        if self.displaced.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.displaced, &mut self.active, grant).map(|step| step.progress());
        }
        if self.rejected.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.rejected, &mut self.active, grant).map(|step| step.progress());
        }
        if self.candidate.is_some() {
            if self.candidate_disposer.is_none() {
                *self.candidate_disposer = Some(semio_framework_plugin::ArtifactDocumentStoreDisposer::new());
                return Ok(item);
            }
            let candidate = self.candidate.as_mut().expect("observed original candidate store");
            let disposer = self.candidate_disposer.as_mut().expect("observed original candidate disposer");
            let step = disposer.close_step(candidate, child).map_err(|error| ValueError::new(ValueRefusalKind::InvariantViolated, error.describe()))?;
            if disposer.terminal_is_empty(candidate) {
                *self.candidate_disposer = None;
                drop(self.candidate.take());
            }
            return Ok(step.progress().unwrap_or_default());
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let factory = self.snapshot_factory.as_ref().expect("initializer snapshot issuer outlives its runtime");
            let step = runtime.close_step(factory, child)?;
            let step = admit_retained_clone_close(child, step, runtime.terminal_is_empty(), "generation2d initializer runtime close")?;
            if runtime.terminal_is_empty() {
                drop(self.runtime.take());
            }
            return Ok(step.progress());
        }
        if let Some(owners) = self.owners.as_mut() {
            if !owners.constructor_is_complete() {
                return owners.admit_constructor(child).map_err(|(error, receipt)| error.with_retained_progress(receipt));
            }
            if owners.uninstalled_owners_terminal_is_empty() {
                drop(self.owners.take());
                return Ok(item);
            }
            let step = owners.close_uninstalled_owners_step(child)?;
            let step = admit_retained_clone_close(child, step, owners.uninstalled_owners_terminal_is_empty(), "generation2d initializer uninstalled owners close")?;
            return Ok(step.progress());
        }
        if let Some(copy) = self.copy.as_mut() {
            let step = copy.abandon(&mut self.active, grant)?;
            if copy.terminal_is_empty() && self.active.is_none() {
                drop(self.copy.take());
            }
            return Ok(step.progress());
        }
        if self.closer.is_some() {
            let closer = self.closer.as_mut().expect("observed original envelope closer");
            semio_framework_job::InteractiveJob::begin_close(closer);
            let demand = closer.retirement_demands(child.maximum_copy_bytes)?;
            let step = closer.close_granted(child, demand)?;
            if closer.terminal_is_empty() {
                drop(self.closer.take());
            }
            return Ok(step.progress());
        }
        if let Some(envelope) = self.envelope.take() {
            *self.closer = Some(semio_framework_plugin::bounded_document_store_initialization_job::<_,_,::semio_framework_os_kernel::os_store::document_store_owners::BoundedDocumentStoreOwnerProducer>(envelope, semio_s_artifact_procedural_2d_model::GENERATION_2D_SCHEMA, self.operation, self.generation, self.actor.clone()));
            return Ok(item);
        }
        if let Some(factory) = self.factory_close.as_mut() {
            let step = factory.step(child)?;
            let step = admit_retained_clone_close(child, step, factory.terminal_is_empty(), "generation2d initializer issuer close")?;
            if factory.terminal_is_empty() {
                *self.factory_close = None;
            }
            return Ok(step.progress());
        }
        if let Some(factory) = self.snapshot_factory.take() {
            let factory: std::sync::Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            *self.factory_close = Some(FactoryAuthority::new(factory));
            return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() });
        }
        Ok(Default::default())
    }

    fn close_is_empty(&self) -> bool {
        self.envelope.is_none()
            && self.copy.is_none()
            && self.runtime.is_none()
            && self.owners.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.displaced.is_none()
            && self.rejected.is_none()
            && self.candidate_disposer.is_none()
            && self.closer.is_none()
            && self.snapshot_factory.is_none()
            && self.factory_close.is_none()
            && self.publication.terminal_is_empty()
    }

    pub(super) fn close_is_complete(&self) -> bool {
        self.close_is_empty()
    }

    pub(super) fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff && self.close_is_empty()
    }
}

impl Drop for Generation2dStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty_inner()), "Generation2d initializer reached Drop before candidate handoff or terminal-empty close");
    }
}

pub fn generation2d_document_store_initialization_job(
    envelope: store::os_store::store_schema::ArtifactEnvelope<Generation2dSnapshot, Generation2dMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    actor: protocol::ActorId,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<Generation2dSnapshot, Generation2dMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(Generation2dStoreInitializationAuthority::new(envelope, operation, generation, actor)))
}

#[cfg(any(test, feature = "testkit"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Generation2dPublicationHostile {
    Missing,
    WrongOperation,
    WrongGeneration,
    WrongBase,
    WrongParent,
}

#[cfg(any(test, feature = "testkit"))]
pub(super) fn generation2d_publication_hostiles() -> &'static std::sync::Mutex<[Option<Generation2dPublicationHostileLease>; GENERATION2D_PUBLICATION_SLOTS]> {
    static HOSTILES: std::sync::OnceLock<std::sync::Mutex<[Option<Generation2dPublicationHostileLease>; GENERATION2D_PUBLICATION_SLOTS]>> = std::sync::OnceLock::new();
    HOSTILES.get_or_init(|| std::sync::Mutex::new([None; GENERATION2D_PUBLICATION_SLOTS]))
}

#[cfg(any(test, feature = "testkit"))]
#[derive(Clone, Copy)]
struct Generation2dPublicationHostileLease {
    pub(super) operation: u64,
    pub(super) hostile: Generation2dPublicationHostile,
    pub(super) observed: Option<&'static str>,
}

#[cfg(any(test, feature = "testkit"))]
pub fn generation2d_arm_publication_hostile(operation: semio_framework_job::OperationId, hostile: Generation2dPublicationHostile) {
    let mut hostiles = generation2d_publication_hostiles().try_lock().expect("Generation2d hostile publication authority is uncontended");
    let slot = hostiles.iter_mut().find(|slot| slot.is_none()).expect("Generation2d hostile publication authority has a fixed slot");
    *slot = Some(Generation2dPublicationHostileLease { operation: operation.0, hostile, observed: None });
}

#[cfg(any(test, feature = "testkit"))]
pub fn generation2d_take_publication_hostile_observed(operation: semio_framework_job::OperationId) -> Option<&'static str> {
    let mut hostiles = generation2d_publication_hostiles().try_lock().expect("Generation2d hostile publication authority is uncontended");
    let slot = hostiles.iter_mut().find(|slot| slot.is_some_and(|value| value.operation == operation.0))?;
    slot.take()?.observed
}
