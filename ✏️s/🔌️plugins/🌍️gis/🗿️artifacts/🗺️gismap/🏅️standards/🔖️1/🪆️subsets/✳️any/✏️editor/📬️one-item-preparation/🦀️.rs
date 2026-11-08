//! 📬️ One-item preparation of the GIS 2D play app: the stamped and unstamped preparation factories and the one-item store publication of a map edit. See [`super`].

use crate::editor::gis2d::modes::edit;
use crate::editor::gis2d::modes::edit::windows::map;
use crate::standards::v1::subsets::any::schema::mutations::GisMapMutation;
use crate::GisMapSnapshot;

//#region 🔖️Gis2dPlayApp
//#region 📬️OneItemPreparation
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct Gis2dOneItemPreparationFactory<P, M> {
    marker: std::marker::PhantomData<fn() -> (P, M)>,
    #[factory_owned]
    stamp: GisMapStampSlot,
}

impl<P, M> Default for Gis2dOneItemPreparationFactory<P, M> {
    fn default() -> Self {
        Self { marker: std::marker::PhantomData, stamp: GisMapStampSlot(None) }
    }
}

/// 🛡️ Server-derived identity carried into one exact Store edit without caller-visible seal authority.
#[derive(Clone)]
pub struct GisMapOneItemStampV1 {
    pub mutation_id: protocol::MutationId,
    pub timestamp: protocol::HybridLogicalTimestamp,
}

semio_framework_value::artifact_retire_struct!(GisMapOneItemStampV1 { mutation_id, timestamp });

/// 🏷️ The factory-owned optional stamp: its heap-bearing mutation id retires through a funded controlled owner.
#[derive(Clone)]
pub struct GisMapStampSlot(Option<GisMapOneItemStampV1>);

impl GisMapStampSlot {
    pub fn as_ref(&self) -> Option<&GisMapOneItemStampV1> {
        self.0.as_ref()
    }
}

impl semio_framework_value::FactoryPayloadRetirement for GisMapStampSlot {
    type CloseState = Option<semio_framework_value::retirement::controlled::ControlledRetirement<GisMapOneItemStampV1>>;
    fn close_state_birth_bytes(&self) -> usize { 0 }
    fn close_state_constructor_depth(&self) -> usize { 0 }
    fn close_state_constructor_copy_bytes(&self) -> usize { 0 }
    fn prepare_close_state(&self) -> Self::CloseState { None }
    fn close_state_preparation_demands(&self, _: &Self::CloseState, _: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { Ok(Default::default()) }
    fn close_state_preparation_work_demand(&self, _: &Self::CloseState, _: usize) -> Result<u64, semio_framework_value::ValueError> { Ok(0) }
    fn prepare_close_state_step(&self, _: &mut Self::CloseState, _: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> semio_framework_value::RetainedWorkPoll { semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome { result: Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default())), work: 0 }) }
    fn close_state_preparation_is_complete(_: &Self::CloseState) -> bool { true }
    fn transfer_payload(value: Self, state: &mut Self::CloseState) {
        if let Some(stamp) = value.0 {
            *state = Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(stamp).unwrap_or_else(|(error, _)| panic!("GIS stamp supports controlled retirement: {error}")));
        }
    }
    fn close_state_demands(state: &Self::CloseState, _: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        state.as_ref().map_or(Ok(Default::default()), |owner| Ok(semio_framework_value::RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(owner.next_copy_byte_demand()?)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? }))
    }
    fn close_state_work_demand(state: &Self::CloseState, body: usize) -> Result<u64, semio_framework_value::ValueError> { state.as_ref().map_or(Ok(0), |owner| owner.next_computational_work(body)) }
    fn close_state_step(state: &mut Self::CloseState, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> semio_framework_value::OriginalRetirementPoll {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let Some(owner) = state.as_mut() else { return semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome { result: Ok(RetainedCloneStep::Complete(Default::default())), work: 0 }) };
        if !owner.terminal_is_empty() { return owner.step(effect) }
        let step = if effect.grant().maximum_items == 0 { RetainedCloneStep::Progress(Default::default()) } else { drop(state.take()); RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }) };
        semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome { result: Ok(step), work: 0 })
    }
    fn close_state_terminal_is_empty(state: &Self::CloseState) -> bool { state.is_none() }
}

/// 📍 Creates the exact parent Map preparation port used by the retained fixed-three assembly.
pub fn gis_map_parent_one_item_preparation_factory() -> std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<GisMapSnapshot, GisMapMutation>> {
    std::sync::Arc::new(Gis2dOneItemPreparationFactory::default())
}

/// 🎨 Creates the exact drawing-child preparation port used by the retained fixed-three assembly.
pub fn gis_map_drawing_one_item_preparation_factory() -> std::sync::Arc<
    dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<
        semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot,
        semio_s_artifact_stdio_semio_drawing::schema::mutations::SemioDrawingMutation,
    >,
> {
    std::sync::Arc::new(Gis2dOneItemPreparationFactory::default())
}

/// 🔢 Creates the exact value-child preparation port used by the retained fixed-three assembly.
pub fn gis_map_value_one_item_preparation_factory() -> std::sync::Arc<
    dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<
        semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot,
        semio_s_artifact_stdio_semio_value_mutations::schema::mutations::SemioValueMutation,
    >,
> {
    std::sync::Arc::new(Gis2dOneItemPreparationFactory::default())
}

/// 🆕️ Creates the exact stamped parent preparation port used by the Hub-owned fixed-three committer.
pub fn gis_map_parent_stamped_one_item_preparation_factory(stamp: GisMapOneItemStampV1) -> std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<GisMapSnapshot, GisMapMutation>> {
    std::sync::Arc::new(Gis2dOneItemPreparationFactory { marker: std::marker::PhantomData, stamp: GisMapStampSlot(Some(stamp)) })
}

/// ✏️ Creates the exact stamped drawing preparation port used by the Hub-owned fixed-three committer.
pub fn gis_map_drawing_stamped_one_item_preparation_factory(
    stamp: GisMapOneItemStampV1,
) -> std::sync::Arc<
    dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<
        semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot,
        semio_s_artifact_stdio_semio_drawing::schema::mutations::SemioDrawingMutation,
    >,
> {
    std::sync::Arc::new(Gis2dOneItemPreparationFactory { marker: std::marker::PhantomData, stamp: GisMapStampSlot(Some(stamp)) })
}

/// 🛞️ Creates the exact stamped value preparation port used by the Hub-owned fixed-three committer.
pub fn gis_map_value_stamped_one_item_preparation_factory(
    stamp: GisMapOneItemStampV1,
) -> std::sync::Arc<
    dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<
        semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot,
        semio_s_artifact_stdio_semio_value_mutations::schema::mutations::SemioValueMutation,
    >,
> {
    std::sync::Arc::new(Gis2dOneItemPreparationFactory { marker: std::marker::PhantomData, stamp: GisMapStampSlot(Some(stamp)) })
}

struct Gis2dOneItemPreparation<P: 'static, M: 'static> {
    owners: store::OneItemOwners<P, M>,
    stamp: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<GisMapOneItemStampV1>>>,
    checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    last_work: u64,
    cancelled: bool,
}

/// 🛂️ The authority-minted one-item edit, carrying the Hub-derived identity when a stamped committer prepares it.
fn gis2d_one_item_edit<M>(forward: M, inverse: Vec<M>, authority: &store::os_store::one_item_publication::ArtifactStoreOneItemLiveAuthority, stamp: Option<GisMapOneItemStampV1>) -> protocol::Edit<M> {
    let mut edit = authority.next_edit(forward, inverse);
    if let Some(stamp) = stamp {
        edit.id = stamp.mutation_id.0.clone();
        edit.mutation_meta[0].timestamp = stamp.timestamp;
        edit.mutation_meta[0].mutation_id = Some(stamp.mutation_id);
    }
    edit
}

/// 📏 The depth-nested demand of the next stamp retirement turn.
fn gis2d_stamp_close_demand(owner: &semio_framework_value::retirement::controlled::ControlledRetirement<GisMapOneItemStampV1>, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
    let mut demand = owner.next_demand(body)?;
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "preparation close depth overflow"))?;
    Ok(demand)
}

impl<P, M> Gis2dOneItemPreparation<P, M>
where P: semio_framework_value::retirement::RetireOwned + Send + Sync + 'static, M: semio_framework_value::retirement::RetireOwned + Send + 'static,
{
    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        self.stamp.as_ref().map_or_else(|| self.owners.close_demands(body), |owner| gis2d_stamp_close_demand(owner, body))
    }
}

impl<P, M> store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<P, M> for Gis2dOneItemPreparationFactory<P, M>
where
    P: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: protocol::Mutation<P> + protocol::ColdMutation<P> + semio_framework_value::retirement::RetireOwned + store::ArtifactCanonicalJsonTree + Send + Sync + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<M>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<M>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn stamped_clock(&self) -> Option<protocol::HybridLogicalTimestamp> {
        self.stamp.as_ref().map(|stamp| stamp.timestamp)
    }

    fn stamped_mutation_id(&self) -> Option<&protocol::MutationId> {
        self.stamp.as_ref().map(|stamp| &stamp.mutation_id)
    }

    fn preflight(&self, mutation: &M, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("GIS map retained preparation rejected its lane".into());
        }
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf::<P, M>(mutation, store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &M, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        let capacity_bytes = size_of::<Gis2dOneItemPreparation<P, M>>().checked_add(self.stamp.0.as_ref().map_or(0, |stamp| stamp.mutation_id.0.len())).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "GIS original stamp birth extent overflow"))?;
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes, depth: 1 })
    }

    fn begin_copy_byte_demand(&self, mutation: &M, lane: store::os_store::store_schema::HistoryLane) -> Result<usize, semio_framework_value::ValueError> {
        self.begin_demand(mutation, lane)?;
        size_of::<store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<P, M, M>>().checked_add(size_of::<Gis2dOneItemPreparation<P, M>>()).and_then(|bytes| bytes.checked_add(self.stamp.0.as_ref().map_or(0, |stamp| stamp.mutation_id.0.len()))).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original preparation header quote overflow"))
    }

    fn begin(&self, request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<P, M, M>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<P, M>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<P, M, M>)> {
        if request.lane != store::os_store::store_schema::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "GIS preparation rejected original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let mut progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        let copied_bytes = self.stamp.0.as_ref().map_or(0, |stamp| stamp.mutation_id.0.len());
        if grant.maximum_copy_bytes < copied_bytes { return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "GIS original stamp copy exceeds supplied authority"), request)); }
        progress.copied_bytes = copied_bytes;
        Ok((Box::new(Gis2dOneItemPreparation {
            owners: store::OneItemOwners::from_request(request),
            stamp: std::mem::ManuallyDrop::new(self.stamp.0.clone().map(semio_framework_value::retirement::controlled::ControlledRetirement::retain)),
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            last_work: 0,
            cancelled: false,
        }), progress))
    }
}

impl<P, M> Gis2dOneItemPreparation<P, M>
where
    P: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: protocol::Mutation<P> + protocol::ColdMutation<P> + semio_framework_value::retirement::RetireOwned + Send + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn advance_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use protocol::ColdMutation as _;
        use protocol::MutationDiff as _;
        use semio_framework_value::{ValueError, ValueRefusalKind};
        if self.owners.refused.is_some() { return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal")); }
        if self.owners.prepared.is_some() { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default())); }
        if self.owners.candidate.is_none() {
            self.last_work = 1;
            let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "GIS map retained preparation lost its exact base root"))?;
            let mutation = self.owners.mutation.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "GIS map retained preparation lost its mutation owner"))?;
            let inverse = mutation.inverse(base.get())?;
            let post = protocol::apply_diff_cold(mutation.diff(base.get()).diff(), base.get()).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "preparation retained the original mutation application refusal"))?;
            let mutation = self.owners.mutation.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "GIS map retained preparation lost its mutation owner"))?;
            *self.owners.candidate = Some((post, inverse, mutation));
            self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: [0; 32] };
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, Default::default()));
        }
        let authority = self.owners.authority.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "GIS map retained preparation lost its Store authority"))?;
        let line_bytes = authority.line_id().map_or(0, str::len);
        if grant.maximum_copy_bytes < line_bytes { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); }
        self.last_work = 1;
        let (post, inverse, mutation) = self.owners.candidate.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "GIS map retained preparation lost its semantic candidate"))?;
        let edit = gis2d_one_item_edit(mutation, inverse, authority, self.stamp.as_mut().and_then(semio_framework_value::retirement::controlled::ControlledRetirement::take_original));
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => { *self.owners.refused = Some((edit, post)); return Err(error); }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: 1 + line_bytes as u64, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()))
    }

    fn close_stamp_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::OriginalRetirementFailure> {
        use semio_framework_value::retained_clone::{retained_clone_grant_admits, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
        use semio_framework_value::{RetainedWorkPoll, ValueError, ValueRefusalKind};
        let Some(owner) = self.stamp.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())) };
        if owner.terminal_is_empty() { *self.stamp = None; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
        let demand = gis2d_stamp_close_demand(owner, grant.maximum_copy_bytes)?;
        if !retained_clone_grant_admits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let work = owner.next_computational_work(grant.maximum_copy_bytes)?;
        let done = semio_framework_value::AuthoringWallet::new(child, u128::from(work)).run(child, work, |effect| owner.step(effect))?.ok_or_else(|| ValueError::literal(ValueRefusalKind::WorkLimit, "GIS stamp retirement work quote was not admitted"))?;
        match done.output {
            RetainedWorkPoll::Complete(turn) => Ok(RetainedCloneStep::Progress(turn.result?.progress())),
            RetainedWorkPoll::Pending { .. } => Err(ValueError::literal(ValueRefusalKind::WorkLimit, "GIS stamp retirement cannot await synchronously").into()),
        }
    }
}

impl<P, M> store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<P, M> for Gis2dOneItemPreparation<P, M>
where
    P: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: protocol::Mutation<P> + protocol::ColdMutation<P> + semio_framework_value::retirement::RetireOwned + Send + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn advance(&mut self, _body: usize, grant: store::os_store::one_item::ArtifactStoreOneItemGrant, receiving: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::OriginalReceivingPrimary> {
        self.last_work = 0;
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.owners.refused.is_some() || self.owners.prepared.is_some() { return self.advance_step(grant).map_err(Into::into); }
        if !receiving.prepare_work(1)? { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); }
        let result = self.advance_step(grant);
        receiving.commit_work(u128::from(self.last_work))?;
        result.map_err(Into::into)
    }
    fn has_pending_receiving_recovery(&self) -> bool { false }
    fn pending_receiving_demand(&self) -> Option<semio_framework_value::RetirementDemand> { None }
    fn pending_receiving_recovery(&mut self) -> Option<semio_framework_value::native_decoding::construction::NativeRecoveryLoan<'_>> { None }
    fn finish_receiving_recovery(&mut self, _control: &mut semio_framework_value::native_decoding::NativeRetirementDecodeControl<'_, '_>) -> semio_framework_value::RetainedWorkOutcome<semio_framework_value::retained_clone::RetainedCloneProgress, semio_framework_value::OriginalReceivingPrimary> { semio_framework_value::RetainedWorkOutcome { result: Ok(Default::default()), work: 0 } }
    fn last_turn_work(&self) -> u64 { self.last_work }
    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<P, M>> { self.owners.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<P, M>> { self.owners.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.owners.begin_close(); }
    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::OriginalRetirementFailure> {
        let grant = grant.retained_grant();
        if self.stamp.is_none() { return Ok(self.owners.close_step(grant)?); }
        if !self.owners.is_closing() || grant.maximum_items == 0 { return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
        self.close_stamp_step(grant)
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.stamp.is_none() && self.owners.terminal_is_empty() }
}

impl<P: 'static, M: 'static> Drop for Gis2dOneItemPreparation<P, M> {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.stamp.is_none(), "preparation must retain its original stamp until supplied-grant terminal closure"); }
}
//#endregion 📬️OneItemPreparation
//#endregion 🔖️Gis2dPlayApp
