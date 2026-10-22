//! 📬️ Store preparation of the VCS play app: the one-item preparation factory and the bounded and resumable proof catalogs. See [`super`].

use super::*;

//#region 🔖️VcsPlayApp
//#region 📬️StorePreparation
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct VcsOneItemPreparationFactory<P, M> {
    pub lane: store::os_store::store_schema::HistoryLane,
    marker: std::marker::PhantomData<fn() -> (P, M)>,
}

impl<P, M> VcsOneItemPreparationFactory<P, M> {
    pub fn new(lane: store::os_store::store_schema::HistoryLane) -> Self {
        Self { lane, marker: std::marker::PhantomData }
    }
}

struct VcsOneItemPreparation<P: 'static, M: 'static> {
    owners: store::OneItemOwners<P, M>,
    pub checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    pub cancelled: bool,
}

impl<P, M> store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<P, M> for VcsOneItemPreparationFactory<P, M>
where
    P: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: protocol::Mutation<P> + semio_framework_value::retirement::RetireOwned + store::ArtifactCanonicalJsonTree + Send + Sync + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<M>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<M>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> { store::admit_artifact_batch_digest(edit, grant) }

    fn preflight(&self, mutation: &M, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != self.lane {
            return Err("VCS one-item preparation rejected its lane".into());
        }
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf::<P, M>(mutation, store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &M, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<VcsOneItemPreparation<P, M>>(), depth: 1 })
    }

    fn begin(&self, request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<P, M, M>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<P, M>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<P, M, M>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };

        if request.lane != self.lane
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "preparation rejected original publication authority"), request));
        }
        Ok((Box::new(VcsOneItemPreparation {
            owners: store::OneItemOwners::from_request(request),

            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
        }), progress))
    }
}

impl<P, M> store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<P, M> for VcsOneItemPreparation<P, M>
where
    P: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: protocol::Mutation<P> + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()));
        }
        let base = self.owners.base.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "VCS one-item preparation lost its exact base root"))?;
        let mutation = self.owners.mutation.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "VCS one-item preparation lost its mutation owner"))?;
        let inverse = mutation.inverse(base.get())?;
        let post = protocol::apply_diff_cold(mutation.diff(base.get()).diff(), base.get()).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation could not apply its original mutation"))?;
        let authority = self.owners.authority.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "VCS one-item preparation lost its Store authority"))?;
        let forward = self.owners.mutation.take().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "VCS one-item preparation lost its mutation owner"))?;
        let edit = authority.next_edit(forward, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, copied_bytes: 0, ..Default::default() }))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<P, M>> { self.owners.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<P, M>> { self.owners.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.owners.begin_close(); }
    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> { self.owners.close_step(grant.retained_grant()) }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.owners.terminal_is_empty() }
}
//#endregion 📬️StorePreparation

//#region 🧾️ProofCatalogs
pub struct VcsBoundedProofs;

impl VcsBoundedProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<VcsPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.vcs.vcs@1/*#editor",
        artifact_schema: "vcs.vcs",
        factory: "VcsBoundedCommandJobFactory",
        factory_type: VcsBoundedCommandJobFactory,
        tools: {
            "incrementCounter" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "renameVcs" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "changeCounter" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "changeStatus" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "changeNotes" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "noMutation" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "canvasPointerDown" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "canvasPointerMove" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "canvasPointerUp" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "canvasWheel" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setActiveExample" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
        }
    }
}

pub struct VcsResumableProofs;

impl VcsResumableProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<VcsPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.vcs.vcs@1/*#editor",
        artifact_schema: "vcs.vcs",
        factory: "VcsResumableCommandJobFactory",
        factory_type: VcsResumableCommandJobFactory,
        tools: {
            "textEdit" => ToolExecutionContract::resumable(8_192, 16_400, 1, 16_384, 7_500, 1, 1),
            "edit" => ToolExecutionContract::resumable(8_192, 16_400, 1, 16_384, 7_500, 1, 1),
        }
    }
}
//#endregion 🧾️ProofCatalogs
//#endregion 🔖️VcsPlayApp
