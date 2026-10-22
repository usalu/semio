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
struct GisMapStampSlot(Option<GisMapOneItemStampV1>);

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
    fn prepare_close_state_step(&self, _: &mut Self::CloseState, _: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> { Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default())) }
    fn close_state_preparation_is_complete(_: &Self::CloseState) -> bool { true }
    fn transfer_payload(value: Self, state: &mut Self::CloseState) {
        if let Some(stamp) = value.0 {
            *state = Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(stamp).unwrap_or_else(|(error, _)| panic!("GIS stamp supports controlled retirement: {error}")));
        }
    }
    fn close_state_demands(state: &Self::CloseState, _: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        state.as_ref().map_or(Ok(Default::default()), |owner| Ok(semio_framework_value::RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(owner.next_copy_byte_demand()?)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? }))
    }
    fn close_state_step(state: &mut Self::CloseState, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let Some(owner) = state.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())) };
        if owner.terminal_is_empty() {
            if grant.maximum_items == 0 {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            drop(state.take());
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        owner.step(grant)
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

/// 📍 Creates the exact stamped parent preparation port used by the Hub-owned fixed-three committer.
pub fn gis_map_parent_stamped_one_item_preparation_factory(stamp: GisMapOneItemStampV1) -> std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<GisMapSnapshot, GisMapMutation>> {
    std::sync::Arc::new(Gis2dOneItemPreparationFactory { marker: std::marker::PhantomData, stamp: GisMapStampSlot(Some(stamp)) })
}

/// 🎨 Creates the exact stamped drawing preparation port used by the Hub-owned fixed-three committer.
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

/// 🔢 Creates the exact stamped value preparation port used by the Hub-owned fixed-three committer.
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

struct Gis2dOneItemPreparation<P, M> {
    base: std::mem::ManuallyDrop<Option<store::os_store::snapshot_read::SnapshotRead<P>>>,

    pub(super) mutation: std::mem::ManuallyDrop<Option<M>>,

    inverse: std::mem::ManuallyDrop<Option<Vec<M>>>,
    refused: std::mem::ManuallyDrop<Option<(protocol::Edit<M>, std::sync::Arc<P>)>>,
    apply_refusal: std::mem::ManuallyDrop<Option<protocol::MutationApplyError>>,
    authority: std::mem::ManuallyDrop<Option<std::sync::Arc<store::os_store::one_item_publication::ArtifactStoreOneItemLiveAuthority>>>,

    candidate: std::mem::ManuallyDrop<Option<(P, Vec<M>, M)>>,

    prepared: std::mem::ManuallyDrop<Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<P, M>>>,

    mutation_retirement: std::mem::ManuallyDrop<Option<std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<M>>>>,
    snapshot_retirement: std::mem::ManuallyDrop<Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<P>>>>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    phase: u8,
    cancelled: bool,
    closing: bool,
    stamp: std::mem::ManuallyDrop<Option<GisMapOneItemStampV1>>,

    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    factories: std::mem::ManuallyDrop<[Option<semio_framework_value::FactoryAuthority>; 2]>,
}

/// 🛡️ The authority-minted one-item edit, carrying the Hub-derived identity when a stamped committer prepares it.
fn gis2d_one_item_edit<M>(forward: M, inverse: Vec<M>, authority: &store::os_store::one_item_publication::ArtifactStoreOneItemLiveAuthority, stamp: Option<GisMapOneItemStampV1>) -> protocol::Edit<M> {
    let mut edit = authority.next_edit(forward, inverse);
    if let Some(stamp) = stamp {
        edit.id = stamp.mutation_id.0.clone();
        edit.mutation_meta[0].timestamp = stamp.timestamp;
        edit.mutation_meta[0].mutation_id = Some(stamp.mutation_id);
    }
    edit
}

impl<P, M> Gis2dOneItemPreparation<P, M>
where P: semio_framework_value::retirement::RetireOwned + Send + Sync + 'static, M: semio_framework_value::retirement::RetireOwned + Send + 'static,
{
    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: semio_framework_value::RetirementDemand| -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "preparation close depth overflow"))?; Ok(demand) };
        if let Some(active) = self.active.as_ref() { return nested(store::os_store::snapshot_read::artifact_retirement_box_demands(active, body)?); }
        if self.prepared.is_some() { let birth = store::os_store::one_item_publication::ArtifactStoreOneItemPrepared::<P, M>::retirement_birth_demand(); return Ok(semio_framework_value::RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth + 1, ..Default::default() }); }
        if self.mutation.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.mutation)?); }
        if self.inverse.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.inverse)?); }
        if self.refused.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.refused)?); }
        if self.apply_refusal.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.apply_refusal)?); }
        if self.candidate.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.candidate)?); }
        if self.stamp.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.stamp)?); }
        if self.base.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.base)?); }
        if let Some(authority) = self.authority.as_ref() { let birth = authority.retirement_birth_demand(); return Ok(semio_framework_value::RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth + 1, ..Default::default() }); }
        if self.mutation_retirement.is_some() || self.snapshot_retirement.is_some() { return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<std::sync::Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        self.factories.iter().find_map(Option::as_ref).map_or(Ok(Default::default()), |factory| nested(factory.demands(body)?))
    }
}

impl<P, M> store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<P, M> for Gis2dOneItemPreparationFactory<P, M>
where
    P: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: protocol::Mutation<P> + semio_framework_value::retirement::RetireOwned + store::ArtifactCanonicalJsonTree + Send + Sync + 'static,
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
            base: std::mem::ManuallyDrop::new(Some(request.base)),
            mutation: std::mem::ManuallyDrop::new(Some(request.mutation)),
            inverse: std::mem::ManuallyDrop::new(None),
            refused: std::mem::ManuallyDrop::new(None),
            apply_refusal: std::mem::ManuallyDrop::new(None),
            authority: std::mem::ManuallyDrop::new(Some(request.authority)),
            candidate: std::mem::ManuallyDrop::new(None),
            prepared: std::mem::ManuallyDrop::new(None),
            mutation_retirement: std::mem::ManuallyDrop::new(Some(request.mutation_retirement)),
            snapshot_retirement: std::mem::ManuallyDrop::new(Some(request.snapshot_retirement)),
            active: std::mem::ManuallyDrop::new(None),
            factories: std::mem::ManuallyDrop::new(Default::default()),
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            phase: 0,
            cancelled: false,
            closing: false,
            stamp: std::mem::ManuallyDrop::new(self.stamp.0.clone()),
        }), progress))
    }
}

impl<P, M> store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<P, M> for Gis2dOneItemPreparation<P, M>
where
    P: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: protocol::Mutation<P> + semio_framework_value::retirement::RetireOwned + Send + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.refused.is_some() || self.apply_refusal.is_some() {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal"));
        }
        if self.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()));
        }
        match self.phase {
            0 => {
                let _authority = self.authority.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "GIS map retained preparation lost its Store authority"))?;
                let base = self.base.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "GIS map retained preparation lost its exact base root"))?;
                let mutation = self.mutation.take().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "GIS map retained preparation lost its mutation owner"))?;
                let inverse = match mutation.inverse(base.get()) {
                    Ok(inverse) => inverse,
                    Err(error) => { *self.mutation = Some(mutation); return Err(error); }
                };
                let post = match protocol::apply_diff_cold(mutation.diff(base.get()).diff(), base.get()) {
                    Ok(post) => post,
                    Err(error) => {
                        *self.mutation = Some(mutation);
                        *self.inverse = Some(inverse);
                        *self.apply_refusal = Some(error);
                        return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retained the original mutation application refusal"));
                    }
                };
                *self.candidate = Some((post, inverse, mutation));
                self.phase = 1;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: [0; 32] };
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, Default::default()))
            }
            1 => {
                let authority = self.authority.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "GIS map retained preparation lost its Store authority"))?;

                let line_bytes = authority.line_id().map_or(0, str::len);
                if grant.maximum_copy_bytes < line_bytes { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); }
                let (post, inverse, mutation) = self.candidate.take().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "GIS map retained preparation lost its semantic candidate"))?;
                let edit = gis2d_one_item_edit(mutation, inverse, authority, self.stamp.take());
                let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
                    Ok(prepared) => prepared,
                    Err((error, edit, post)) => { *self.refused = Some((edit, post)); return Err(error); }
                };
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: 1 + line_bytes as u64, digest: prepared.edit_digest() };
                *self.prepared = Some(prepared);
                self.phase = 2;
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()))
            }
            _ => Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked),
        }
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
        let empty = RetainedCloneProgress::default(); let grant = grant.retained_grant();
        if !self.closing || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "preparation close exceeds original depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active.is_some() { return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if self.prepared.is_some() {
            if self.mutation_retirement.is_none() || self.snapshot_retirement.is_none() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation retains its original installed issuers")); }
            let original = self.prepared.take().expect("observed original prepared candidate");
            let mutations = self.mutation_retirement.take().expect("original mutation issuer");
            let snapshots = self.snapshot_retirement.take().expect("original snapshot issuer");
            return match original.admit_retirement(mutations, snapshots, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "original prepared close birth")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation child changed its actual admitted birth")); } Ok(RetainedCloneStep::Progress(progress)) },
                Err((error, original, mutations, snapshots)) => { *self.prepared = Some(original); *self.mutation_retirement = Some(mutations); *self.snapshot_retirement = Some(snapshots); Err(error) },
            };
        }
        if self.mutation.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.mutation, &mut self.active, child); }
        if self.inverse.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.inverse, &mut self.active, child); }
        if self.refused.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.refused, &mut self.active, child); }
        if self.apply_refusal.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.apply_refusal, &mut self.active, child); }
        if self.candidate.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.candidate, &mut self.active, child); }
        if self.stamp.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.stamp, &mut self.active, child); }
        if self.base.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.base, &mut self.active, child); }
        if let Some(authority) = self.authority.take() { return match authority.retire(child) { Ok((owner, progress)) => { *self.active = Some(owner); semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "original preparation authority close birth")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation authority changed admitted birth")); } Ok(RetainedCloneStep::Progress(progress)) }, Err((error, original)) => { *self.authority = Some(original); Err(error) } }; }
        if let Some(factory) = self.mutation_retirement.take() { let factory: std::sync::Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factories[0] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })); }
        if let Some(factory) = self.snapshot_retirement.take() { let factory: std::sync::Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factories[1] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })); }
        if let Some(slot) = self.factories.iter_mut().find(|slot| slot.is_some()) { let factory = slot.as_mut().expect("original preparation factory alias"); let step = factory.step(child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "original preparation factory close")?; if factory.terminal_is_empty() { *slot = None; } return Ok(RetainedCloneStep::Progress(step.progress())); }
        Ok(RetainedCloneStep::Complete(empty))
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.active.is_none() && self.factories.iter().all(Option::is_none) && self.mutation_retirement.is_none() && self.snapshot_retirement.is_none() && self.inverse.is_none() && self.refused.is_none() && self.apply_refusal.is_none() && self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.candidate.is_none() && self.stamp.is_none() && self.prepared.is_none()
    }
}

impl<P, M> Drop for Gis2dOneItemPreparation<P, M> {
    fn drop(&mut self) { assert!(std::thread::panicking() || (self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.prepared.is_none() && self.inverse.is_none() && self.refused.is_none() && self.apply_refusal.is_none() && self.mutation_retirement.is_none() && self.snapshot_retirement.is_none() && self.active.is_none() && self.candidate.is_none() && self.stamp.is_none() && self.factories.iter().all(Option::is_none)), "preparation must retain original owners until supplied-grant terminal closure"); }
}
//#endregion 📬️OneItemPreparation
//#endregion 🔖️Gis2dPlayApp
