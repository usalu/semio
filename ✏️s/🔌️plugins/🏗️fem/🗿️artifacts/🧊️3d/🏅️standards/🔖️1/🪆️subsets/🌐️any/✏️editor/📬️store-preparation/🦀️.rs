//! 📬️ Store preparation of the FEM 3D editor: the artifact mutation byte accounting and the one-item store preparation factory. See [`super`].

use super::*;

//#region 📬️ArtifactStorePreparation
/// 📏️ One document mutation's retained envelope, measured on its own canonical binary op rather than
/// on a cloned snapshot — `OpBinary` is the exact shape the store publishes.
fn fem3d_artifact_mutation_retained_bytes(mutation: &Fem3dMutation) -> Result<usize, String> {
    protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "fem3d-artifact-mutation-encode-failed".to_string())
}

fn admit_fem3d_artifact_mutation(mutation: &Fem3dMutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = fem3d_artifact_mutation_retained_bytes(mutation)?;
    if retained_bytes > FEM3D_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("fem3d-artifact-mutation-envelope".into());
    }
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
}

/// 📬️ Required by the Artifact publication lane: every document-editing retained tool emits `Fem3dMutation`s, and
/// `VcsArtifactApp` rejects any tool whose declared lane has no one-item preparation factory with
/// `interactive-job.publication-authority-missing`.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct Fem3dArtifactPreparationFactory;

struct Fem3dArtifactPreparation {
    pub(super) owners: store::OneItemOwners<Fem3dSnapshot, Fem3dMutation>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    pub(super) cancelled: bool,
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Fem3dSnapshot, Fem3dMutation> for Fem3dArtifactPreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<Fem3dMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Fem3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    /// 🧺️ One forward row plus the inverse rows the leaf's payload schema declares (`x-semio-inverse-rows`: one per node
    /// and solid a `move-selection` restores, one for every other kind).
    fn preflight(&self, mutation: &Fem3dMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("fem3d-artifact-lane".into());
        }
        admit_fem3d_artifact_mutation(mutation)
    }

    fn begin_demand(&self, _mutation: &Fem3dMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "fem3d-artifact-lane"));
        }
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Fem3dArtifactPreparation>(), depth: 1 })
    }
    fn begin_copy_byte_demand(&self,input:&Fem3dMutation,lane:store::os_store::store_schema::HistoryLane)->Result<usize,semio_framework_value::ValueError>{self.begin_demand(input,lane)?; std::mem::size_of::<store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Fem3dSnapshot, Fem3dMutation, Fem3dMutation>>().checked_add(std::mem::size_of::<Fem3dArtifactPreparation>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original preparation header quote overflow"))}


    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Fem3dSnapshot, Fem3dMutation, Fem3dMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Fem3dSnapshot, Fem3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Fem3dSnapshot, Fem3dMutation, Fem3dMutation>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        let retained_bytes = fem3d_artifact_mutation_retained_bytes(&request.mutation).unwrap_or(FEM3D_ARTIFACT_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > FEM3D_ARTIFACT_STORE_MAXIMUM_BYTES
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "fem3d-artifact-request-refused"), request));
        }
        Ok((Box::new(Fem3dArtifactPreparation {
            owners: store::OneItemOwners::from_request(request),
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            cancelled: false,
        }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Fem3dSnapshot, Fem3dMutation> for Fem3dArtifactPreparation {
    fn advance(&mut self, body:usize,grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use protocol::Mutation as _;
        let fault = |message: &'static str| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, message);
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() || self.owners.failure.is_some() {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, semio_framework_value::retained_clone::RetainedCloneProgress::default()));
        }
        let authority = self.owners.authority.as_ref().ok_or_else(|| fault("fem3d-artifact-authority-missing"))?;
        let base = self.owners.base.as_ref().ok_or_else(|| fault("fem3d-artifact-base-owner-missing"))?;
        let mutation = self.owners.mutation.take().ok_or_else(|| fault("fem3d-artifact-mutation-owner-missing"))?;
        let inverse = match mutation.inverse(base.get()) {
            Ok(inverse) => inverse,
            Err(error) => {
                *self.owners.mutation = Some(mutation);
                return Err(error);
            }
        };
        let post = match protocol::apply_diff_cold(mutation.diff(base.get()).diff(), base.get()) {
            Ok(post) => post,
            Err(_) => {
                *self.owners.mutation = Some(mutation);
                *self.owners.inverse = Some(inverse);
                *self.owners.failure = Some(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retained the original mutation application refusal"));
                return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retained the original mutation application refusal"));
            }
        };
        let edit = authority.next_edit(mutation, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Fem3dSnapshot, Fem3dMutation>> {
        self.owners.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Fem3dSnapshot, Fem3dMutation>> {
        self.owners.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.owners.begin_close();
    }
    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.owners.close_step(grant.retained_grant())
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}
//#endregion 📬️ArtifactStorePreparation
