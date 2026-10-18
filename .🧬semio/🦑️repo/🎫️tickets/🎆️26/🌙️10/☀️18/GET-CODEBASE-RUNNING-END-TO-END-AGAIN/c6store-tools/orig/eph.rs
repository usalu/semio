//! 🫧️ Exact presence, transient and interaction publication owners.

use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory, ValueError, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value_snapshot_read::*;
use semio_framework_os_store_one_item::*;
use protocol::{Mutation, MutationDiff, apply_operation};
use semio_framework_value::{FromValue, ToValue};
use std::{collections::BTreeMap, marker::PhantomData, sync::Arc, mem::{size_of, size_of_val}};

#[path = "../👥️presence/🦀️.rs"]
pub mod presence;
use presence::{LaneItemReceipt, PRESENCE_PEER_ID_BYTES, PRESENCE_PEER_SLOTS, PresencePeersCapacityError, PresencePeersRetiredEntries, PresencePeersRetirement, PresencePeersRoot};
use presence::{PresencePeerEntry, PresencePeerRootCustody, presence_arc_bytes};

//#region 📬️EphemeralOneItemPublication
pub struct ArtifactEphemeralOneItemPrepared<P> {
    pub next_root: Arc<P>,
}

pub struct ArtifactEphemeralBaseRead<P>(pub ArtifactEphemeralBaseOwner<P>);

/// 🫧️ Which owner a preparation's base came from. Public so an app can build the exact request its
/// own registered ephemeral preparation factory receives, and drive it directly in a conformance
/// test, without standing up the whole presence/transient store around it.
pub enum ArtifactEphemeralBaseOwner<P> {
    Presence(SnapshotRead<P>),
    Transient(Arc<P>),
    TransientRead(SnapshotRead<P>),
}

impl<P> AsRef<P> for ArtifactEphemeralBaseRead<P> {
    fn as_ref(&self) -> &P {
        match &self.0 {
            ArtifactEphemeralBaseOwner::Presence(read) => read.get(),
            ArtifactEphemeralBaseOwner::Transient(root) => root.as_ref(),
            ArtifactEphemeralBaseOwner::TransientRead(read) => read.get(),
        }
    }
}

impl<P> std::ops::Deref for ArtifactEphemeralBaseRead<P> {
    type Target = P;
    fn deref(&self) -> &P {
        self.as_ref()
    }
}

pub struct ArtifactEphemeralOneItemPreparationRequest<P, Mutation> {
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub base: ArtifactEphemeralBaseRead<P>,
    pub mutation: Mutation,
}

pub trait ArtifactEphemeralOneItemPreparation<P, Mutation>: Send {
    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError>;
    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint;
    fn prepared(&self) -> Option<&ArtifactEphemeralOneItemPrepared<P>>;
    fn take_prepared(&mut self) -> Option<ArtifactEphemeralOneItemPrepared<P>>;
    fn cancel(&mut self);
    fn begin_close(&mut self);
    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError>;
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError>;
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_depth_demand(&self) -> Result<usize, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

pub trait ArtifactEphemeralOneItemPreparationFactory<P, Mutation>: Send + Sync + semio_framework_value::FactoryRetirement {
    fn preflight(&self, mutation: &Mutation) -> Result<ArtifactStoreOneItemFootprint, String>;
    fn begin(&self, request: ArtifactEphemeralOneItemPreparationRequest<P, Mutation>) -> Result<Box<dyn ArtifactEphemeralOneItemPreparation<P, Mutation>>, ArtifactEphemeralOneItemPreparationRequest<P, Mutation>>;
}

#[derive(Debug)]
pub struct ArtifactEphemeralOneItemAdmissionRejected<Mutation> {
    pub reason: String,
    pub mutation: Mutation,
}

impl<Mutation> ArtifactEphemeralOneItemAdmissionRejected<Mutation> {
    pub fn into_owners(self) -> (String, Mutation) {
        (self.reason, self.mutation)
    }
}

pub struct ArtifactEphemeralOneItemPublication<P, Mutation> {
    operation: semio_framework_job::OperationId,
    expected_generation: u64,
    footprint: ArtifactStoreOneItemFootprint,
    preparation: std::mem::ManuallyDrop<Option<Box<dyn ArtifactEphemeralOneItemPreparation<P, Mutation>>>>,
    displaced_root_retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    root_retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>,
    returned_read_registry: std::mem::ManuallyDrop<Option<SnapshotReadRegistryAliasRetirement>>,
    returned_read_retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    owned_retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,
    receipt: std::mem::ManuallyDrop<Option<LaneItemReceipt>>,
    /// 📍️ Last checkpoint the preparation owner reported, kept so progress survives the turn the
    /// owner is released on: a host polling `progress()` must never see a gesture walk backwards.
    retained_checkpoint: ArtifactStoreOneItemCheckpoint,
    factory_retirement: [Option<semio_framework_value::FactoryAuthority>; 2],
    attempts: u8,
    published: bool,
    cancel_requested: bool,
    close_started: bool,
    fault: std::mem::ManuallyDrop<Option<ValueError>>,
    phase: ArtifactStoreOneItemPublicationPhase,
}

impl<P, Mutation> ArtifactEphemeralOneItemPublication<P, Mutation> {
    /// 🧫️ Borrows the exact retained preparation for native custody laws.
    #[cfg(any(test,feature="testkit"))]
    pub fn original_preparation(&self) -> Option<&dyn ArtifactEphemeralOneItemPreparation<P, Mutation>> { self.preparation.as_deref() }

    /// 🧪️ Builds a publication already closing around one preparation so native custody laws can walk its retirement demands.
    #[cfg(any(test,feature="testkit"))]
    pub fn closing_with_preparation(operation: semio_framework_job::OperationId, preparation: Box<dyn ArtifactEphemeralOneItemPreparation<P, Mutation>>) -> Self {
        use std::mem::ManuallyDrop as Held;
        Self {
            operation, expected_generation: 0,
            footprint: ArtifactStoreOneItemFootprint::for_ephemeral_item(0),
            preparation: Held::new(Some(preparation)),
            displaced_root_retirement: Held::new(None), root_retirement_factory: Held::new(None),
            returned_read_registry: Held::new(None), returned_read_retirement: Held::new(None),
            owned_retirement_factory: Held::new(None), receipt: Held::new(None),
            retained_checkpoint: Default::default(), factory_retirement: [None, None],
            attempts: 0, published: false, cancel_requested: false, close_started: true,
            fault: Held::new(None), phase: ArtifactStoreOneItemPublicationPhase::Closing,
        }
    }

    pub fn operation(&self) -> semio_framework_job::OperationId {
        self.operation
    }

    pub fn phase(&self) -> ArtifactStoreOneItemPublicationPhase {
        self.phase
    }

    /// 📍️ Monotone gesture progress: the live preparation's own checkpoint while it owns one, and the
    /// last checkpoint it reported once it has been released. Answering `default()` (all zeros) the
    /// moment the owner goes away made a batch's progress snap back to 0 between items — measured on
    /// `🌊️flow` as `preparation progress changed from 22013 to 0` (ticket 26/09/19 `📓️flow.md` §5.4).
    pub fn progress(&self) -> ArtifactStoreOneItemCheckpoint {
        self.preparation.as_ref().map_or(self.retained_checkpoint, |owner| owner.checkpoint())
    }

    pub fn fault(&self) -> Option<&str> { self.fault.as_ref().map(|fault|fault.message.as_ref()) }

    /// 🚫️ Borrows the one original refusal and its actual consumed physical currencies.
    pub fn preparation_refusal(&self) -> Option<&ValueError> { self.fault.as_ref() }

    fn advance_preparation(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ArtifactStoreOneItemAdvance> {
        let result = self.preparation.as_mut().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"ephemeral publication lost its preparation owner")).and_then(|owner| {
            let grant=ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), ..grant };
            let step=owner.advance(grant)?;
            validate_original_preparation_step(step,grant,owner.prepared().is_some())
        });
        match result {
            Ok(step)=>Ok(step),
            Err(reason)=>{*self.fault=Some(reason);self.begin_close();Err(ArtifactStoreOneItemAdvance::PreparationRefused)}
        }
    }

    pub fn retry(&mut self) -> bool {
        if self.phase != ArtifactStoreOneItemPublicationPhase::AwaitingAck || self.attempts >= ARTIFACT_STORE_ONE_ITEM_MAXIMUM_RETRIES {
            return false;
        }
        self.attempts += 1;
        true
    }

    pub fn acknowledge(&mut self) -> bool {
        if self.phase != ArtifactStoreOneItemPublicationPhase::AwaitingAck {
            return false;
        }
        self.phase = ArtifactStoreOneItemPublicationPhase::Closing;
        self.begin_close();
        true
    }

    pub fn begin_close(&mut self) {
        if self.close_started {
            return;
        }
        self.close_started = true;
        self.phase = if self.phase == ArtifactStoreOneItemPublicationPhase::Complete { ArtifactStoreOneItemPublicationPhase::Complete } else { ArtifactStoreOneItemPublicationPhase::Closing };
        if let Some(owner) = self.preparation.as_mut() {
            if !self.published {
                self.cancel_requested = true;
                owner.cancel();
            }
            owner.begin_close();
        }
    }

    /// 📐️ Borrows the original cleanup frontier without spending its physical authority.
    pub fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> where P: Send + Sync + 'static {
        use semio_framework_value::RetirementDemand;
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "ephemeral cleanup depth overflow"))?; Ok(demand) };
        if let Some(owner) = self.preparation.as_ref() {
            if owner.terminal_is_empty() { return Ok(RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() }); }
            return nested(RetirementDemand { copy_bytes: owner.next_close_copy_byte_demand()?, capacity_bytes: owner.next_close_capacity_byte_demand(body)?, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()? });
        }
        if let Some(owner) = self.returned_read_retirement.as_ref() { return nested(artifact_retirement_box_demands(owner, body)?); }
        if let Some(registry) = self.returned_read_registry.as_ref().filter(|registry| registry.has_returned()) { return nested(registry.returned_admission_demands::<P>(|_| RetirementDemand { capacity_bytes: ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(), depth: 1, ..Default::default() }).map_err(SnapshotReadLeaseRefusal::into_value_error)?); }
        if let Some(owner) = self.displaced_root_retirement.as_ref() { return nested(artifact_retirement_box_demands(owner, body)?); }
        if self.returned_read_registry.is_some() { return snapshot_registry_alias_demands(&self.returned_read_registry); }
        if let Some(factory) = self.factory_retirement.iter().flatten().next() { return nested(factory.demands(body)?); }
        if self.root_retirement_factory.is_some() || self.owned_retirement_factory.is_some() || self.receipt.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if let Some(fault) = self.fault.as_ref() { let release_bytes=match &fault.message{std::borrow::Cow::Borrowed(_)=>0,std::borrow::Cow::Owned(text)=>text.capacity()};return Ok(RetirementDemand { release_bytes, depth: 1, ..Default::default() }); }
        Ok(RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }

    /// ♻️ Closes one original child or its separately admitted physical frame.
    pub fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError> where P: Send + Sync + 'static {
        let grant = grant.retained_grant();let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "ephemeral cleanup exceeds admitted depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.begin_close();
        if let Some(owner) = self.preparation.as_mut() {
            if owner.terminal_is_empty() { self.retained_checkpoint = owner.checkpoint();let bytes = size_of_val(owner.as_ref());drop(self.preparation.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..empty })); }
            let child = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: grant.maximum_copy_bytes, maximum_capacity_bytes: grant.maximum_capacity_bytes, maximum_release_bytes: grant.maximum_release_bytes, maximum_depth: grant.maximum_depth - 1 };
            let step = owner.close_step(child)?;let step = semio_framework_value::retained_clone::admit_retained_clone_close(child.retained_grant(), step, owner.terminal_is_empty(), "ephemeral original preparation")?;return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.returned_read_retirement.is_some() || self.returned_read_registry.as_ref().is_some_and(|registry| registry.has_returned()) {
            let registry = self.returned_read_registry.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "returned ephemeral child lost its registry"))?;let factory = self.owned_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "returned ephemeral child lost its issuer"))?;
            return advance_returned_snapshot_read(registry, &mut self.returned_read_retirement, factory, RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant }).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.displaced_root_retirement.is_some() { return artifact_retirement_box_close_step(&mut self.displaced_root_retirement, RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant }).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if self.returned_read_registry.is_some() { return snapshot_registry_alias_close_step(&mut self.returned_read_registry, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(index) = self.factory_retirement.iter().position(Option::is_some) {
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };let factory = self.factory_retirement[index].as_mut().expect("observed original factory");let step = factory.step(child)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "ephemeral original factory")?;if factory.terminal_is_empty() { self.factory_retirement[index] = None; }return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(factory) = self.root_retirement_factory.take() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;self.factory_retirement[0] = Some(semio_framework_value::FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); }
        if let Some(factory) = self.owned_retirement_factory.take() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;self.factory_retirement[1] = Some(semio_framework_value::FactoryAuthority::new(factory));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); }
        if self.receipt.take().is_some() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); }
        if let Some(fault) = self.fault.take() { drop(fault);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty })); }
        self.phase = ArtifactStoreOneItemPublicationPhase::Complete;Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == ArtifactStoreOneItemPublicationPhase::Complete
            && self.preparation.is_none()
            && self.displaced_root_retirement.is_none()
            && self.root_retirement_factory.is_none()
            && self.returned_read_registry.is_none()
            && self.returned_read_retirement.is_none()
            && self.owned_retirement_factory.is_none()
            && self.receipt.is_none()
            && self.fault.is_none()
            && self.factory_retirement.iter().all(Option::is_none)
    }
}

impl<P, Mutation> Drop for ArtifactEphemeralOneItemPublication<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "ephemeral one-item publication reached Drop without its terminal-empty witness");
    }
}
//#endregion 📬️EphemeralOneItemPublication

/// 👥️ The PRESENCE lane's store: ephemeral SHARED state — a last-writer-wins roster, NOT an
/// event log.
///
/// 🎯️ Why not `ArtifactStore` (which `ConfigStore`/`DraftStore` both alias): presence has no
/// history, no undo, no checkpoints and no merge. Each peer is the sole author of its own presence
/// value, and the wire already carries whole encoded snapshots per peer
/// (`ClientFrame::Presence { peer }` / `ServerFrame::Presence { peers }`), so a later frame from a
/// peer simply SUPERSEDES its earlier one. Modelling that as an op log would mint an unbounded
/// history of cursor positions nobody can ever undo.
///
/// `generation` bumps on every local change so the host can tell "something to broadcast" from
/// "nothing changed" without diffing snapshots — the signal a heartbeat coalescer throttles on.
pub struct PresenceStore<P, Mutation> {
    local: std::mem::ManuallyDrop<Option<Arc<P>>>,
    peers: std::mem::ManuallyDrop<Option<Arc<PresencePeersRoot<P>>>>,
    peer_retirement_factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    local_retirement_factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    local_reads: std::mem::ManuallyDrop<Option<SnapshotReadRegistryHandle>>,
    active_returned_local: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    generation: u64,
    close_started: bool,
    _mutation: PhantomData<fn() -> Mutation>,
}

pub struct PresencePeersPublication<P> {
    base_root: std::mem::ManuallyDrop<Option<Arc<PresencePeersRoot<P>>>>,
    candidate: std::mem::ManuallyDrop<Option<PresencePeersRoot<P>>>,
    created: std::mem::ManuallyDrop<PresencePeersRetiredEntries<P>>,
    retired: std::mem::ManuallyDrop<PresencePeersRetiredEntries<P>>,
    active: std::mem::ManuallyDrop<Option<PresencePeersRetirement<P>>>,
    factory: std::mem::ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>,
    factory_close: Option<semio_framework_value::FactoryAuthority>,
    prune_cursor: usize,
    pruning_complete: bool,
}

pub struct PresencePeersCommit<P> {
    base_root: std::mem::ManuallyDrop<Arc<PresencePeersRoot<P>>>,
    root: std::mem::ManuallyDrop<Arc<PresencePeersRoot<P>>>,
    retirement: std::mem::ManuallyDrop<Option<PresencePeersRetirement<P>>>,
    factory: std::mem::ManuallyDrop<Arc<dyn SnapshotRetirementFactory<P>>>,
    transferred: bool,
}
impl<P> PresencePeersCommit<P> {
    fn into_parts(mut self) -> (Arc<PresencePeersRoot<P>>, Arc<PresencePeersRoot<P>>, Option<PresencePeersRetirement<P>>, Arc<dyn SnapshotRetirementFactory<P>>) { self.transferred = true; unsafe { (std::mem::ManuallyDrop::take(&mut self.base_root), std::mem::ManuallyDrop::take(&mut self.root), std::mem::ManuallyDrop::take(&mut self.retirement), std::mem::ManuallyDrop::take(&mut self.factory)) } }
}
impl<P> Drop for PresencePeersCommit<P> { fn drop(&mut self) { assert!(std::thread::panicking() || self.transferred, "original peer commit requires consuming admitted publication or retirement"); } }

#[path = "../👥️presence/🚪️io/📬️publication/🦀️.rs"]
mod presence_publication_io;

#[path="../👥️presence/🔗️read/🦀️.rs"]
mod presence_peer_read;
pub use presence_peer_read::PresencePeersRead;

#[path = "../👥️presence/🚫️rejection/🦀️.rs"]
mod presence_peer_rejection;
pub use presence_peer_rejection::PresencePeerAdmissionRejected;

pub use semio_framework_os_presence_native_snapshot::ArtifactPresenceSnapshot;

impl<P: Send + Sync + 'static> PresencePeersPublication<P> {
    pub fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() {
            let mut demand = active.retirement_demands(body)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "presence peer publication depth overflow"))?;
            return Ok(demand);
        }
        if self.candidate.is_none() && self.created.len == 0 && self.retired.len == 0 && self.base_root.is_none() {
            if self.factory.is_some() { return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
            if let Some(factory) = self.factory_close.as_ref() { let mut demand = factory.demands(body)?; demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "peer publication factory depth overflow"))?; return Ok(demand); }
        }
        Ok(semio_framework_value::RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{admit_retained_clone_close, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "presence peer publication exceeds admitted depth")); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let moved = RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty });
        if let Some(active) = self.active.as_mut() {
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = active.close_step(child)?;
            let step = admit_retained_clone_close(child, step, active.terminal_is_empty(), "presence peer publication cleanup")?;
            if active.terminal_is_empty() { drop(self.active.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(candidate) = self.candidate.as_mut() {
            if candidate.len != 0 {
                candidate.len -= 1;
                *self.active = Some(PresencePeersRetirement::new(PresencePeersRetiredEntries::one(candidate.entries[candidate.len].take()), self.factory.as_ref().expect("retained original peer factory").clone()));
            } else {
                drop(self.candidate.take());
            }
            return Ok(moved);
        }
        if self.retired.len != 0 {
            self.retired.len -= 1;
            let index = self.retired.len;
            *self.active = Some(PresencePeersRetirement::new(PresencePeersRetiredEntries::one(self.retired.entries[index].take()), self.factory.as_ref().expect("retained original peer factory").clone()));
            return Ok(moved);
        }
        if self.created.len != 0 {
            let created = std::mem::replace(&mut *self.created, PresencePeersRetiredEntries::empty());
            *self.active = Some(PresencePeersRetirement::new(created, self.factory.as_ref().expect("retained original peer factory").clone()));
            return Ok(moved);
        }
        if let Some(root) = self.base_root.take() {
            if Arc::strong_count(&root) == 1 { *self.active = Some(PresencePeersRetirement::from_root(root, Some(self.factory.as_ref().expect("retained original peer factory").clone()), PresencePeerRootCustody::BaseAlias)); } else { drop(root); }
            return Ok(moved);
        }
        if let Some(factory) = self.factory.take() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factory_close = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })); }
        if let Some(factory) = self.factory_close.as_mut() { let step = factory.step(child)?; let step = admit_retained_clone_close(child, step, factory.terminal_is_empty(), "original peer publication factory")?; if factory.terminal_is_empty() { self.factory_close = None; } return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) }); }
        Ok(RetainedCloneStep::Complete(empty))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.base_root.is_none() && self.candidate.is_none() && self.created.len == 0 && self.retired.len == 0 && self.active.is_none() && self.factory.is_none() && self.factory_close.is_none()
    }
}

impl<P> Drop for PresencePeersPublication<P> {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(
                std::thread::panicking() || (self.base_root.is_none() && self.candidate.is_none() && self.created.len == 0 && self.retired.len == 0 && self.active.is_none() && self.factory.is_none() && self.factory_close.is_none()),
                "incomplete presence peer publication reached Drop without bounded disposal"
            );
        }
    }
}

#[cfg(test)]
use semio_framework_value_snapshot_read::presence_test_retirement;

#[path = "../👥️presence/♻️retirement/🦀️.rs"]
mod presence_retirement;
pub use presence_retirement::PresenceStoreRetirement;

// 🚫️async: E1 impl of external `Default` — signature fixed outside this repo.
impl<P: Clone + Default, Mutation: self::Mutation<P>> Default for PresenceStore<P, Mutation> {
    fn default() -> Self {
        Self::new(P::default())
    }
}

impl<P: Clone, Mutation: self::Mutation<P>> PresenceStore<P, Mutation> {
    /// 🏗️ A roster holding only this actor's own initial presence.
    /// 🚫️async: E1 pure constructor, consumed by `Default::default()` — see R9.
    pub fn new(local: P) -> Self {
        Self {
            local: std::mem::ManuallyDrop::new(Some(Arc::new(local))),
            peers: std::mem::ManuallyDrop::new(Some(Arc::new(PresencePeersRoot::empty()))),
            peer_retirement_factory: None,
            local_retirement_factory: None,
            local_reads: std::mem::ManuallyDrop::new(Some(SnapshotReadRegistryHandle::new())),
            active_returned_local: std::mem::ManuallyDrop::new(None),
            generation: 0,
            close_started: false,
            _mutation: PhantomData,
        }
    }

    /// 👤️ This actor's own presence — what gets broadcast.
    pub fn local(&self) -> &P {
        self.local.as_ref().expect("live local presence owner").as_ref()
    }

    /// 🧵️ Captures the event-maintained local presence root without cloning its payload.
    pub fn local_read(&self) -> Result<SnapshotRead<P>, String>
    where
        P: Send + Sync + 'static,
    {
        if self.close_started || self.local_retirement_factory.is_none() {
            return Err("presence local read requires a live exact local retirement owner".into());
        }
        let owner = Arc::clone(self.local.as_ref().expect("live local presence owner"));
        let lease = self.local_reads.as_ref().expect("live presence read registry").try_issue(owner.clone()).map_err(|_| "presence local read registry is busy or exhausted".to_string())?;
        Ok(SnapshotRead::new(owner, lease))
    }

    pub fn install_local_retirement_factory(&mut self, factory: Arc<dyn SnapshotRetirementFactory<P>>) -> Result<(), String> {
        if self.close_started || self.local_retirement_factory.is_some() {
            return Err("presence local retirement factory is already installed".into());
        }
        self.local_retirement_factory = Some(factory);
        Ok(())
    }

    pub fn maintenance_local_reads_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>
    where P: Send + Sync + 'static,
    {
        match self.local_reads.as_ref() { Some(reads) => presence_retirement::advance_returned_local(reads, &mut self.active_returned_local, self.local_retirement_factory.as_ref(), grant), None => Ok(RetainedCloneStep::Complete(Default::default())) }
    }
    /// 📏️ Quotes the next returned local-read maintenance turn on every axis without advancing it.
    pub fn maintenance_local_reads_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where P: Send + Sync + 'static,
    {
        use semio_framework_value::RetirementDemand;
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "presence local maintenance depth overflow"))?; Ok(demand) };
        if let Some(active) = self.active_returned_local.as_ref() { return nested(artifact_retirement_box_demands(active, body)?); }
        if let Some(reads) = self.local_reads.as_ref().filter(|reads| reads.has_returned()) {
            let factory = self.local_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "presence returned read lacks its original factory"))?;
            return nested(reads.returned_admission_demands::<P>(|root| RetirementDemand { capacity_bytes: factory.retirement_birth_bytes(root), depth: 1, ..Default::default() }).map_err(SnapshotReadLeaseRefusal::into_value_error)?);
        }
        Ok(Default::default())
    }
    pub fn local_read_maintenance_is_idle(&self) -> bool { self.active_returned_local.is_none() && self.local_reads.as_ref().is_none_or(|reads| !reads.has_returned()) }

    /// ✍️ Applies local presence operations atomically, bumping `generation` after the full batch.
    pub async fn apply(&mut self, mutations: &[Mutation]) -> protocol::MutationApplyResult<()>
    where
        P: Send + Sync + 'static,
    {
        if self.close_started {
            return Err(protocol::MutationApplyError::new("mutation.apply.store-closed", "presence store no longer admits local mutations"));
        }
        if mutations.is_empty() {
            return Ok(());
        }
        let previous = self.local_read().map_err(|reason| protocol::MutationApplyError::new("mutation.apply.local-owner", reason))?;
        let mut candidate = self.local.as_ref().expect("live local presence owner").as_ref().clone();
        for mutation in mutations {
            candidate = apply_operation::<P, Mutation>(&candidate, mutation, 0).0?;
        }
        if !mutations.is_empty() {
            *self.local = Some(Arc::new(candidate));
            self.generation = self.generation.wrapping_add(1);
        }
        previous.return_to_registry();
        Ok(())
    }

    pub fn begin_publish_one(
        &self,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        mutation: Mutation,
        factory: Option<&dyn ArtifactEphemeralOneItemPreparationFactory<P, Mutation>>,
        root_retirement_factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    ) -> Result<ArtifactEphemeralOneItemPublication<P, Mutation>, ArtifactEphemeralOneItemAdmissionRejected<Mutation>>
    where
        P: Send + Sync + 'static,
    {
        let reject = |reason: String, mutation: Mutation| ArtifactEphemeralOneItemAdmissionRejected { reason, mutation };
        if self.close_started {
            return Err(reject("presence store no longer admits local publication".into(), mutation));
        }
        if self.generation != expected_generation {
            return Err(reject("presence publication generation is stale".into(), mutation));
        }
        let Some(factory) = factory else {
            return Err(reject("presence publication requires an explicit app-owned retained preparation factory".into(), mutation));
        };
        let Some(root_retirement_factory) = root_retirement_factory else {
            return Err(reject("presence publication requires an explicit local-root retirement factory".into(), mutation));
        };
        let footprint = match factory.preflight(&mutation) {
            Ok(footprint) if footprint.is_admissible() => footprint,
            Ok(_) => return Err(reject("presence publication footprint exceeds its fixed item or byte capacity".into(), mutation)),
            Err(reason) => return Err(reject(reason, mutation)),
        };
        if !self.local_retirement_factory.as_ref().is_some_and(|installed| Arc::ptr_eq(installed, &root_retirement_factory)) {
            return Err(reject("presence publication requires its exact installed local-root retirement factory".into(), mutation));
        }
        let base = match self.local_read() {
            Ok(read) => ArtifactEphemeralBaseRead(ArtifactEphemeralBaseOwner::Presence(read)),
            Err(reason) => return Err(reject(reason, mutation)),
        };
        let request = ArtifactEphemeralOneItemPreparationRequest { operation, generation: semio_framework_job::Generation(expected_generation), base, mutation };
        let preparation = match factory.begin(request) {
            Ok(preparation) => preparation,
            Err(request) => return Err(reject("presence preparation factory rejected its exact owner bundle".into(), request.mutation)),
        };
        Ok(ArtifactEphemeralOneItemPublication {
            operation,
            expected_generation,
            footprint,
            preparation: std::mem::ManuallyDrop::new(Some(preparation)),
            displaced_root_retirement: std::mem::ManuallyDrop::new(None),
            root_retirement_factory: std::mem::ManuallyDrop::new(Some(root_retirement_factory)),
            returned_read_registry: std::mem::ManuallyDrop::new(None),
            returned_read_retirement: std::mem::ManuallyDrop::new(None),
            owned_retirement_factory: std::mem::ManuallyDrop::new(None),
            receipt: std::mem::ManuallyDrop::new(None),
            retained_checkpoint: ArtifactStoreOneItemCheckpoint::default(),
            factory_retirement: [None, None],
            attempts: 0,
            published: false,
            cancel_requested: false,
            close_started: false,
            fault: std::mem::ManuallyDrop::new(None),
            phase: ArtifactStoreOneItemPublicationPhase::Preparing,
        })
    }

    pub fn advance_publish_one(&mut self, publication: &mut ArtifactEphemeralOneItemPublication<P, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, String>
    where
        P: Send + Sync + 'static,
    {
        if publication.phase == ArtifactStoreOneItemPublicationPhase::Complete {
            return Ok(ArtifactStoreOneItemAdvance::Complete);
        }
        if publication.phase == ArtifactStoreOneItemPublicationPhase::Closing {
            return Ok(ArtifactStoreOneItemAdvance::Blocked);
        }
        if publication.phase == ArtifactStoreOneItemPublicationPhase::AwaitingAck {
            return Ok(ArtifactStoreOneItemAdvance::AwaitingAck(publication.receipt.expect("presence publication awaiting ACK retains its receipt")));
        }
        if self.close_started {
            publication.begin_close();
            return Err("presence store closed before publication".into());
        }
        if publication.cancel_requested {
            publication.begin_close();
            return Ok(ArtifactStoreOneItemAdvance::Blocked);
        }
        if self.generation != publication.expected_generation {
            *publication.fault = Some(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"presence publication became stale before commit"));
            publication.phase = ArtifactStoreOneItemPublicationPhase::Fault;
            publication.begin_close();
            return Err("presence publication became stale before commit".into());
        }
        if !grant.permits_one() {
            return Ok(ArtifactStoreOneItemAdvance::Blocked);
        }
        match publication.phase {
            ArtifactStoreOneItemPublicationPhase::Preparing => match publication.advance_preparation(grant) {
                Err(refused)=>Ok(refused),
                Ok(ArtifactStoreOneItemPreparationStep::Progress(checkpoint, ownership)) => Ok(ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint, ownership)),
                Ok(ArtifactStoreOneItemPreparationStep::Prepared(checkpoint, ownership)) => {
                    publication.phase = ArtifactStoreOneItemPublicationPhase::PreparingCursor;
                    Ok(ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint, ownership))
                }
                Ok(ArtifactStoreOneItemPreparationStep::Blocked) => Ok(ArtifactStoreOneItemAdvance::Blocked),
            },
            ArtifactStoreOneItemPublicationPhase::PreparingCursor => {
                if publication.preparation.as_ref().and_then(|owner| owner.prepared()).is_none() {
                    return Err("presence prepared root is absent".into());
                }
                publication.phase = ArtifactStoreOneItemPublicationPhase::PreflightingCommit;
                Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()))
            }
            ArtifactStoreOneItemPublicationPhase::PreflightingCommit => {
                if self.generation == u64::MAX || publication.footprint.work_items == 0 {
                    return Err("presence publication fixed commit authority is exhausted".into());
                }
                publication.phase = ArtifactStoreOneItemPublicationPhase::Publishing;
                Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()))
            }
            ArtifactStoreOneItemPublicationPhase::Publishing => {
                let prepared = publication.preparation.as_mut().and_then(|owner| owner.take_prepared()).ok_or_else(|| "presence publication lost its prepared root".to_string())?;
                let generation_before = self.generation;
                let previous = self.local.replace(prepared.next_root).expect("live original local presence");
                let retirement_factory = publication.root_retirement_factory.as_ref().ok_or_else(|| "presence publication lost its local-root retirement factory".to_string())?;
                *publication.displaced_root_retirement = Some(ArtifactStoreDisplacedSnapshotRetirement::boxed(previous, Arc::clone(retirement_factory)));
                self.generation += 1;
                let receipt = LaneItemReceipt { generation_before, generation_after: self.generation };
                publication.expected_generation = self.generation;
                *publication.receipt = Some(receipt);
                publication.published = true;
                publication.phase = ArtifactStoreOneItemPublicationPhase::AwaitingAck;
                Ok(ArtifactStoreOneItemAdvance::Published(receipt))
            }
            ArtifactStoreOneItemPublicationPhase::Fault => Ok(ArtifactStoreOneItemAdvance::PreparationRefused),
            ArtifactStoreOneItemPublicationPhase::AwaitingAck | ArtifactStoreOneItemPublicationPhase::Closing | ArtifactStoreOneItemPublicationPhase::Complete => unreachable!("handled presence publication control phase"),
        }
    }

    pub fn cancel_publish_one(&mut self, publication: &mut ArtifactEphemeralOneItemPublication<P, Mutation>) -> bool {
        if matches!(publication.phase, ArtifactStoreOneItemPublicationPhase::Complete | ArtifactStoreOneItemPublicationPhase::Closing) {
            return false;
        }
        publication.cancel_requested = true;
        publication.begin_close();
        true
    }

    /// 🎯️ Applies one retained publication item against an exact generation and returns its
    /// per-lane receipt. A stale item is rejected before its mutation is inspected.
    pub fn apply_one(&mut self, expected_generation: u64, mutation: Mutation) -> Result<LaneItemReceipt, (String, Mutation)>
    where
        P: Send + Sync + 'static,
    {
        if self.close_started {
            return Err(("presence store no longer admits local mutations".into(), mutation));
        }
        if self.generation != expected_generation {
            return Err(("presence publication generation is stale".into(), mutation));
        }
        let before = self.generation;
        let previous = match self.local_read() {
            Ok(previous) => previous,
            Err(reason) => return Err((reason, mutation)),
        };
        let candidate = match apply_operation::<P, Mutation>(self.local.as_ref().expect("live local presence owner").as_ref(), &mutation, 0).0 {
            Ok(candidate) => candidate,
            Err(error) => return Err((error.to_string(), mutation)),
        };
        *self.local = Some(Arc::new(candidate));
        self.generation = self.generation.wrapping_add(1);
        previous.return_to_registry();
        Ok(LaneItemReceipt { generation_before: before, generation_after: self.generation })
    }

    pub fn peers_root(&self) -> Arc<PresencePeersRoot<P>> {
        Arc::clone(self.peers.as_ref().expect("live peer presence owner"))
    }

    pub fn install_peer_retirement_factory(&mut self, factory: Arc<dyn SnapshotRetirementFactory<P>>) -> Result<(), String> {
        if self.close_started || self.peer_retirement_factory.is_some() {
            return Err("presence peer retirement factory is already installed".into());
        }
        self.peer_retirement_factory = Some(factory);
        Ok(())
    }

    pub fn peer_retirement_available(&self) -> bool {
        self.peer_retirement_factory.is_some()
    }

    /// 🔗️ The installed peer-root retirement issuer, lent to a captured alias so its last holder can retire the roster it ends up owning.
    pub fn peer_retirement_factory(&self) -> Option<Arc<dyn SnapshotRetirementFactory<P>>> {
        self.peer_retirement_factory.clone()
    }

    pub fn begin_peer_retirement(&self, retired: PresencePeersRetiredEntries<P>) -> Result<PresencePeersRetirement<P>, PresencePeersRetiredEntries<P>>
    where
        P: Send + Sync + 'static,
    {
        let Some(factory) = self.peer_retirement_factory.as_ref() else { return Err(retired) };
        Ok(PresencePeersRetirement::new(retired, factory.clone()))
    }

    pub fn begin_peer_publication(&self, grant: RetainedCloneGrant) -> Result<(PresencePeersPublication<P>, RetainedCloneProgress), ValueError>
    where P: Send + Sync + 'static,
    {
        if self.close_started { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "presence store no longer admits peer publication")); }
        let factory = self.peer_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "presence publication has no original snapshot retirement factory"))?;
        PresencePeersPublication::new(self.peers.as_ref().expect("live original peer owner"), factory, grant)
    }

    pub fn publish_peer_commit(&mut self, commit: PresencePeersCommit<P>) -> Result<Option<PresencePeersRetirement<P>>, PresencePeersCommit<P>>
    where
        P: Send + Sync + 'static,
    {
        if self.close_started || !Arc::ptr_eq(self.peers.as_ref().expect("live peer presence owner"), &commit.base_root) || !self.peer_retirement_factory.as_ref().is_some_and(|factory| Arc::ptr_eq(factory, &commit.factory)) {
            return Err(commit);
        }
        let (base_root, root, retired, factory) = commit.into_parts();
        let previous = self.peers.replace(root).expect("live original peer presence");
        let mut retirement = retired.unwrap_or_else(|| PresencePeersRetirement::new(PresencePeersRetiredEntries::empty(), factory));
        *retirement.root = Some(previous);
        drop(base_root);
        Ok(Some(retirement))
    }

    /// 🔢️ Bumps on every local change; never on adopting a remote peer (that needs no rebroadcast).
    pub async fn generation(&self) -> u64 {
        self.generation
    }

    /// 🪪️ Reads the event-maintained generation at an already-owned operation boundary.
    pub fn generation_now(&self) -> u64 {
        self.generation
    }
}

/// 🫧️ The TRANSIENT lane's store: ephemeral LOCAL-ONLY UI state.
///
/// 🎯️ Presence minus the roster. Nothing here is ever shared, persisted, packed, checkpointed or
/// undone — it is exactly the state that used to hide in plugin `thread_local!`s and untyped shell
/// fields, given a typed home so it can be reached only through `Emit`/`Lanes` like every other
/// lane. If a value must survive a reload it belongs in `config`; if a peer must see it, in
/// `presence`; if it is document content, in the artifact (or its draft).
pub struct TransientStore<P, Mutation> {
    current: Option<Arc<P>>,
    reads: Option<SnapshotReadRegistryHandle>,
    close_started: bool,
    active_returned_read: Option<Box<dyn ErasedSnapshotRetirement>>,
    generation: u64,
    _mutation: PhantomData<fn() -> Mutation>,
}

// 🚫️async: E1 impl of external `Default` — signature fixed outside this repo.
impl<P: Clone + Default, Mutation: self::Mutation<P>> Default for TransientStore<P, Mutation> {
    fn default() -> Self {
        Self::new(P::default())
    }
}

impl<P: Clone, Mutation: self::Mutation<P>> TransientStore<P, Mutation> {
    /// 🚫️async: E1 pure constructor, consumed by `Default::default()` — see R9.
    pub fn new(current: P) -> Self {
        Self { current: Some(Arc::new(current)), reads: Some(SnapshotReadRegistryHandle::new()), close_started: false, active_returned_read: None, generation: 0, _mutation: PhantomData }
    }

    pub async fn current(&self) -> &P {
        self.current.as_ref().expect("live transient root remains").as_ref()
    }

    /// 🧵️ Captures the event-maintained transient root without cloning its payload.
    pub fn current_root(&self) -> Arc<P> {
        self.current.as_ref().expect("live transient root remains").clone()
    }

    /// 🧵️ Captures one tracked transient root capability for retained work.
    pub fn current_read(&self) -> Result<SnapshotRead<P>, String>
    where
        P: Send + Sync + 'static,
    {
        let owner = Arc::clone(self.current.as_ref().expect("live transient root remains"));
        let lease = self.reads.as_ref().expect("live transient read registry remains").try_issue(owner.clone()).map_err(|_| "transient read registry is busy or exhausted".to_string())?;
        Ok(SnapshotRead::new(owner, lease))
    }

    /// 🧰️ Captures the same tracked capability across a heterogeneous owner registry.
    pub fn current_read_erased(&self) -> Result<ErasedSnapshotRead, String>
    where
        P: Send + Sync + 'static,
    {
        let owner = Arc::clone(self.current.as_ref().expect("live transient root remains"));
        let lease = self.reads.as_ref().expect("live transient read registry remains").try_issue(owner.clone()).map_err(|_| "transient read registry is busy or exhausted".to_string())?;
        Ok(ErasedSnapshotRead::new(owner, lease))
    }

    /// 🧹️ Advances one returned read through its exact owned-value retirement authority.
    pub fn maintenance_returned_reads_step(&mut self, factory: &Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>
    where
        P: Send + Sync + 'static,
    {
        match self.reads.as_ref() { Some(reads) => advance_returned_snapshot_read(reads, &mut self.active_returned_read, factory, grant), None => Ok(RetainedCloneStep::Complete(Default::default())) }
    }

    /// 🧹️ Advances one returned read through a root-only retirement authority, for stores that own no owned-value factory.
    pub fn maintenance_returned_reads_step_root(&mut self, factory: &dyn SnapshotRetirementFactory<P>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>
    where
        P: Send + Sync + 'static,
    {
        match self.reads.as_ref() { Some(reads) => advance_returned_snapshot_read_root(reads, &mut self.active_returned_read, factory, grant), None => Ok(RetainedCloneStep::Complete(Default::default())) }
    }

    /// 📏️ Quotes the next returned read's birth or physical retirement under a root-only retirement authority.
    pub fn maintenance_returned_reads_demands_root(&self, factory: &dyn SnapshotRetirementFactory<P>, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where
        P: Send + Sync + 'static,
    {
        if let Some(owner) = self.active_returned_read.as_ref() { return artifact_retirement_box_demands(owner, body); }
        match self.reads.as_ref() {
            Some(reads) if reads.has_returned() => reads.returned_admission_demands::<P>(|root| semio_framework_value::RetirementDemand { capacity_bytes: factory.retirement_birth_bytes(root), depth: 1, ..Default::default() }).map_err(SnapshotReadLeaseRefusal::into_value_error),
            _ => Ok(Default::default()),
        }
    }

    /// 🧾️ A returned read, or its active retirement, still awaits the returned-read pump.
    pub fn returned_read_pending(&self) -> bool {
        self.active_returned_read.is_some() || self.reads.as_ref().is_some_and(|reads| reads.has_returned())
    }

    pub fn returned_reads_terminal_is_empty(&self) -> bool {
        self.active_returned_read.is_none() && self.reads.as_ref().is_none_or(|reads| reads.terminal_is_empty())
    }

    /// 🔒️ A live tracked read, not yet returned, is the only owner that keeps the read registry from terminal-empty custody.
    pub fn live_read_holds_registry(&self) -> bool {
        self.active_returned_read.is_none() && self.reads.as_ref().is_some_and(|reads| !reads.has_returned() && !reads.terminal_is_empty())
    }

    pub fn current_weak(&self) -> std::sync::Weak<P> {
        self.current.as_ref().map_or_else(std::sync::Weak::new, Arc::downgrade)
    }

    pub fn current_matches(&self, root: &std::sync::Weak<P>) -> bool {
        root.upgrade().is_some_and(|root| self.current.as_ref().is_some_and(|current| Arc::ptr_eq(&root, current)))
    }

    /// ✍️ Applies transient operations atomically, without history.
    pub async fn apply(&mut self, mutations: &[Mutation]) -> protocol::MutationApplyResult<()> {
        let mut candidate = self.current.as_ref().expect("live transient root remains").as_ref().clone();
        for mutation in mutations {
            candidate = apply_operation::<P, Mutation>(&candidate, mutation, 0).0?;
        }
        if !mutations.is_empty() {
            self.current = Some(Arc::new(candidate));
            self.generation = self.generation.wrapping_add(1);
        }
        Ok(())
    }

    pub fn begin_publish_one(
        &self,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        mutation: Mutation,
        factory: Option<&dyn ArtifactEphemeralOneItemPreparationFactory<P, Mutation>>,
        root_retirement_factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    ) -> Result<ArtifactEphemeralOneItemPublication<P, Mutation>, ArtifactEphemeralOneItemAdmissionRejected<Mutation>> {
        let reject = |reason: String, mutation: Mutation| ArtifactEphemeralOneItemAdmissionRejected { reason, mutation };
        if self.close_started || self.generation != expected_generation {
            return Err(reject("transient publication generation is stale".into(), mutation));
        }
        let Some(factory) = factory else {
            return Err(reject("transient publication requires an explicit app-owned retained preparation factory".into(), mutation));
        };
        let Some(root_retirement_factory) = root_retirement_factory else {
            return Err(reject("transient publication requires an explicit local-root retirement factory".into(), mutation));
        };
        let footprint = match factory.preflight(&mutation) {
            Ok(footprint) if footprint.is_admissible() => footprint,
            Ok(_) => return Err(reject("transient publication footprint exceeds its fixed item or byte capacity".into(), mutation)),
            Err(reason) => return Err(reject(reason, mutation)),
        };
        let request =
            ArtifactEphemeralOneItemPreparationRequest { operation, generation: semio_framework_job::Generation(expected_generation), base: ArtifactEphemeralBaseRead(ArtifactEphemeralBaseOwner::Transient(Arc::clone(self.current.as_ref().expect("live transient root remains")))), mutation };
        let preparation = match factory.begin(request) {
            Ok(preparation) => preparation,
            Err(request) => return Err(reject("transient preparation factory rejected its exact owner bundle".into(), request.mutation)),
        };
        Ok(ArtifactEphemeralOneItemPublication {
            operation,
            expected_generation,
            footprint,
            preparation: std::mem::ManuallyDrop::new(Some(preparation)),
            displaced_root_retirement: std::mem::ManuallyDrop::new(None),
            root_retirement_factory: std::mem::ManuallyDrop::new(Some(root_retirement_factory)),
            returned_read_registry: std::mem::ManuallyDrop::new(None),
            returned_read_retirement: std::mem::ManuallyDrop::new(None),
            owned_retirement_factory: std::mem::ManuallyDrop::new(None),
            receipt: std::mem::ManuallyDrop::new(None),
            retained_checkpoint: ArtifactStoreOneItemCheckpoint::default(),
            factory_retirement: [None, None],
            attempts: 0,
            published: false,
            cancel_requested: false,
            close_started: false,
            fault: std::mem::ManuallyDrop::new(None),
            phase: ArtifactStoreOneItemPublicationPhase::Preparing,
        })
    }

    /// 📬️ Begins one transient publication from a tracked base read. Displaced roots use
    /// unique-or-release retirement so a still-live tool read cannot create a close cycle.
    pub fn begin_publish_one_leased(
        &self,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        mutation: Mutation,
        factory: &dyn ArtifactEphemeralOneItemPreparationFactory<P, Mutation>,
        owned_retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    ) -> Result<ArtifactEphemeralOneItemPublication<P, Mutation>, ArtifactEphemeralOneItemAdmissionRejected<Mutation>>
    where
        P: Send + Sync + 'static,
    {
        let reject = |reason: String, mutation: Mutation| ArtifactEphemeralOneItemAdmissionRejected { reason, mutation };
        if self.close_started || self.generation != expected_generation {
            return Err(reject("transient publication generation is stale".into(), mutation));
        }
        let footprint = match factory.preflight(&mutation) {
            Ok(footprint) if footprint.is_admissible() => footprint,
            Ok(_) => return Err(reject("transient publication footprint exceeds its fixed item or byte capacity".into(), mutation)),
            Err(reason) => return Err(reject(reason, mutation)),
        };
        let base = match self.current_read() {
            Ok(read) => ArtifactEphemeralBaseRead(ArtifactEphemeralBaseOwner::TransientRead(read)),
            Err(reason) => return Err(reject(reason, mutation)),
        };
        let request = ArtifactEphemeralOneItemPreparationRequest { operation, generation: semio_framework_job::Generation(expected_generation), base, mutation };
        let preparation = match factory.begin(request) {
            Ok(preparation) => preparation,
            Err(request) => return Err(reject("transient preparation factory rejected its exact owner bundle".into(), request.mutation)),
        };
        Ok(ArtifactEphemeralOneItemPublication {
            operation,
            expected_generation,
            footprint,
            preparation: std::mem::ManuallyDrop::new(Some(preparation)),
            displaced_root_retirement: std::mem::ManuallyDrop::new(None),
            root_retirement_factory: std::mem::ManuallyDrop::new(None),
            returned_read_registry: std::mem::ManuallyDrop::new(Some(SnapshotReadRegistryAliasRetirement::new(self.reads.as_ref().expect("live transient read registry remains").clone()))),
            returned_read_retirement: std::mem::ManuallyDrop::new(None),
            owned_retirement_factory: std::mem::ManuallyDrop::new(Some(owned_retirement_factory)),
            receipt: std::mem::ManuallyDrop::new(None),
            retained_checkpoint: ArtifactStoreOneItemCheckpoint::default(),
            factory_retirement: [None, None],
            attempts: 0,
            published: false,
            cancel_requested: false,
            close_started: false,
            fault: std::mem::ManuallyDrop::new(None),
            phase: ArtifactStoreOneItemPublicationPhase::Preparing,
        })
    }

    pub fn advance_publish_one(&mut self, publication: &mut ArtifactEphemeralOneItemPublication<P, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, String>
    where
        P: Send + Sync + 'static,
    {
        if publication.phase == ArtifactStoreOneItemPublicationPhase::Complete {
            return Ok(ArtifactStoreOneItemAdvance::Complete);
        }
        if publication.phase == ArtifactStoreOneItemPublicationPhase::Closing {
            return Ok(ArtifactStoreOneItemAdvance::Blocked);
        }
        if publication.phase == ArtifactStoreOneItemPublicationPhase::AwaitingAck {
            return Ok(ArtifactStoreOneItemAdvance::AwaitingAck(publication.receipt.expect("transient publication awaiting ACK retains its receipt")));
        }
        if publication.cancel_requested {
            publication.begin_close();
            return Ok(ArtifactStoreOneItemAdvance::Blocked);
        }
        if self.close_started || self.generation != publication.expected_generation {
            *publication.fault = Some(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"transient publication became stale before commit"));
            publication.phase = ArtifactStoreOneItemPublicationPhase::Fault;
            publication.begin_close();
            return Err("transient publication became stale before commit".into());
        }
        if !grant.permits_one() {
            return Ok(ArtifactStoreOneItemAdvance::Blocked);
        }
        match publication.phase {
            ArtifactStoreOneItemPublicationPhase::Preparing => match publication.advance_preparation(grant) {
                Err(refused)=>Ok(refused),
                Ok(ArtifactStoreOneItemPreparationStep::Progress(checkpoint, ownership)) => Ok(ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint, ownership)),
                Ok(ArtifactStoreOneItemPreparationStep::Prepared(checkpoint, ownership)) => {
                    publication.phase = ArtifactStoreOneItemPublicationPhase::PreparingCursor;
                    Ok(ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint, ownership))
                }
                Ok(ArtifactStoreOneItemPreparationStep::Blocked) => Ok(ArtifactStoreOneItemAdvance::Blocked),
            },
            ArtifactStoreOneItemPublicationPhase::PreparingCursor => {
                if publication.preparation.as_ref().and_then(|owner| owner.prepared()).is_none() {
                    return Err("transient prepared root is absent".into());
                }
                publication.phase = ArtifactStoreOneItemPublicationPhase::PreflightingCommit;
                Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()))
            }
            ArtifactStoreOneItemPublicationPhase::PreflightingCommit => {
                if self.generation == u64::MAX || publication.footprint.work_items == 0 {
                    return Err("transient publication fixed commit authority is exhausted".into());
                }
                publication.phase = ArtifactStoreOneItemPublicationPhase::Publishing;
                Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()))
            }
            ArtifactStoreOneItemPublicationPhase::Publishing => {
                let prepared = publication.preparation.as_mut().and_then(|owner| owner.take_prepared()).ok_or_else(|| "transient publication lost its prepared root".to_string())?;
                let generation_before = self.generation;
                let previous = self.current.replace(prepared.next_root).expect("live transient publication root remains");
                *publication.displaced_root_retirement = Some(if let Some(factory) = publication.owned_retirement_factory.as_ref() {
                    Box::new(ReturnedSnapshotReadRetirement::new(previous, factory.clone()))
                } else {
                    ArtifactStoreDisplacedSnapshotRetirement::boxed(previous, Arc::clone(publication.root_retirement_factory.as_ref().ok_or_else(|| "transient publication lost its local-root retirement factory".to_string())?))
                });
                self.generation += 1;
                let receipt = LaneItemReceipt { generation_before, generation_after: self.generation };
                publication.expected_generation = self.generation;
                *publication.receipt = Some(receipt);
                publication.published = true;
                publication.phase = ArtifactStoreOneItemPublicationPhase::AwaitingAck;
                Ok(ArtifactStoreOneItemAdvance::Published(receipt))
            }
            ArtifactStoreOneItemPublicationPhase::Fault => Ok(ArtifactStoreOneItemAdvance::PreparationRefused),
            ArtifactStoreOneItemPublicationPhase::AwaitingAck | ArtifactStoreOneItemPublicationPhase::Closing | ArtifactStoreOneItemPublicationPhase::Complete => unreachable!("handled transient publication control phase"),
        }
    }

    pub fn cancel_publish_one(&mut self, publication: &mut ArtifactEphemeralOneItemPublication<P, Mutation>) -> bool {
        if matches!(publication.phase, ArtifactStoreOneItemPublicationPhase::Complete | ArtifactStoreOneItemPublicationPhase::Closing) {
            return false;
        }
        publication.cancel_requested = true;
        publication.begin_close();
        true
    }

    /// 🎯️ Applies one retained local-only item against an exact live generation.
    pub fn apply_one(&mut self, expected_generation: u64, mutation: Mutation) -> Result<LaneItemReceipt, (String, Mutation)> {
        if self.close_started || self.generation != expected_generation {
            return Err(("transient publication generation is stale".into(), mutation));
        }
        let before = self.generation;
        let candidate = match apply_operation::<P, Mutation>(self.current.as_ref().expect("live transient root remains").as_ref(), &mutation, 0).0 {
            Ok(candidate) => candidate,
            Err(error) => return Err((error.to_string(), mutation)),
        };
        self.current = Some(Arc::new(candidate));
        self.generation = self.generation.wrapping_add(1);
        Ok(LaneItemReceipt { generation_before: before, generation_after: self.generation })
    }

    /// 🔄️ Discards everything — what a host does when a view closes.
    pub async fn reset(&mut self, current: P) {
        assert!(!self.close_started, "closed transient store cannot be reset");
        self.current = Some(Arc::new(current));
        self.generation = self.generation.wrapping_add(1);
    }

    /// 🔢️ Bumps on every change, so a renderer can skip untouched frames.
    pub async fn generation(&self) -> u64 {
        self.generation
    }

    /// 🪪️ Reads the event-maintained transient generation without suspension.
    pub fn generation_now(&self) -> u64 {
        self.generation
    }

    /// 📏️ Borrows the next returned read's actual construction or physical retirement demand.
    pub fn maintenance_returned_reads_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where P: Send + Sync + 'static {
        if let Some(owner) = self.active_returned_read.as_ref() { return artifact_retirement_box_demands(owner, body); }
        if self.reads.is_none() { return Ok(Default::default()); }
        if self.reads.as_ref().expect("live transient read registry remains").has_returned() { return self.reads.as_ref().expect("live transient read registry remains").returned_admission_demands::<P>(|_| semio_framework_value::RetirementDemand { capacity_bytes: ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(), depth: 1, ..Default::default() }).map_err(SnapshotReadLeaseRefusal::into_value_error); }
        Ok(Default::default())
    }

    /// 🏁️ Moves every original root and tracked read into close custody without a replacement allocation.
    pub fn begin_close_retirement(&mut self, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>) -> TransientStoreRetirement<P>
    where P: Send + Sync + 'static {
        self.close_started = true;
        TransientStoreRetirement { root: std::mem::ManuallyDrop::new(self.current.take()), reads: std::mem::ManuallyDrop::new(self.reads.take().map(SnapshotReadRegistryAliasRetirement::new)), active_returned: std::mem::ManuallyDrop::new(self.active_returned_read.take()), active_root: std::mem::ManuallyDrop::new(None), factory: Some(factory), factory_close: None }
    }

    /// 🫙️ The live shell owns no root, tracked registry or returned read after consuming close.
    pub fn close_terminal_is_empty(&self) -> bool { self.current.is_none() && self.reads.is_none() && self.active_returned_read.is_none() }

    /// ♻️ Detaches this store's exact root/read registry and installs a fresh terminal root.
    pub fn begin_retirement(&mut self, terminal: P, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>) -> TransientStoreRetirement<P>
    where
        P: Send + Sync + 'static,
    {
        let next_generation = self.generation.wrapping_add(1);
        let displaced = std::mem::replace(self, Self::new(terminal));
        self.generation = next_generation;
        TransientStoreRetirement::new(displaced, factory)
    }

    /// 🧾️ Closed stores retain no substitute root, read registry or returned cursor.
    pub fn detached_terminal_is_empty(&self) -> bool { self.close_started && self.current.is_none() && self.reads.is_none() && self.active_returned_read.is_none() }

}

/// ♻️ Detached transient root plus every returned/live tracked read.
pub struct TransientStoreRetirement<P> {
    root: std::mem::ManuallyDrop<Option<Arc<P>>>,
    reads: std::mem::ManuallyDrop<Option<SnapshotReadRegistryAliasRetirement>>,
    active_returned: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    active_root: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    factory: Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>,
    factory_close: Option<semio_framework_value::FactoryAuthority>,
}
impl<P> TransientStoreRetirement<P> {
    fn new<Mutation>(store: TransientStore<P, Mutation>, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>) -> Self {
        let store = std::mem::ManuallyDrop::new(store);
        Self { root: std::mem::ManuallyDrop::new(unsafe { std::ptr::read(&store.current) }), reads: std::mem::ManuallyDrop::new(unsafe { std::ptr::read(&store.reads) }.map(SnapshotReadRegistryAliasRetirement::new)), active_returned: std::mem::ManuallyDrop::new(unsafe { std::ptr::read(&store.active_returned_read) }), active_root: std::mem::ManuallyDrop::new(None), factory: Some(factory), factory_close: None }
    }
    pub fn terminal_is_empty(&self) -> bool { self.root.is_none() && self.reads.is_none() && self.active_returned.is_none() && self.active_root.is_none() && self.factory.is_none() && self.factory_close.is_none() }
    /// 🔒️ The retirement can only continue once the live tracked read that holds the registry is returned.
    pub fn awaits_live_read(&self) -> bool { self.root.is_none() && self.active_returned.is_none() && self.active_root.is_none() && self.reads.as_ref().is_some_and(|reads| !reads.has_returned() && !reads.terminal_is_empty()) }
}
impl<P: Send + Sync + 'static> TransientStoreRetirement<P> {
    pub fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "transient retirement depth overflow"))?; Ok(demand) };
        if let Some(active) = self.active_returned.as_ref() { return nested(artifact_retirement_box_demands(active, body)?); }
        if let Some(reads) = self.reads.as_ref().filter(|reads| reads.has_returned()) { return nested(reads.returned_admission_demands::<P>(|_| RetirementDemand { capacity_bytes: ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(), depth: 1, ..Default::default() }).map_err(SnapshotReadLeaseRefusal::into_value_error)?); }
        if let Some(active) = self.active_root.as_ref() { return nested(artifact_retirement_box_demands(active, body)?); }
        if self.root.is_some() { return Ok(RetirementDemand { capacity_bytes: ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(), depth: 2, ..Default::default() }); }
        if self.reads.is_some() { return snapshot_registry_alias_demands(&self.reads); }
        if self.factory.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        self.factory_close.as_ref().map_or(Ok(Default::default()), |factory| nested(factory.demands(body)?))
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress};
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "transient retirement exceeds admitted depth")); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(reads) = self.reads.as_ref() { if self.active_returned.is_some() || reads.has_returned() { return advance_returned_snapshot_read(reads, &mut self.active_returned, self.factory.as_ref().expect("retained transient factory"), child).map(|step| RetainedCloneStep::Progress(step.progress())); } }
        if self.active_root.is_some() { return artifact_retirement_box_close_step(&mut self.active_root, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(root) = self.root.take() {
            return match ReturnedSnapshotReadRetirement::admit(root, self.factory.as_ref().expect("retained transient factory").clone(), child) { Ok((owner, receipt)) => { *self.active_root = Some(owner); if !receipt.fits(child) || receipt.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "transient constructor changed admitted receipt")); } Ok(RetainedCloneStep::Progress(receipt)) }, Err((error, root, alias)) => { *self.root = Some(root); drop(alias); Err(error) } };
        }
        if let Some(reads) = self.reads.as_ref() { if !reads.terminal_is_empty() { return Ok(RetainedCloneStep::Progress(Default::default())); } return snapshot_registry_alias_close_step(&mut self.reads, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(factory) = self.factory.take() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factory_close = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); }
        let factory = self.factory_close.as_mut().expect("retained transient factory ticket"); let step = factory.step(child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "transient factory")?; if factory.terminal_is_empty() { self.factory_close = None; } Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }
}
impl<P> Drop for TransientStoreRetirement<P> {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "transient store retirement reached Drop before terminal-empty ownership"); if self.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.root); std::mem::ManuallyDrop::drop(&mut self.reads); std::mem::ManuallyDrop::drop(&mut self.active_returned); std::mem::ManuallyDrop::drop(&mut self.active_root); } } }
}

//#region 🔖️InteractionStore
/// 🕹️ Ephemeral LOCAL-ONLY lane for the framework interaction mechanism's hover half — the
/// `PresenceStore`/`TransientStore` sibling for `InteractionState.hover`.
///
/// 🎯️ Only hover lives here. Selection + `active_mode`/`active_granularity` are
/// **persisted-local** (`InteractionState`'s other three fields) and already flow through the
/// ordinary `ConfigStore` (`ArtifactStore<Config, ConfigMutation>`) via `ArtifactCommand::
/// ApplyInLane { lane: HistoryLane::Interaction, .. }` — no new store type needed for that half,
/// it is the ENTIRE point of the `HistoryLane` mechanism above. Hover must never take that path
/// (never persisted, never in `Edit`/`Change` history), so it gets this dedicated ephemeral
/// sibling instead, shaped exactly like `TransientStore` (single local value, no peer roster —
/// peers' hover arrives over the wire as `PresencePeer.interaction`, not through this store).
///
/// `S`/`Mutation` are supplied by the caller (mirrors `PresenceStore<A::Presence,
/// A::PresenceMutation>` in `VcsArtifactApp`): this crate has no dependency edge to the
/// `semio-framework` crate that owns `InteractionState`/`DomainHover` (that crate depends on
/// THIS one, for its `spr`/`dsl`/`pack` types — the reverse edge would cycle), so `S` stays
/// generic here and gets instantiated with the app's concrete hover-shaped type one layer up,
/// where that dependency edge actually exists.
#[derive(Clone, Debug)]
pub struct InteractionStore<S, Mutation> {
    hover: S,
    generation: u64,
    _mutation: PhantomData<fn() -> Mutation>,
}

// 🚫️async: E1 impl of external `Default` — signature fixed outside this repo.
impl<S: Clone + Default, Mutation: self::Mutation<S>> Default for InteractionStore<S, Mutation> {
    fn default() -> Self {
        Self::new(S::default())
    }
}

impl<S: Clone, Mutation: self::Mutation<S>> InteractionStore<S, Mutation> {
    /// 🏗️ Starts with this actor's own initial hover state (typically empty).
    /// 🚫️async: E1 pure constructor, consumed by `Default::default()` — see R9.
    pub fn new(hover: S) -> Self {
        Self { hover, generation: 0, _mutation: PhantomData }
    }

    /// 👁️ This actor's own hover — what the presence heartbeat mirrors onto `PresenceDomain.hovered`.
    pub async fn hover(&self) -> &S {
        &self.hover
    }

    /// ✍️ Applies hover updates atomically. The caller's `Mutation` impl is expected to route
    /// through the framework's `next_hover` pure fn before landing here.
    pub async fn apply(&mut self, mutations: &[Mutation]) -> protocol::MutationApplyResult<()> {
        let mut candidate = self.hover.clone();
        for mutation in mutations {
            candidate = apply_operation::<S, Mutation>(&candidate, mutation, 0).0?;
        }
        if !mutations.is_empty() {
            self.hover = candidate;
            self.generation = self.generation.wrapping_add(1);
        }
        Ok(())
    }

    /// 🔄️ Discards hover — a host clears this when a view/window closes (nothing left to hover).
    pub async fn reset(&mut self, hover: S) {
        self.hover = hover;
        self.generation = self.generation.wrapping_add(1);
    }

    /// 🔢️ Bumps on every local hover change; drives the same broadcast-coalescing signal as
    /// `PresenceStore::generation`.
    pub async fn generation(&self) -> u64 {
        self.generation
    }
}
//#endregion 🔖️InteractionStore
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority


#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:semio_framework_trace_memory_testing::RequestedAllocator=semio_framework_trace_memory_testing::RequestedAllocator;
