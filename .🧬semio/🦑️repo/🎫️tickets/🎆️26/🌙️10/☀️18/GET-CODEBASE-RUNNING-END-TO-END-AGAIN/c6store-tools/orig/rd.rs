//! 🔗️ Opaque snapshot reads and the Store's owner catalogs: derived snapshots, replays and history previews, the
//! snapshot read lease registry, the per-owner retirement factories and displaced-owner reservations, and the
//! document and member store owner catalogs with their unscheduled completion. Engine-held views (close view and
//! disposers over a live `ArtifactStore`) stay with the engine; everything here is vocabulary the engine and every
//! facet above it read in one direction.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).


#[path = "📥️lifted/🏪️store/🦀️.rs"]
pub mod lifted_store;
#[path = "../🕰️history/📐️planning/♻️retirement/🦀️.rs"]
pub mod history_plan_retirement;
#[path = "../🕰️history/📐️planning/🎟️admission/🦀️.rs"]
pub mod history_read_admission;
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirementFactory;

use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreChangeRetirementFactory;
use semio_framework_os_vcs::io::binary::genesis::AdmittedArtifactGenesis;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_os_store_replay::replay_preparation::{ArtifactReplayPreparation, ArtifactReplayPreparationFactory, ArtifactReplayPreparationProgress};
use semio_framework_os_spr::command::ArtifactMessageSource;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::MutationId;
use semio_framework_os_spr::{DiffRegions, Edit, Mutation, MutationDiff, MutationMeta, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::Arc;
use semio_framework_value_snapshot_read::SnapshotReadRegistryHandle;
use semio_framework_value_snapshot_read::{SnapshotReadRegistryAliasRetirement,snapshot_registry_alias_demands,snapshot_registry_alias_close_step};
use semio_framework_os_spr::conflict::ProtocolConflictRetirement as ArtifactStoreConflictRetirement;
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
use semio_framework_os_vcs::{Author, VcsError};
use semio_framework_os_vcs::{Alternative, ArtifactHistoryLedger, ArtifactVcs, Change, Checkpoint, Identified, Patchable};
use semio_framework_os_store_envelope::owner::OwnerRef;
use semio_framework_os_pack::pack_rt;
use semio_framework_os_store_message_ledger::ArtifactEditMessageLedgerRetirement;
use semio_framework_os_store_ledger::CursorRevisionAccumulator;
use semio_framework_pack_json::ArtifactCanonicalJsonTree;

use semio_framework_os_store_one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
use semio_framework_os_store_envelope::{ArtifactBackboneRef, ArtifactEnvelope, ArtifactEnvelopeOwners, MigrationProvenance};
use semio_framework_os_store_cursor::ArtifactCursor;
use semio_framework_os_store_replay::{EditReplay, EditReplayResult, ReplayProgress};
use semio_framework_os_store_ledger::lifted_replay::EffectiveSupersessions;
use semio_framework_os_store_command::{HistoryLane, OpenToolTransaction};
use semio_framework_os_store_projection::retire_shared_projection;

/// 🌱️ Immutable projection derived by one Store from its own event history or a provisional operation.
pub struct ArtifactDerivedSnapshot<P> {
    pub owner: Arc<P>,
    pub registry: SnapshotReadRegistryHandle,
    pub generation: u64,
    pub revision: [u8; 32],
}

impl<P> ArtifactDerivedSnapshot<P> {
    /// 👁️ Borrows the complete typed projection without transferring its derivation authority.
    pub fn snapshot(&self) -> &P {
        self.owner.as_ref()
    }

    /// 🪞️ Borrows the immutable alias used by renderer seams.
    pub fn snapshot_owner(&self) -> &Arc<P> {
        &self.owner
    }

    /// ♻️ Transfers the projection alias to the caller's registered bounded retirement.
    pub fn into_snapshot_owner(self) -> Arc<P> {
        self.owner
    }
}

/// ⏪️ Store-owned history replay whose input edits and projection origin cannot be supplied by the caller.
pub struct ArtifactDerivedReplay<P, Mu: Mutation<P>> {
    pub replay: Option<EditReplay<P, Mu>>,
    pub plan: Option<HistoryReadPlan>,
    pub plan_retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<HistoryReadPlan>>,
    pub registry: SnapshotReadRegistryHandle,
    pub generation: u64,
    pub revision: [u8; 32],
}

impl<P, Mu: Mutation<P> + OpBinary> ArtifactDerivedReplay<P, Mu> {
    /// 📈️ Reports genuine cumulative work inside the currently sealed operation producer.
    pub fn operation_preparation_progress(&self) -> ArtifactReplayPreparationProgress {
        self.replay.as_ref().map_or_else(ArtifactReplayPreparationProgress::default, |replay|replay.operation_preparation_progress())
    }

    pub fn retain_completed_plan(&mut self) -> &mut HistoryReadPlan {
        let original=self.plan.take().expect("completed replay planner");
        retain_completed_history_plan(&mut self.plan_retirement,original)
    }

    /// 📏️ Borrows the exact cleanup requirements of the completed planner's original residue.
    pub fn planning_retirement_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError> {retirement_owner_demands(&self.plan_retirement,body)}

    /// ♻️ Closes one original planner residue frontier under the caller's independent grants.
    pub fn retire_completed_plan(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {retirement_owner_close(&mut self.plan_retirement,grant)}
}

/// 🕰️ A generation-fenced cursor resolving a preview's inputs and folding its prefix under the caller's deadline.
pub struct ArtifactDerivedHistoryPreview<P, Mu: Mutation<P>> {
    pub plan: Option<HistoryReadPlan>,
    pub plan_retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<HistoryReadPlan>>,
    pub drafts: protocol::HistoryInputDrafts,
    pub target: Option<(usize, usize)>,
    pub cursor: usize,
    pub operation: usize,
    pub state: Option<Arc<P>>,
    pub progress: ReplayProgress,
    pub registry: SnapshotReadRegistryHandle,
    pub generation: u64,
    pub revision: [u8; 32],
    pub marker: PhantomData<fn() -> Mu>,
    pub finished: bool,
}

impl<P, Mu: Mutation<P>> ArtifactDerivedHistoryPreview<P, Mu> {
    /// 📶️ Work completed while planning or folding the selected prefix.
    pub fn progress(&self) -> ReplayProgress { self.plan.as_ref().map_or(self.progress, |plan| plan.progress) }

    /// 📤️ Transfers the completed projection while retaining every original input and planner residue.
    pub fn take_finished_snapshot_owner(&mut self)->Result<Arc<P>,VcsError> {
        if !self.finished{return Err(VcsError::HistoryReplaying);}
        self.state.take().ok_or(VcsError::HistoryReplaying)
    }

    pub fn retain_completed_plan(&mut self)->usize {
        let original=self.plan.take().expect("completed preview planner");
        let plan=retain_completed_history_plan(&mut self.plan_retirement,original);
        self.target=Some(plan.positions[plan.target.as_ref().expect("preview target")]);
        self.progress=plan.progress;self.progress.total=self.progress.done;
        self.drafts=std::mem::take(&mut plan.drafts);
        plan.first_input
    }

    /// 🧮️ Borrows each independent cleanup demand while preserving the original completed planner.
    pub fn planning_retirement_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError> {retirement_owner_demands(&self.plan_retirement,body)}

    /// 🧹️ Closes one completed planner frontier without dropping any ungranted backing.
    pub fn retire_completed_plan(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {retirement_owner_close(&mut self.plan_retirement,grant)}
}

impl<P, Mu: Mutation<P>> Drop for ArtifactDerivedHistoryPreview<P, Mu> {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            retire_shared_projection::<P, Mu>(state);
        }
    }
}

pub struct HistoryReadPlan {
    pub target: Option<MutationId>,
    pub drafts: protocol::HistoryInputDrafts,
    pub positions: protocol::HistoryFoldIndex<MutationId, (usize, usize)>,
    pub first_input: usize,
    pub last_draft: Option<usize>,
    pub order: semio_framework_os_vcs::HistoryPageStack<String>,
    pub counts: semio_framework_os_vcs::HistoryPageStack<u32>,
    pub cursor: usize,
    pub located_target: bool,
    pub located_draft: Option<MutationId>,
    pub located_inputs: bool,
    pub operations: u32,
    pub supersessions: EffectiveSupersessions,
    pub copied: Option<MutationId>,
    pub copying_drafts: bool,
    pub report: bool,
    pub progress: ReplayProgress,
    pub finished: bool,
}

impl HistoryReadPlan {
    pub fn new(target: Option<MutationId>, drafts: protocol::HistoryInputDrafts, edits: usize, report: bool) -> Self {
        Self {
            target, drafts, positions: protocol::HistoryFoldIndex::new(), first_input: edits, last_draft: None, order: semio_framework_os_vcs::HistoryPageStack::empty(), counts: semio_framework_os_vcs::HistoryPageStack::empty(), cursor: 0, located_target: false, located_draft: None, located_inputs: false, operations: 0,
            supersessions: EffectiveSupersessions::new(), copied: None, copying_drafts: false, report,
            progress: ReplayProgress { done: 0, total: 0 }, finished: false,
        }
    }
}

pub fn retain_completed_history_plan(slot:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<HistoryReadPlan>>,original:HistoryReadPlan)->&mut HistoryReadPlan {
    assert!(slot.is_none(),"completed history planner residue remains attached");
    *slot=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("history planner declares controlled retirement")));
    slot.as_mut().unwrap().original_mut().unwrap()
}

pub struct ArtifactHistoryReadRetirement<P, Mu: Mutation<P>> {
    pub derived: std::mem::ManuallyDrop<Option<ArtifactDerivedSnapshot<P>>>,
    pub derived_alias: std::mem::ManuallyDrop<Option<Arc<P>>>,
    pub preview: std::mem::ManuallyDrop<Option<ArtifactDerivedHistoryPreview<P, Mu>>>,
    pub replay: std::mem::ManuallyDrop<Option<ArtifactDerivedReplay<P, Mu>>>,
    pub finished: std::mem::ManuallyDrop<Option<EditReplayResult<P, Mu>>>,
    pub loaded_replay: std::mem::ManuallyDrop<Option<EditReplay<P, Mu>>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub plan_retirement: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<HistoryReadPlan>>>,
    pub draft_retirement: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<protocol::HistoryInputDrafts>>>,
    pub registry_retirement: std::mem::ManuallyDrop<Option<SnapshotReadRegistryAliasRetirement>>,
    pub snapshots: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,
    pub mutations: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mu>>>>,
    pub factory_retirement:[Option<semio_framework_value::FactoryAuthority>;2],
}

impl<P, Mu> ArtifactHistoryReadRetirement<P, Mu>
where P: Send + Sync + 'static, Mu: Mutation<P> + Send + 'static {
    fn derived_snapshot_demands(&self)->Option<Result<semio_framework_value::RetirementDemand,ValueError>> {
        if self.registry_retirement.is_some(){return Some(snapshot_registry_alias_demands(&self.registry_retirement))}
        if self.derived_alias.is_some(){return Some(Ok(semio_framework_value::RetirementDemand{capacity_bytes:ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(),depth:2,..Default::default()}))}
        self.derived.is_some().then_some(Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()}))
    }
    fn close_derived_snapshot(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError>{
        use semio_framework_value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        let Some(demand)=self.derived_snapshot_demands()else{return Ok(None)};let demand=demand?;
        if grant.maximum_items==0||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
        if self.registry_retirement.is_some(){return snapshot_registry_alias_close_step(&mut self.registry_retirement,grant).map(Some)}
        if self.derived_alias.is_some(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let Some((owner,receipt))=ReturnedSnapshotReadRetirement::admit_original(&mut self.derived_alias,self.snapshots.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"derived original lost its snapshot issuer"))?,child)?else{unreachable!("preflight retains derived original alias")};*self.active=Some(owner);return Ok(Some(RetainedCloneStep::Progress(receipt)))}
        let ArtifactDerivedSnapshot{owner,registry,..}=self.derived.take().unwrap();*self.derived_alias=Some(owner);*self.registry_retirement=Some(SnapshotReadRegistryAliasRetirement::new(registry));Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})))
    }
    fn plan_demands(&self,body:usize)->Option<Result<semio_framework_value::RetirementDemand,ValueError>> {
        if self.plan_retirement.is_some() {return Some(retirement_owner_demands(&self.plan_retirement,body));}
        if self.preview.as_ref().is_some_and(|owner|owner.plan.is_some()||owner.plan_retirement.is_some())||self.replay.as_ref().is_some_and(|owner|owner.plan.is_some()||owner.plan_retirement.is_some()) {return Some(Ok(semio_framework_value::RetirementDemand {depth:1,..Default::default()}));}
        None
    }
    fn close_plan(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress};
        let Some(demand)=self.plan_demands(grant.maximum_copy_bytes) else{return Ok(None)};
        let demand=demand?;if grant.maximum_items==0 {return Ok(Some(RetainedCloneStep::Progress(Default::default())));}if grant.maximum_depth<demand.depth {return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history plan retirement requires its original owner depth"));}
        if self.plan_retirement.is_some() {return retirement_owner_close(&mut self.plan_retirement,grant).map(Some);}
        if let Some(owner)=self.preview.as_mut().and_then(|owner|owner.plan_retirement.take()).or_else(||self.replay.as_mut().and_then(|owner|owner.plan_retirement.take())) {*self.plan_retirement=Some(owner);}else {
            let original=self.preview.as_mut().and_then(|owner|owner.plan.take()).or_else(||self.replay.as_mut().and_then(|owner|owner.plan.take())).expect("observed original history planner");
            *self.plan_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("history planner schema declares controlled retirement")));
        }
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()})))
    }
    fn preview_demands(&self,body:usize)->Option<Result<semio_framework_value::RetirementDemand,ValueError>> {
        use semio_framework_value::RetirementDemand;
        if self.draft_retirement.is_some() {return Some(retirement_owner_demands(&self.draft_retirement,body));}
        if self.registry_retirement.is_some() {return Some(snapshot_registry_alias_demands(&self.registry_retirement));}
        let preview=self.preview.as_ref()?;
        if !preview.drafts.terminal_is_empty() {return Some(Ok(RetirementDemand {depth:1,..Default::default()}));}
        if preview.state.is_some() {return Some(Ok(RetirementDemand {capacity_bytes:ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(),depth:2,..Default::default()}));}
        Some(Ok(RetirementDemand {depth:1,..Default::default()}))
    }
    fn close_preview(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress};
        let Some(demand)=self.preview_demands(grant.maximum_copy_bytes)else{return Ok(None)};let demand=demand?;
        if grant.maximum_items==0||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_copy_bytes<demand.copy_bytes {return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        if grant.maximum_depth<demand.depth {return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history preview retirement requires its exact owner depth"));}
        if self.draft_retirement.is_some() {return retirement_owner_close(&mut self.draft_retirement,grant).map(Some);}
        if self.registry_retirement.is_some() {return snapshot_registry_alias_close_step(&mut self.registry_retirement,grant).map(Some);}
        let preview=self.preview.as_mut().expect("observed original preview");
        if !preview.drafts.terminal_is_empty() {let drafts=std::mem::take(&mut preview.drafts);*self.draft_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(drafts).unwrap_or_else(|_|unreachable!("draft schema declares controlled retirement")));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()})));}
        if let Some(snapshot)=preview.state.take() {
            let factory=self.snapshots.as_ref().expect("original preview retains its snapshot issuer").clone();
            let child_grant=RetainedCloneGrant {maximum_depth:grant.maximum_depth-1,..grant};
            return match ReturnedSnapshotReadRetirement::admit(snapshot,factory,child_grant) {Ok((owner,progress))=>{*self.active=Some(owner);Ok(Some(RetainedCloneStep::Progress(progress)))},Err((error,snapshot,factory))=>{preview.state=Some(snapshot);drop(factory);Err(error)}};
        }
        *self.registry_retirement=Some(SnapshotReadRegistryAliasRetirement::new(preview.registry.clone()));drop(self.preview.take());
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()})))
    }
    fn factory_demands(&self,body:usize)->Option<Result<semio_framework_value::RetirementDemand,ValueError>> {
        if self.derived.is_some()||self.derived_alias.is_some()||self.preview.is_some()||self.replay.is_some()||self.loaded_replay.is_some()||self.finished.is_some() {return None;}
        if let Some(owner)=self.factory_retirement.iter().flatten().next() {return Some(owner.demands(body).and_then(|mut demand| {demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history factory retirement depth overflow"))?;Ok(demand)}));}
        (self.snapshots.is_some()||self.mutations.is_some()).then_some(Ok(semio_framework_value::RetirementDemand {depth:1,..Default::default()}))
    }
    fn close_factories(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress};
        let Some(demand)=self.factory_demands(grant.maximum_copy_bytes)else{return Ok(None)};let demand=demand?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes {return Ok(Some(RetainedCloneStep::Progress(Default::default())));}if grant.maximum_depth<demand.depth {return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history factory retirement requires its original owner depth"));}
        if let Some(slot)=self.factory_retirement.iter_mut().find(|owner|owner.is_some()) {
            let owner=slot.as_mut().unwrap();
            let child_grant=RetainedCloneGrant {maximum_depth:grant.maximum_depth-1,..grant};
            let step=owner.step(child_grant)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child_grant,step,owner.terminal_is_empty(),"history factory authority")?;
            if owner.terminal_is_empty() {drop(slot.take());}
            return Ok(Some(step));
        }
        let factory:Arc<dyn semio_framework_value::FactoryRetirement>=if let Some(factory)=self.snapshots.take(){factory}else{self.mutations.take().expect("original history mutation issuer")};
        self.factory_retirement[0]=Some(semio_framework_value::FactoryAuthority::new(factory));Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()})))
    }
    fn raw_replay_demands(&self,body:usize)->Option<Result<semio_framework_value::RetirementDemand,ValueError>> {
        let demand=if let Some(owner)=self.finished.as_ref(){if owner.terminal_is_empty(){return Some(Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()}));}owner.retirement_demands(body)}else if let Some(owner)=self.loaded_replay.as_ref().or_else(||self.replay.as_ref().and_then(|owner|owner.replay.as_ref())){if owner.terminal_is_empty(){return Some(Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()}));}owner.retirement_demands(body)}else{return self.replay.is_some().then_some(Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()}));};
        Some(demand.and_then(|mut demand|{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history raw replay retirement depth overflow"))?;Ok(demand)}))
    }
    fn close_raw_replay(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress,admit_retained_clone_close};
        let Some(demand)=self.raw_replay_demands(grant.maximum_copy_bytes)else{return Ok(None)};let demand=demand?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        let moved=RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()});
        let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        if let Some(owner)=self.finished.as_mut(){if owner.terminal_is_empty(){drop(self.finished.take());return Ok(Some(moved));}owner.begin_retirement();let step=owner.close_original_step(child)?;return admit_retained_clone_close(child,step,owner.terminal_is_empty(),"history original finished replay").map(Some);}
        if let Some(owner)=self.loaded_replay.as_mut(){if owner.terminal_is_empty(){drop(self.loaded_replay.take());return Ok(Some(moved));}owner.begin_retirement();let step=owner.close_original_step(child)?;return admit_retained_clone_close(child,step,owner.terminal_is_empty(),"history original loaded replay").map(Some);}
        let derived=self.replay.as_mut().expect("observed original derived replay");
        if let Some(owner)=derived.replay.as_mut(){if owner.terminal_is_empty(){drop(derived.replay.take());return Ok(Some(moved));}owner.begin_retirement();let step=owner.close_original_step(child)?;return admit_retained_clone_close(child,step,owner.terminal_is_empty(),"history original derived replay").map(Some);}
        *self.registry_retirement=Some(SnapshotReadRegistryAliasRetirement::new(derived.registry.clone()));drop(self.replay.take());Ok(Some(moved))
    }
}

impl<P, Mu> ErasedSnapshotRetirement for ArtifactHistoryReadRetirement<P, Mu>
where P: Send + Sync + 'static, Mu: Mutation<P> + Send + 'static {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::RetainedCloneStep;
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.active.is_some() {return artifact_retirement_box_close_step(&mut self.active,grant);}
        let moved = |step: RetainedCloneStep| match step {
            RetainedCloneStep::Complete(progress) => RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }),
            RetainedCloneStep::Progress(progress) => RetainedCloneStep::Progress(progress),
        };
        if let Some(step)=self.close_derived_snapshot(grant)?{return Ok(moved(step))}
        if let Some(step) = self.close_plan(grant)? { return Ok(moved(step)); }
        if let Some(step) = self.close_preview(grant)? { return Ok(moved(step)); }
        if let Some(step) = self.close_raw_replay(grant)? { return Ok(moved(step)); }
        if let Some(step) = self.close_factories(grant)? { return Ok(moved(step)); }
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "history read residual owners require exact constructor admission"))
    }
    fn terminal_is_empty(&self) -> bool { self.derived.is_none() && self.derived_alias.is_none() && self.preview.is_none() && self.replay.is_none() && self.loaded_replay.is_none() && self.finished.is_none() && self.active.is_none() && self.plan_retirement.is_none() && self.draft_retirement.is_none() && self.registry_retirement.is_none() && self.snapshots.is_none() && self.mutations.is_none() && self.factory_retirement.iter().all(Option::is_none) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { if let Some(active)=self.active.as_ref(){return artifact_retirement_box_demands(active,0).map(|demand|demand.copy_bytes);} self.derived_snapshot_demands().or_else(||self.plan_demands(0)).or_else(||self.preview_demands(0)).or_else(||self.raw_replay_demands(0)).or_else(||self.factory_demands(0)).map_or_else(|| if self.terminal_is_empty() { Ok(0) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "history read residual copy authority is not admitted")) }, |demand| demand.map(|demand| demand.copy_bytes)) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { if let Some(active)=self.active.as_ref(){return artifact_retirement_box_demands(active,body).map(|demand|demand.capacity_bytes);} self.derived_snapshot_demands().or_else(||self.plan_demands(body)).or_else(||self.preview_demands(body)).or_else(||self.raw_replay_demands(body)).or_else(||self.factory_demands(body)).map_or_else(|| if self.terminal_is_empty() { Ok(0) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "history read residual constructor authority is not admitted")) }, |demand| demand.map(|demand| demand.capacity_bytes)) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { if let Some(active)=self.active.as_ref(){return artifact_retirement_box_demands(active,0).map(|demand|demand.release_bytes);} self.derived_snapshot_demands().or_else(||self.plan_demands(0)).or_else(||self.preview_demands(0)).or_else(||self.raw_replay_demands(0)).or_else(||self.factory_demands(0)).map_or_else(|| if self.terminal_is_empty() { Ok(0) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "history read residual release authority is not admitted")) }, |demand| demand.map(|demand| demand.release_bytes)) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { if let Some(active)=self.active.as_ref(){return artifact_retirement_box_demands(active,0).map(|demand|demand.depth);} self.derived_snapshot_demands().or_else(||self.plan_demands(0)).or_else(||self.preview_demands(0)).or_else(||self.raw_replay_demands(0)).or_else(||self.factory_demands(0)).map_or_else(|| if self.terminal_is_empty() { Ok(0) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "history read residual depth authority is not admitted")) }, |demand| demand.map(|demand| demand.depth)) }
}

impl<P, Mu: Mutation<P>> Drop for ArtifactHistoryReadRetirement<P, Mu> {
    fn drop(&mut self) { assert!(std::thread::panicking() || (self.derived.is_none() && self.derived_alias.is_none() && self.preview.is_none() && self.replay.is_none() && self.loaded_replay.is_none() && self.finished.is_none() && self.active.is_none() && self.plan_retirement.is_none() && self.draft_retirement.is_none() && self.registry_retirement.is_none() && self.snapshots.is_none() && self.mutations.is_none() && self.factory_retirement.iter().all(Option::is_none)), "history read retirement dropped before its exact terminal witness"); }
}

pub use semio_framework_value_snapshot_read::*;





















pub struct ArtifactStoreRevisionAccumulatorRetirement {
    pub owner: semio_framework_value::retirement::controlled::ControlledRetirement<CursorRevisionAccumulator>,
}

impl ArtifactStoreRevisionAccumulatorRetirement {
    pub fn new(accumulator: CursorRevisionAccumulator) -> Self {
        Self { owner: semio_framework_value::retirement::controlled::ControlledRetirement::new(accumulator).unwrap_or_else(|_| unreachable!("Store revision schema declares controlled retirement")) }
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreRevisionAccumulatorRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.owner.step(grant) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_copy_byte_demand() }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { self.owner.next_capacity_byte_demand(copy) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_release_byte_demand() }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.owner.next_depth_demand() }
    fn terminal_is_empty(&self) -> bool { self.owner.terminal_is_empty() }
}

pub struct ArtifactStoreCursorRetirement {
    pub owner: semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactCursor>,
}

impl ArtifactStoreCursorRetirement {
    pub fn new(cursor: ArtifactCursor) -> Self {
        Self { owner: semio_framework_value::retirement::controlled::ControlledRetirement::new(cursor).unwrap_or_else(|_| unreachable!("Store cursor declares controlled retirement")) }
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreCursorRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.owner.step(grant) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_copy_byte_demand() }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { self.owner.next_capacity_byte_demand(copy) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_release_byte_demand() }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.owner.next_depth_demand() }
    fn terminal_is_empty(&self) -> bool { self.owner.terminal_is_empty() }
}

























pub struct ArtifactStoreMutationDagRetirement {
    pub dag: std::mem::ManuallyDrop<Option<semio_framework_os_spr::MutationDag>>,
    pub envelope: std::mem::ManuallyDrop<Option<ArtifactStoreConflictRetirement>>,
    pub identity: std::mem::ManuallyDrop<Option<ArtifactStoreStringRetirement>>,
}

impl ArtifactStoreMutationDagRetirement {
    pub fn new(dag: semio_framework_os_spr::MutationDag) -> Self {
        Self { dag: std::mem::ManuallyDrop::new(Some(dag)), envelope: std::mem::ManuallyDrop::new(None), identity: std::mem::ManuallyDrop::new(None) }
    }
    pub fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.envelope.is_some() { return retirement_owner_demands(&self.envelope, copy); }
        if self.identity.is_some() { return retirement_owner_demands(&self.identity, copy); }
        Ok(semio_framework_value::RetirementDemand { release_bytes: self.dag.as_ref().map_or(0, |dag| dag.next_backing_release_byte_demand()), depth: usize::from(self.dag.is_some()), ..Default::default() })
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreMutationDagRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth < self.demands(grant.maximum_copy_bytes)?.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "mutation DAG exceeds admitted original owner depth")); }
        let grant = RetainedCloneGrant { maximum_items: 1, ..grant };
        if self.envelope.is_some() { return retirement_owner_close(&mut self.envelope, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if self.identity.is_some() { return retirement_owner_close(&mut self.identity, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        let dag = self.dag.as_mut().unwrap();
        if let Some(owner) = dag.take_one_close_owner() {
            match owner {
                semio_framework_os_spr::MutationDagCloseOwner::Envelope(envelope) => *self.envelope = Some(ArtifactStoreConflictRetirement::causal_envelope(envelope)),
                semio_framework_os_spr::MutationDagCloseOwner::Identity(id) => *self.identity = Some(ArtifactStoreStringRetirement::new(id)),
            }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if !dag.terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "mutation DAG lost an exact owner before terminal retirement")); }
        if !dag.backing_is_empty() {
            let (copied_items, released_bytes, _) = dag.close_backing_step(1, grant.maximum_release_bytes);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items, released_bytes, ..empty }));
        }
        drop(self.dag.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }))
    }
    fn terminal_is_empty(&self) -> bool { self.dag.is_none() && self.envelope.is_none() && self.identity.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl Drop for ArtifactStoreMutationDagRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "mutation DAG retirement reached Drop before every exact nested owner was terminal-empty");
        if self.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.dag); std::mem::ManuallyDrop::drop(&mut self.envelope); std::mem::ManuallyDrop::drop(&mut self.identity); } }
    }
}

pub enum ArtifactStoreVcsRetirementPhase {
    Mutations,
    Edits,
    Changes,
    Checkpoints,
    Alternatives,
    InitialSnapshot,
    Complete,
}

/// 📏️ Observes every current child demand and the separate physical cursor-shell release.

/// 🪆️ Quotes only the depth axis of the boxed turn, so a depth admission never re-derives the other three currencies.

/// 📦️ Retires a current nested payload and its original box in separate fully granted turns.

/// 🎫️ Prices a current original typed owner before its controlled child frame is born.

/// 📋️ Admits the complete original derived snapshot and registry under unchanged caller authority.
pub fn admit_original_snapshot_alias<P,Mu>(original:&mut Option<ArtifactDerivedSnapshot<P>>,snapshots:&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,mutations:&Arc<dyn ArtifactOwnedValueRetirementFactory<Mu>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>
where P:Send+Sync+'static,Mu:Mutation<P>+Send+'static {
    history_read_admission::admit_history_read_retirement(original,snapshots,mutations,grant,|owner,original|*owner.derived=Some(original))
}

/// 🧳️ Admits a typed child without losing the exact original pending owner on refusal.

/// 🔗️ Prices the lease frame of a shared root the holder never owns uniquely: only the last alias pays the deep release.

/// 🔗️ Admits a shared root as a lease without losing the exact original alias on refusal.

pub use semio_framework_value_retirement_owner::{retirement_owner_demands,retirement_owner_close};
pub use semio_framework_os_store_replay::lifted_snapshot_read::{ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS, ArtifactStoreStringVectorRetirement, ArtifactStoreSupersessionRetirement};
pub use semio_framework_os_store_ledger::lifted_snapshot_read::{ArtifactStoreDisplacedOwnerReservation, ArtifactStoreEditMetadata, ArtifactStoreEditRetirement, ArtifactStoreEditRetirementState, ArtifactStoreHistoryMetadataRetirement, ArtifactStoreHistoryMetadataRetirementState, ArtifactStoreOperationRowOwners, ArtifactStoreStringRetirement};

pub struct ArtifactStoreHistoryLedgerRetirement<T> {
    pub ledger: std::mem::ManuallyDrop<Option<ArtifactHistoryLedger<T>>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub pending: std::mem::ManuallyDrop<Option<T>>,
    pub visibility: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::shared::SharedControlledRetirement<semio_framework_os_vcs::ArtifactGroupVisibility>>>,
    pub factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,
    pub closing_factory: std::mem::ManuallyDrop<Option<semio_framework_value::FactoryAuthority>>,
}

impl<T: Send + 'static> ArtifactStoreHistoryLedgerRetirement<T> {
    pub fn new(ledger: ArtifactHistoryLedger<T>, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<T>>) -> Self {
        Self { ledger: std::mem::ManuallyDrop::new(Some(ledger)), active: std::mem::ManuallyDrop::new(None), pending: std::mem::ManuallyDrop::new(None), visibility: std::mem::ManuallyDrop::new(None), factory: std::mem::ManuallyDrop::new(Some(factory)), closing_factory: std::mem::ManuallyDrop::new(None) }
    }
    pub fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() { return artifact_retirement_box_demands(active, copy); }
        if self.visibility.is_some() { return retirement_owner_demands(&self.visibility, copy); }
        if let Some(ledger) = self.ledger.as_ref() {
            if let Some(value) = self.pending.as_ref().or_else(|| ledger.last()) { return Ok(semio_framework_value::RetirementDemand { capacity_bytes: self.factory.as_ref().unwrap().retirement_birth_bytes(value), depth: 2, ..Default::default() }); }
            if !ledger.terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history ledger retains an unresolved original group or reservation")); }
            if ledger.retained_terminal_slot_count() != 0 { ledger.next_terminal_slot_visibility()?; return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
            return Ok(semio_framework_value::RetirementDemand { release_bytes: ledger.next_empty_page_release_byte_demand()?, depth: 1, ..Default::default() });
        }
        if self.factory.is_some() { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
        if let Some(factory) = self.closing_factory.as_ref() {
            let mut demand = factory.demands(copy)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history factory capability depth overflow"))?;
            return Ok(demand);
        }
        Ok(Default::default())
    }
}

impl<T: Send + 'static> ErasedSnapshotRetirement for ArtifactStoreHistoryLedgerRetirement<T> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_progress};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history ledger exceeds admitted original owner depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let grant = RetainedCloneGrant { maximum_items: 1, ..grant };
        if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, grant); }
        if self.visibility.is_some() { return retirement_owner_close(&mut self.visibility, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(ledger) = self.ledger.as_mut() {
            if let Some(value) = self.pending.take().or_else(|| ledger.pop()) {
                let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
                return match self.factory.as_ref().unwrap().retire_owned(value, child) {
                    Ok((owner, progress)) => { *self.active = Some(owner); admit_retained_clone_progress(child, progress, "original history entry constructor")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "history entry factory changed its exact declared birth")); } Ok(RetainedCloneStep::Progress(progress)) }
                    Err((error, value)) => { *self.pending = Some(value); Err(error) }
                };
            }
            if ledger.retained_terminal_slot_count() != 0 {
                let (visibility, progress) = ledger.retire_terminal_slot(grant)?;
                *self.visibility = visibility.map(semio_framework_value::retirement::shared::SharedControlledRetirement::lease);
                return Ok(RetainedCloneStep::Progress(progress));
            }
            if !ledger.backing_is_empty() { return ledger.release_empty_page(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
            drop(self.ledger.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if self.factory.is_some() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = self.factory.take().unwrap();
            *self.closing_factory = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        let factory = self.closing_factory.as_mut().unwrap();
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let step = factory.step(child)?;
        semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "original history factory capability")?;
        if factory.terminal_is_empty() { drop(self.closing_factory.take()); }
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }
    fn terminal_is_empty(&self) -> bool { self.ledger.is_none() && self.active.is_none() && self.pending.is_none() && self.visibility.is_none() && self.factory.is_none() && self.closing_factory.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<T> Drop for ArtifactStoreHistoryLedgerRetirement<T> {
    fn drop(&mut self) {
        let empty = self.ledger.is_none() && self.active.is_none() && self.pending.is_none() && self.visibility.is_none() && self.factory.is_none() && self.closing_factory.is_none();
        assert!(std::thread::panicking() || empty, "original history ledger retains its payloads, visibility, native backing and factory until exact terminal");
        if empty { unsafe { std::mem::ManuallyDrop::drop(&mut self.ledger); std::mem::ManuallyDrop::drop(&mut self.active); std::mem::ManuallyDrop::drop(&mut self.pending); std::mem::ManuallyDrop::drop(&mut self.visibility); std::mem::ManuallyDrop::drop(&mut self.factory); std::mem::ManuallyDrop::drop(&mut self.closing_factory); } }
    }
}

pub struct ArtifactStoreVcsRetirement<P, Mutation> {
    pub vcs: std::mem::ManuallyDrop<Option<ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation>>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub phase: ArtifactStoreVcsRetirementPhase,
    pub initial_snapshot_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,
    pub mutation_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>>,
    pub closing_factories: std::mem::ManuallyDrop<[Option<semio_framework_value::FactoryAuthority>; 2]>,
}





#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct ArtifactStoreCheckpointRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<Checkpoint> for ArtifactStoreCheckpointRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &Checkpoint) -> usize { size_of::<ArtifactStoreHistoryMetadataRetirement>() }
    fn retire_owned(&self, value: Checkpoint, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Checkpoint)> {
        admit_artifact_retirement(value, grant, ArtifactStoreHistoryMetadataRetirement::checkpoint)
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct ArtifactStoreAlternativeRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<Alternative> for ArtifactStoreAlternativeRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &Alternative) -> usize { size_of::<ArtifactStoreHistoryMetadataRetirement>() }
    fn retire_owned(&self, value: Alternative, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Alternative)> {
        admit_artifact_retirement(value, grant, ArtifactStoreHistoryMetadataRetirement::alternative)
    }
}

pub fn artifact_history_ledger_birth<T: Send + 'static, F: semio_framework_value::FactoryPayloadRetirement>() -> Result<usize, ValueError> {
    size_of::<ArtifactStoreHistoryLedgerRetirement<T>>().checked_add(semio_framework_value::factory_arc_birth_bytes::<F>()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history owner constructor extent overflow"))
}

pub fn artifact_history_ledger_handoff<T: Send + 'static, F: ArtifactOwnedValueRetirementFactory<T> + semio_framework_value::FactoryPayloadRetirement>(ledger: &mut ArtifactHistoryLedger<T>, factory: F, slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
    let capacity = artifact_history_ledger_birth::<T, F>()?;
    if grant.maximum_items == 0 || grant.maximum_capacity_bytes < capacity { return Ok(RetainedCloneStep::Progress(Default::default())); }
    if grant.maximum_depth < 2 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history owner constructor requires original child depth")); }
    let original = std::mem::replace(ledger, ArtifactHistoryLedger::empty());
    *slot = Some(Box::new(ArtifactStoreHistoryLedgerRetirement::new(original, Arc::new(factory))));
    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: capacity, ..Default::default() }))
}

impl<P, Mutation> ArtifactStoreVcsRetirement<P, Mutation> {
    pub fn new(vcs: ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation>, initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>) -> Self {
        Self { vcs: std::mem::ManuallyDrop::new(Some(vcs)), active: std::mem::ManuallyDrop::new(None), phase: ArtifactStoreVcsRetirementPhase::Mutations, initial_snapshot_factory: std::mem::ManuallyDrop::new(Some(initial_snapshot_factory)), mutation_factory: std::mem::ManuallyDrop::new(Some(mutation_factory)), closing_factories: std::mem::ManuallyDrop::new([None, None]) }
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactStoreVcsRetirement<P, Mutation> {
    pub fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() { return artifact_retirement_box_demands(active, copy); }
        let Some(vcs) = self.vcs.as_ref() else {
            if self.initial_snapshot_factory.is_some() || self.mutation_factory.is_some() { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
            if let Some(factory) = self.closing_factories.iter().flatten().next() { let mut demand = factory.demands(copy)?; demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "VCS factory depth overflow"))?; return Ok(demand); }
            return Ok(Default::default());
        };
        let capacity_bytes = match self.phase {
            ArtifactStoreVcsRetirementPhase::Edits if !vcs.edits.backing_is_empty() || !vcs.edits.terminal_is_empty() => artifact_history_ledger_birth::<Edit<Mutation>, ArtifactStoreDecodedEditRetirementFactory<Mutation>>()?,
            ArtifactStoreVcsRetirementPhase::Changes if !vcs.changes.backing_is_empty() || !vcs.changes.terminal_is_empty() => artifact_history_ledger_birth::<Change, ArtifactStoreChangeRetirementFactory>()?,
            ArtifactStoreVcsRetirementPhase::Checkpoints if !vcs.checkpoints.backing_is_empty() || !vcs.checkpoints.terminal_is_empty() => artifact_history_ledger_birth::<Checkpoint, ArtifactStoreCheckpointRetirementFactory>()?,
            ArtifactStoreVcsRetirementPhase::Alternatives if !vcs.alternatives.backing_is_empty() || !vcs.alternatives.terminal_is_empty() => artifact_history_ledger_birth::<Alternative, ArtifactStoreAlternativeRetirementFactory>()?,
            ArtifactStoreVcsRetirementPhase::InitialSnapshot => size_of::<ArtifactGenesisRetirement<P>>(),
            _ => 0,
        };
        Ok(semio_framework_value::RetirementDemand { capacity_bytes, depth: if capacity_bytes == 0 { 1 } else { 2 }, ..Default::default() })
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ErasedSnapshotRetirement for ArtifactStoreVcsRetirement<P, Mutation> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "VCS retirement exceeds admitted original depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let grant = RetainedCloneGrant { maximum_items: 1, ..grant };
        if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, grant); }
        if self.vcs.is_none() {
            if self.initial_snapshot_factory.is_some() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = self.initial_snapshot_factory.take().unwrap(); self.closing_factories[0] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); }
            if self.mutation_factory.is_some() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = self.mutation_factory.take().unwrap(); self.closing_factories[1] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); }
            let slot = self.closing_factories.iter_mut().find(|slot| slot.is_some()).unwrap();
            let factory = slot.as_mut().unwrap();
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = factory.step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "original VCS factory capability")?;
            if factory.terminal_is_empty() { drop(slot.take()); }
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        let vcs = self.vcs.as_mut().unwrap();
        match self.phase {
            ArtifactStoreVcsRetirementPhase::Mutations => self.phase = ArtifactStoreVcsRetirementPhase::Edits,
            ArtifactStoreVcsRetirementPhase::Edits => {
                self.phase = ArtifactStoreVcsRetirementPhase::Changes;
                if demand.capacity_bytes != 0 { return artifact_history_ledger_handoff(&mut vcs.edits, ArtifactStoreDecodedEditRetirementFactory { mutation_factory: self.mutation_factory.as_ref().unwrap().clone() }, &mut self.active, grant); }
            }
            ArtifactStoreVcsRetirementPhase::Changes => {
                self.phase = ArtifactStoreVcsRetirementPhase::Checkpoints;
                if demand.capacity_bytes != 0 { return artifact_history_ledger_handoff(&mut vcs.changes, ArtifactStoreChangeRetirementFactory, &mut self.active, grant); }
            }
            ArtifactStoreVcsRetirementPhase::Checkpoints => {
                self.phase = ArtifactStoreVcsRetirementPhase::Alternatives;
                if demand.capacity_bytes != 0 { return artifact_history_ledger_handoff(&mut vcs.checkpoints, ArtifactStoreCheckpointRetirementFactory, &mut self.active, grant); }
            }
            ArtifactStoreVcsRetirementPhase::Alternatives => {
                self.phase = ArtifactStoreVcsRetirementPhase::InitialSnapshot;
                if demand.capacity_bytes != 0 { return artifact_history_ledger_handoff(&mut vcs.alternatives, ArtifactStoreAlternativeRetirementFactory, &mut self.active, grant); }
            }
            ArtifactStoreVcsRetirementPhase::InitialSnapshot => {
                if !(vcs.edits.terminal_is_empty() && vcs.edits.backing_is_empty() && vcs.changes.terminal_is_empty() && vcs.changes.backing_is_empty() && vcs.checkpoints.terminal_is_empty() && vcs.checkpoints.backing_is_empty() && vcs.alternatives.terminal_is_empty() && vcs.alternatives.backing_is_empty()) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "VCS genesis handoff still retains original history backing")); }
                let ArtifactVcs { genesis, edits, changes, checkpoints, alternatives } = self.vcs.take().unwrap();
                drop((edits, changes, checkpoints, alternatives));
                *self.active = Some(Box::new(ArtifactGenesisRetirement::new(genesis, self.initial_snapshot_factory.as_ref().unwrap().clone())));
                self.phase = ArtifactStoreVcsRetirementPhase::Complete;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: demand.capacity_bytes, ..empty }));
            }
            ArtifactStoreVcsRetirementPhase::Complete => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "completed VCS phase still retains its original VCS")),
        }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
    }
    fn terminal_is_empty(&self) -> bool { self.vcs.is_none() && self.active.is_none() && self.initial_snapshot_factory.is_none() && self.mutation_factory.is_none() && self.closing_factories.iter().all(Option::is_none) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P, Mutation> Drop for ArtifactStoreVcsRetirement<P, Mutation> {
    fn drop(&mut self) {
        let empty = self.vcs.is_none() && self.active.is_none() && self.initial_snapshot_factory.is_none() && self.mutation_factory.is_none() && self.closing_factories.iter().all(Option::is_none);
        assert!(std::thread::panicking() || empty, "VCS retirement reached Drop before every exact history backing and factory became terminal-empty");
        if empty { unsafe { std::mem::ManuallyDrop::drop(&mut self.vcs); std::mem::ManuallyDrop::drop(&mut self.active); std::mem::ManuallyDrop::drop(&mut self.initial_snapshot_factory); std::mem::ManuallyDrop::drop(&mut self.mutation_factory); std::mem::ManuallyDrop::drop(&mut self.closing_factories); } }
    }
}

/// 📏️ Quotes the exact cursor frame [`retire_document_envelope`] births before any original field or factory transfers.
pub fn retire_document_envelope_birth_bytes<P: Send + Sync + 'static, Mutation: Send + 'static>() -> usize {
    size_of::<ArtifactStoreEnvelopeRetirement<P, Mutation>>()
}

/// ♻️ Admits whole detached document custody before any original field or factory transfers.
pub fn retire_document_envelope<P: Send + Sync + 'static, Mutation: Send + 'static>(
    envelope: ArtifactEnvelope<P, Mutation>,
    initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
    grant: RetainedCloneGrant,
) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, ArtifactEnvelope<P, Mutation>, Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>)> {
    admit_artifact_retirement((envelope, initial_snapshot_factory, mutation_factory), grant, |(envelope, snapshots, mutations)| ArtifactStoreEnvelopeRetirement::new(envelope, snapshots, mutations))
        .map_err(|(error, (envelope, snapshots, mutations))| (error, envelope, snapshots, mutations))
}

pub struct ArtifactEnvelopeMetadata {
    pub schema: String,
    pub id: String,
    pub backbone: Option<ArtifactBackboneRef>,
    pub active_alternative_id: Option<String>,
    pub cursor: Option<ArtifactCursor>,
    pub dialect: Option<semio_framework_artifact_reference::ArtifactDialect>,
    pub migrated_from: Option<MigrationProvenance>,
    pub owner: Option<OwnerRef>,
    pub lanes: protocol::HistoryFoldIndex<String, HistoryLane>,
    pub viewer_checkpoint_id: Option<String>,
    pub conflicts: Vec<semio_framework_os_spr::Conflict>,
    pub transitions: Vec<semio_framework_os_spr::MutationEnvelope>,
    pub open_transaction: Option<OpenToolTransaction>,
}

pub struct ArtifactStoreEnvelopeRetirement<P, Mutation> {
    pub vcs: std::mem::ManuallyDrop<Option<ArtifactStoreVcsRetirement<P, Mutation>>>,
    pub messages: std::mem::ManuallyDrop<Option<ArtifactEditMessageLedgerRetirement>>,
    pub metadata: std::mem::ManuallyDrop<Option<ArtifactEnvelopeMetadata>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}

impl<P, Mutation> ArtifactStoreEnvelopeRetirement<P, Mutation> {
    pub fn new(envelope: ArtifactEnvelope<P, Mutation>, snapshots: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, mutations: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>) -> Self {
        let ArtifactEnvelopeOwners { schema, id, vcs, backbone, active_alternative_id, cursor, dialect, migrated_from, owner, lanes, viewer_checkpoint_id, edit_messages, conflicts, transitions, history_shape: _, open_transaction } = envelope.into_owners();
        Self {
            vcs: std::mem::ManuallyDrop::new(Some(ArtifactStoreVcsRetirement::new(vcs, snapshots, mutations))),
            messages: std::mem::ManuallyDrop::new(Some(ArtifactEditMessageLedgerRetirement::new(edit_messages))),
            metadata: std::mem::ManuallyDrop::new(Some(ArtifactEnvelopeMetadata { schema, id, backbone, active_alternative_id, cursor, dialect, migrated_from, owner, lanes, viewer_checkpoint_id, conflicts, transitions, open_transaction })),
            active: std::mem::ManuallyDrop::new(None),
        }
    }
    pub fn ownership_is_empty(&self) -> bool { self.vcs.is_none() && self.messages.is_none() && self.metadata.is_none() && self.active.is_none() }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactStoreEnvelopeRetirement<P, Mutation> {
    pub fn demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() { return artifact_retirement_box_demands(active, body); }
        if self.vcs.is_some() { return retirement_owner_demands(&self.vcs, body); }
        if self.messages.is_some() { return retirement_owner_demands(&self.messages, body); }
        artifact_retirement_owned_birth_demands(&self.metadata)
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ErasedSnapshotRetirement for ArtifactStoreEnvelopeRetirement<P, Mutation> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.ownership_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let step = if self.active.is_some() { artifact_retirement_box_close_step(&mut self.active, grant) }
            else if self.vcs.is_some() { retirement_owner_close(&mut self.vcs, grant) }
            else if self.messages.is_some() { retirement_owner_close(&mut self.messages, grant) }
            else { artifact_retirement_admit_owned(&mut self.metadata, &mut self.active, grant) }?;
        Ok(if self.ownership_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }
    fn terminal_is_empty(&self) -> bool { self.ownership_is_empty() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P, Mutation> Drop for ArtifactStoreEnvelopeRetirement<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.ownership_is_empty(), "whole envelope reached Drop before every original VCS, message backing, metadata and factory owner closed");
        if self.ownership_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.vcs); std::mem::ManuallyDrop::drop(&mut self.messages); std::mem::ManuallyDrop::drop(&mut self.metadata); std::mem::ManuallyDrop::drop(&mut self.active); } }
    }
}

pub struct ArtifactGenesisRetirement<P: Send + Sync + 'static> {
    pub snapshot: std::mem::ManuallyDrop<Option<ReturnedSnapshotReadRetirement<P>>>,
    pub pack: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::shared::SharedControlledRetirement<Vec<u8>>>>,
}

impl<P: Send + Sync + 'static> ArtifactGenesisRetirement<P> {
    pub fn new(genesis: AdmittedArtifactGenesis<P>, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>) -> Self {
        let (snapshot, pack) = genesis.into_owners();
        Self { snapshot: std::mem::ManuallyDrop::new(Some(ReturnedSnapshotReadRetirement::new(snapshot, factory))), pack: std::mem::ManuallyDrop::new(Some(semio_framework_value::retirement::shared::SharedControlledRetirement::lease(pack))) }
    }
    pub fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.snapshot.is_some() { retirement_owner_demands(&self.snapshot, copy) } else { retirement_owner_demands(&self.pack, copy) }
    }
}

impl<P: Send + Sync + 'static> ErasedSnapshotRetirement for ArtifactGenesisRetirement<P> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let grant = RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant };
        let step = if self.snapshot.is_some() { retirement_owner_close(&mut self.snapshot, grant)? } else { retirement_owner_close(&mut self.pack, grant)? };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }
    fn terminal_is_empty(&self) -> bool { self.snapshot.is_none() && self.pack.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P: Send + Sync + 'static> Drop for ArtifactGenesisRetirement<P> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "genesis retired before both immutable owners became terminal-empty");
        if self.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.snapshot); std::mem::ManuallyDrop::drop(&mut self.pack); } }
    }
}

/// 📦️ Retains a displaced shared snapshot with its exact factory until a granted turn constructs its retirement frame.
pub struct ArtifactStoreDisplacedSnapshotRetirement<P: Send + Sync + 'static> {
    pub pending: std::mem::ManuallyDrop<Option<Arc<P>>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub factory: std::mem::ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>,
    pub closing_factory: std::mem::ManuallyDrop<Option<semio_framework_value::FactoryAuthority>>,
}

impl<P: Send + Sync + 'static> ArtifactStoreDisplacedSnapshotRetirement<P> {
    pub fn new(snapshot: Arc<P>, factory: Arc<dyn SnapshotRetirementFactory<P>>) -> Self {
        Self { pending: std::mem::ManuallyDrop::new(Some(snapshot)), active: std::mem::ManuallyDrop::new(None), factory: std::mem::ManuallyDrop::new(Some(factory)), closing_factory: std::mem::ManuallyDrop::new(None) }
    }

    pub fn boxed(snapshot: Arc<P>, factory: Arc<dyn SnapshotRetirementFactory<P>>) -> Box<dyn ErasedSnapshotRetirement> { Box::new(Self::new(snapshot, factory)) }

    pub fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "displaced snapshot depth overflow"))?; Ok(demand) };
        if let Some(snapshot) = self.pending.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "displaced snapshot lost its original factory"))?.retirement_birth_bytes(snapshot), depth: 2, ..Default::default() }); }
        if let Some(active) = self.active.as_ref() { return nested(semio_framework_value::factory_ticket_demands(active, copy)?); }
        if self.factory.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if let Some(authority) = self.closing_factory.as_ref() { return nested(authority.demands(copy)?); }
        Ok(Default::default())
    }
}

impl<P: Send + Sync + 'static> ErasedSnapshotRetirement for ArtifactStoreDisplacedSnapshotRetirement<P> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::admit_retained_clone_close;
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "displaced snapshot exceeds admitted depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(snapshot) = self.pending.take() {
            return match self.factory.as_ref().expect("pending displaced snapshot retains its original factory").retire(snapshot, child) {
                Ok((owner, progress)) => {
                    *self.active = Some(owner);
                    if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "displaced snapshot constructor changed its admitted receipt")); }
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, snapshot)) => { *self.pending = Some(snapshot); Err(error) }
            };
        }
        if self.active.is_some() { return semio_framework_value::close_factory_ticket(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(factory) = self.factory.take() {
            *self.closing_factory = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        let authority = self.closing_factory.as_mut().expect("displaced snapshot retains its closing factory");
        let step = authority.step(child)?;
        let step = admit_retained_clone_close(child, step, authority.terminal_is_empty(), "displaced snapshot factory capability")?;
        if authority.terminal_is_empty() { drop(self.closing_factory.take()); }
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }
    fn terminal_is_empty(&self) -> bool { self.pending.is_none() && self.active.is_none() && self.factory.is_none() && self.closing_factory.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P: Send + Sync + 'static> Drop for ArtifactStoreDisplacedSnapshotRetirement<P> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "displaced snapshot retirement abandoned its original snapshot or factory capability");
        if self.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.pending); std::mem::ManuallyDrop::drop(&mut self.active); std::mem::ManuallyDrop::drop(&mut self.factory); std::mem::ManuallyDrop::drop(&mut self.closing_factory); } }
    }
}



pub const ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY: usize = 1_024;

pub const ARTIFACT_STORE_DISPLACED_RESERVATION_CAPACITY: usize = 8;

/// 🌡️ One-item retirement steps a change spends on the displaced queue while it is under pressure (`relieve_displaced_pressure`).
pub const DISPLACED_RELIEF_STEPS: usize = 16;

/// 🌡️ Displaced-owner occupancy at which a store reports maintenance PRESSURE — a quarter of the
/// fixed queue. Every commit displaces its envelope, current snapshot and dag (three owners, more
/// with a tail undo entry) and the round-robin maintenance pump drains ONE owner step per visit, so
/// a live cadence of coalesced commits (a playback tick, a gumball drag step) fills the queue a
/// hundred times faster than the fair rotation empties it. Above this mark the host drains the
/// pressured queue out of turn and in bursts until it is back under.
pub const ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY: usize = ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY / 4;

pub struct ArtifactStoreDisplacedRetirements {
    pub owners: std::mem::ManuallyDrop<VecDeque<Box<dyn ErasedSnapshotRetirement>>>,
    owner_reservations: [Option<(u64, usize)>; ARTIFACT_STORE_DISPLACED_RESERVATION_CAPACITY],
    owner_generation: u64,
}



impl ArtifactStoreDisplacedRetirements {
    pub fn new() -> Self {
        Self { owners: std::mem::ManuallyDrop::new(VecDeque::with_capacity(ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY)), owner_reservations: [None; ARTIFACT_STORE_DISPLACED_RESERVATION_CAPACITY], owner_generation: 0 }
    }

    pub fn reserve(&self, count: usize) -> Result<(), VcsError> {
        let occupied = self.owners.len().checked_add(self.owner_reservations.iter().flatten().map(|(_, count)| *count).sum::<usize>()).ok_or_else(|| VcsError::ValidationFailed("artifact store displaced-owner occupancy overflowed".into()))?;
        if count > ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY.saturating_sub(occupied) {
            return Err(VcsError::ValidationFailed("artifact store displaced-owner fixed retirement authority is saturated".into()));
        }
        Ok(())
    }

    pub fn reserve_owner_slots(&mut self, count: usize) -> Result<ArtifactStoreDisplacedOwnerReservation, VcsError> {
        if count == 0 {
            return Err(VcsError::ValidationFailed("artifact store displaced-owner reservation is empty".into()));
        }
        let slot = self.owner_reservations.iter().position(Option::is_none).ok_or_else(|| VcsError::ValidationFailed("artifact store displaced-owner reservation registry is saturated".into()))?;
        self.reserve(count)?;
        let generation = self.owner_generation.checked_add(1).ok_or_else(|| VcsError::ValidationFailed("artifact store displaced-owner reservation generation exhausted".into()))?;
        self.owner_generation = generation;
        self.owner_reservations[slot] = Some((generation, count));
        Ok(ArtifactStoreDisplacedOwnerReservation { slot: slot as u8, generation, remaining: count })
    }

    pub fn push_owner_reserved(&mut self, reservation: &mut ArtifactStoreDisplacedOwnerReservation, owner: Box<dyn ErasedSnapshotRetirement>) -> Result<(), Box<dyn ErasedSnapshotRetirement>> {
        let slot = reservation.slot as usize;
        if reservation.remaining == 0 || self.owner_reservations.get(slot).copied().flatten() != Some((reservation.generation, reservation.remaining)) || self.owners.len() >= ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY {
            return Err(owner);
        }
        reservation.remaining -= 1;
        self.owner_reservations[slot] = (reservation.remaining != 0).then_some((reservation.generation, reservation.remaining));
        self.owners.push_back(owner);
        Ok(())
    }

    pub fn release_owner_slots(&mut self, reservation: ArtifactStoreDisplacedOwnerReservation) -> Result<(), VcsError> {
        if reservation.remaining == 0 {
            if self.owner_reservations[reservation.slot as usize].is_some_and(|(generation, _)| generation == reservation.generation) {
                return Err(VcsError::ValidationFailed("artifact store consumed displaced-owner reservation retained live credits".into()));
            }
            return Ok(());
        }
        if self.owner_reservations[reservation.slot as usize] != Some((reservation.generation, reservation.remaining)) {
            return Err(VcsError::ValidationFailed("artifact store displaced-owner reservation is stale or partially consumed".into()));
        }
        self.owner_reservations[reservation.slot as usize] = None;
        Ok(())
    }

    pub fn push_reserved(&mut self, owner: Box<dyn ErasedSnapshotRetirement>) {
        let reserved = self.owner_reservations.iter().flatten().map(|(_, count)| *count).sum::<usize>();
        assert!(self.owners.len() < ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY.saturating_sub(reserved), "artifact store displaced-owner admission token was consumed after its fixed slot changed");
        self.owners.push_back(owner);
    }

    pub fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(owner) = self.owners.front() {
            let mut demand = artifact_retirement_box_demands(owner, copy)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "displaced queue retirement depth overflow"))?;
            return Ok(demand);
        }
        if self.owner_reservations.iter().any(Option::is_some) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "displaced queue retains reserved original owner slots")); }
        Ok(Default::default())
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        let Some(owner) = self.owners.front_mut() else { return Ok(RetainedCloneStep::Complete(empty)); };
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "displaced queue exceeds admitted original child depth")); }
        if grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || (demand.copy_bytes != 0 && grant.maximum_copy_bytes == 0) { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if owner.terminal_is_empty() {
            let bytes = size_of_val(owner.as_ref());
            drop(self.owners.pop_front());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..empty }));
        }
        let step = owner.close_step(child)?;
        let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owner.terminal_is_empty(), "original displaced queue child")?;
        if matches!(step, RetainedCloneStep::Complete(_)) && !owner.terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "displaced child reported completion while original ownership remains")); }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.owners.is_empty() && self.owner_reservations.iter().all(Option::is_none)
    }

    pub fn backing_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "displaced backing remains occupied by original owners or reservations")); }
        std::alloc::Layout::array::<Box<dyn ErasedSnapshotRetirement>>(self.owners.capacity()).map(|layout| layout.size()).map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "displaced original queue backing extent overflow"))
    }

    pub fn release_empty_backing_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        let bytes = self.backing_release_byte_demand()?;
        if bytes == 0 { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 || grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "displaced backing requires original frame depth")); }
        drop(std::mem::take(&mut *self.owners));
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..empty }))
    }

    /// 🔭️ Why this queue is not terminal — a queued owner the closer is draining, or a RESERVED
    /// slot no owner ever filled. The two answer `Blocked` from the same arm and need different
    /// cures, so the census names them apart.
    pub fn witness(&self) -> String {
        let reserved = self.owner_reservations.iter().flatten().map(|(_, count)| *count).sum::<usize>();
        format!("owners={}/reserved={}", self.owners.len(), reserved)
    }

    pub fn under_pressure(&self) -> bool {
        self.owners.len() >= ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY
    }
}

impl Drop for ArtifactStoreDisplacedRetirements {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "artifact store displaced-owner authority reached Drop before bounded retirement completed");
        let owners = unsafe { std::mem::ManuallyDrop::take(&mut self.owners) };
        drop(owners);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactStoreCursorDisposerPhase {
    ReturnedReads,
    Displaced,
    RuntimeOwners(u8),
    PrefixSnapshots,
    TailSnapshot,
    CurrentSnapshot,
    Backbone,
    Causal,
    Envelope,
    Complete,
}


