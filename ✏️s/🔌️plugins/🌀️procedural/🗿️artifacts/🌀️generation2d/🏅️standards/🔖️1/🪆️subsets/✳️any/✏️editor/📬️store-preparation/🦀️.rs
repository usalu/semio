//! 📬️ Store preparation of the generation2d editor: the artifact and config one-item publication factories with their admission and preparation. See [`super`].

use super::*;

//#region 📬️ArtifactStorePreparation
const GENERATION2D_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 262_144;

fn generation2d_artifact_mutation_retained_bytes(mutation: &Generation2dMutation) -> Result<usize, String> {
    ::protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "generation2d-artifact-mutation-encode-failed".to_string())
}

fn admit_generation2d_artifact_mutation(mutation: &Generation2dMutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = generation2d_artifact_mutation_retained_bytes(mutation)?;
    if retained_bytes > GENERATION2D_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("generation2d-artifact-mutation-envelope".into());
    }
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
}

/// 🧬️ Raises the mutation's delta, applies it and CLOSES the delta — a `Generation2dDiff` owns the
/// projections it displaces, so the intermediate delta is retired rather than dropped. The mutation stays
/// with its caller, so a refusal never loses an original owner.
fn prepare_generation2d_artifact(base: &Generation2dSnapshot, mutation: &Generation2dMutation) -> Result<(Generation2dSnapshot, Vec<Generation2dMutation>), semio_framework_value::ValueError> {
    use semio_framework_value::{ValueError, ValueRefusalKind};
    admit_generation2d_artifact_mutation(mutation).map_err(|_| ValueError::literal(ValueRefusalKind::OwnershipLimit, "generation2d-artifact-mutation-envelope"))?;
    let inverse = protocol::Mutation::inverse(mutation, base)?;
    let diff = protocol::Mutation::diff(mutation, base).into_parts().0;
    let applied = protocol::apply_diff_cold(&diff, base);
    diff.retire_cold();
    match applied {
        Ok(post) => Ok((post, inverse)),
        Err(_) => {
            inverse.into_iter().for_each(semio_s_artifact_procedural_2d_mutations::central_apply::generation2d_retire_mutation_cold);
            Err(ValueError::literal(ValueRefusalKind::InvalidValue, "generation2d-artifact-diff-apply-failed"))
        }
    }
}

fn generation2d_preparation_refusal(message: &'static str) -> semio_framework_value::ValueError {
    semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, message)
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct Generation2dArtifactStorePreparationFactory;

struct Generation2dArtifactStorePreparation {
    pub(super) owners: store::OneItemOwners<Generation2dSnapshot, Generation2dMutation>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    pub(super) retained_bytes: usize,
    pub(super) cancelled: bool,
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Generation2dSnapshot, Generation2dMutation> for Generation2dArtifactStorePreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<Generation2dMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Generation2dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &Generation2dMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("generation2d-artifact-lane".into());
        }
        admit_generation2d_artifact_mutation(mutation)
    }

    fn begin_demand(&self, _mutation: &Generation2dMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err(generation2d_preparation_refusal("generation2d-artifact-lane"));
        }
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Generation2dArtifactStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Generation2dSnapshot, Generation2dMutation, Generation2dMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Generation2dSnapshot, Generation2dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Generation2dSnapshot, Generation2dMutation, Generation2dMutation>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        let retained_bytes = generation2d_artifact_mutation_retained_bytes(&request.mutation).unwrap_or(GENERATION2D_ARTIFACT_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > GENERATION2D_ARTIFACT_STORE_MAXIMUM_BYTES
        {
            return Err((generation2d_preparation_refusal("generation2d-artifact-request-refused"), request));
        }
        Ok((Box::new(Generation2dArtifactStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), retained_bytes, cancelled: false }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Generation2dSnapshot, Generation2dMutation> for Generation2dArtifactStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(generation2d_preparation_refusal("generation2d-artifact-original-refusal-retained"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()));
        }
        if grant.maximum_copy_bytes < self.retained_bytes {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let base = self.owners.base.as_ref().ok_or_else(|| generation2d_preparation_refusal("generation2d-artifact-base-owner-missing"))?;
        let mutation = self.owners.mutation.as_ref().ok_or_else(|| generation2d_preparation_refusal("generation2d-artifact-mutation-owner-missing"))?;
        let (post, inverse) = prepare_generation2d_artifact(base.get(), mutation)?;
        let authority = self.owners.authority.as_ref().ok_or_else(|| generation2d_preparation_refusal("generation2d-artifact-authority-missing"))?;
        let forward = self.owners.mutation.take().ok_or_else(|| generation2d_preparation_refusal("generation2d-artifact-mutation-owner-missing"))?;
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

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Generation2dSnapshot, Generation2dMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Generation2dSnapshot, Generation2dMutation>> {
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

//#region 📬️ConfigStorePreparation
pub(super) const GENERATION2D_CONFIG_TEXT_MAXIMUM_BYTES: usize = 128;

const GENERATION2D_CONFIG_PUBLICATION_MAXIMUM_BYTES: usize = 4_096;

//#region 🎟️Admission
fn generation2d_config_text_bytes(config: &Generation2dConfig) -> usize {
    [config.show_mode.len(), config.selected_generation_id.as_ref().map_or(0, String::len)].into_iter().fold(0usize, usize::saturating_add)
}

fn generation2d_config_publication_bytes(mutation: &Generation2dConfigMutation) -> Result<usize, String> {
    let bytes = match mutation {
        Generation2dConfigMutation::SetShowMode(SetShowModeSetting { value }) => value.len(),
        Generation2dConfigMutation::SetSelectedGeneration(SetSelectedGenerationSetting { selected_generation_id }) => selected_generation_id.as_ref().map_or(0, String::len),
    };
    if bytes > GENERATION2D_CONFIG_TEXT_MAXIMUM_BYTES {
        return Err("generation2d-config-text-envelope".into());
    }
    Ok(GENERATION2D_CONFIG_PUBLICATION_MAXIMUM_BYTES)
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct Generation2dConfigPreparationFactory;

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Generation2dConfig, Generation2dConfigMutation> for Generation2dConfigPreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<Generation2dConfigMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Generation2dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &Generation2dConfigMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("generation2d-config-lane".into());
        }
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, generation2d_config_publication_bytes(mutation)?))
    }

    fn begin_demand(&self, _mutation: &Generation2dConfigMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err(generation2d_preparation_refusal("generation2d-config-lane"));
        }
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Generation2dConfigPreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Generation2dConfig, Generation2dConfigMutation, Generation2dConfigMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Generation2dConfig, Generation2dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Generation2dConfig, Generation2dConfigMutation, Generation2dConfigMutation>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        if request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > 64
            || self.preflight(&request.mutation, request.lane).is_err()
            || generation2d_config_text_bytes(request.base.get()) > GENERATION2D_CONFIG_TEXT_MAXIMUM_BYTES
        {
            return Err((generation2d_preparation_refusal("generation2d-config-request-refused"), request));
        }
        Ok((Box::new(Generation2dConfigPreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), cancelled: false }), progress))
    }
}
//#endregion 🎟️Admission

//#region 🧵️Preparation
pub(super) struct Generation2dConfigPreparation {
    pub(super) owners: store::OneItemOwners<Generation2dConfig, Generation2dConfigMutation>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    pub(super) cancelled: bool,
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Generation2dConfig, Generation2dConfigMutation> for Generation2dConfigPreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || grant.maximum_copy_bytes < GENERATION2D_CONFIG_PUBLICATION_MAXIMUM_BYTES || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(generation2d_preparation_refusal("generation2d-config-original-refusal-retained"));
        }
        if self.checkpoint.cursor != 0 {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()));
        }
        let base = self.owners.base.as_ref().ok_or_else(|| generation2d_preparation_refusal("generation2d-config-base-owner-missing"))?;
        let mutation = self.owners.mutation.as_ref().ok_or_else(|| generation2d_preparation_refusal("generation2d-config-mutation-owner-missing"))?;
        let mut next = base.get().clone();
        let inverse = match mutation {
            Generation2dConfigMutation::SetShowMode(setting) => {
                next.show_mode = setting.value.clone();
                Generation2dConfigMutation::SetShowMode(SetShowModeSetting { value: base.get().show_mode.clone() })
            }
            Generation2dConfigMutation::SetSelectedGeneration(setting) => {
                next.selected_generation_id.clone_from(&setting.selected_generation_id);
                Generation2dConfigMutation::SetSelectedGeneration(SetSelectedGenerationSetting { selected_generation_id: base.get().selected_generation_id.clone() })
            }
        };
        if generation2d_config_text_bytes(&next) > GENERATION2D_CONFIG_TEXT_MAXIMUM_BYTES {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "generation2d-config-post-text-envelope"));
        }
        let authority = self.owners.authority.as_ref().ok_or_else(|| generation2d_preparation_refusal("generation2d-config-authority-missing"))?;
        let edit = authority.next_edit(mutation.clone(), vec![inverse]);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(next)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: GENERATION2D_CONFIG_PUBLICATION_MAXIMUM_BYTES as u64, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, copied_bytes: GENERATION2D_CONFIG_PUBLICATION_MAXIMUM_BYTES, ..Default::default() }))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Generation2dConfig, Generation2dConfigMutation>> {
        self.owners.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Generation2dConfig, Generation2dConfigMutation>> {
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
//#endregion 🧵️Preparation
//#endregion 📬️ConfigStorePreparation
