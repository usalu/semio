//! 📬️ Artifact store preparation: the artifact mutation admission, the store preparation factory and the one item publication implementations. See [`super`].

use super::*;

//#region 🔖️Generation3dPlayApp
//#region 📬️ArtifactStorePreparation
fn generation3d_artifact_mutation_retained_bytes(mutation: &Generation3dMutation) -> Result<usize, String> {
    ::protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "generation3d-artifact-mutation-encode-failed".to_string())
}

pub fn admit_generation3d_artifact_mutation(mutation: &Generation3dMutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = generation3d_artifact_mutation_retained_bytes(mutation)?;
    if retained_bytes > GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("generation3d-artifact-mutation-envelope".into());
    }
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
}

/// 🧬️ Raises the mutation's delta, applies it and CLOSES the delta — a `Generation3dDiff` owns the
/// projections it displaces (a `FlowHostSnapshot` whose `layout` is an `OrderedMap` root that rejects a bare
/// drop), so the intermediate delta is retired rather than dropped: leaving it to drop glue aborted the
/// whole store-publication turn the moment a layout entry existed
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The mutation stays with its caller, so a refusal never
/// loses an original owner.
fn prepare_generation3d_artifact(base: &Generation3dSnapshot, mutation: &Generation3dMutation) -> Result<(Generation3dSnapshot, Vec<Generation3dMutation>), semio_framework_value::ValueError> {
    use semio_framework_value::{ValueError, ValueRefusalKind};
    admit_generation3d_artifact_mutation(mutation).map_err(|_| ValueError::literal(ValueRefusalKind::OwnershipLimit, "generation3d-artifact-mutation-envelope"))?;
    let inverse = protocol::ColdMutation::inverse(mutation, base)?;
    let diff = protocol::ColdMutation::diff(mutation, base).into_parts().0;
    let applied = protocol::apply_diff_cold(&diff, base);
    diff.retire_cold();
    match applied {
        Ok(post) => Ok((post, inverse)),
        Err(_) => {
            inverse.into_iter().for_each(|row| protocol::Mutation::<Generation3dSnapshot>::retire_cold(row));
            Err(ValueError::literal(ValueRefusalKind::InvalidValue, "generation3d-artifact-diff-apply-failed"))
        }
    }
}

fn generation3d_preparation_refusal(message: &'static str) -> semio_framework_value::ValueError {
    semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, message)
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct Generation3dArtifactStorePreparationFactory;

struct Generation3dArtifactStorePreparation {
    owners: store::OneItemOwners<Generation3dSnapshot, Generation3dMutation>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Generation3dSnapshot, Generation3dMutation> for Generation3dArtifactStorePreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<Generation3dMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Generation3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &Generation3dMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("generation3d-artifact-lane".into());
        }
        admit_generation3d_artifact_mutation(mutation)
    }

    fn begin_demand(&self, _mutation: &Generation3dMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err(generation3d_preparation_refusal("generation3d-artifact-lane"));
        }
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Generation3dArtifactStorePreparation>(), depth: 1 })
    }

    fn begin_copy_byte_demand(&self, mutation: &Generation3dMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<usize, semio_framework_value::ValueError> {
        self.begin_demand(mutation, lane)?;
        size_of::<store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Generation3dSnapshot, Generation3dMutation, Generation3dMutation>>().checked_add(size_of::<Generation3dArtifactStorePreparation>()).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original preparation header quote overflow"))
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Generation3dSnapshot, Generation3dMutation, Generation3dMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Generation3dSnapshot, Generation3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Generation3dSnapshot, Generation3dMutation, Generation3dMutation>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        let retained_bytes = generation3d_artifact_mutation_retained_bytes(&request.mutation).unwrap_or(GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES
        {
            return Err((generation3d_preparation_refusal("generation3d-artifact-request-refused"), request));
        }
        Ok((Box::new(Generation3dArtifactStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), retained_bytes, cancelled: false }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Generation3dSnapshot, Generation3dMutation> for Generation3dArtifactStorePreparation {
    fn advance(&mut self, _body: usize, grant: store::os_store::one_item::ArtifactStoreOneItemGrant, _receiving: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(generation3d_preparation_refusal("generation3d-artifact-original-refusal-retained"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()));
        }
        if grant.maximum_copy_bytes < self.retained_bytes {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let base = self.owners.base.as_ref().ok_or_else(|| generation3d_preparation_refusal("generation3d-artifact-base-owner-missing"))?;
        let mutation = self.owners.mutation.as_ref().ok_or_else(|| generation3d_preparation_refusal("generation3d-artifact-mutation-owner-missing"))?;
        let (post, inverse) = prepare_generation3d_artifact(base.get(), mutation)?;
        let authority = self.owners.authority.as_ref().ok_or_else(|| generation3d_preparation_refusal("generation3d-artifact-authority-missing"))?;
        let forward = self.owners.mutation.take().ok_or_else(|| generation3d_preparation_refusal("generation3d-artifact-mutation-owner-missing"))?;
        let edit = authority.next_edit(forward, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, copied_bytes: self.retained_bytes, ..Default::default() }))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Generation3dSnapshot, Generation3dMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Generation3dSnapshot, Generation3dMutation>> {
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

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}
//#endregion 📬️ArtifactStorePreparation
//#endregion 🔖️Generation3dPlayApp
