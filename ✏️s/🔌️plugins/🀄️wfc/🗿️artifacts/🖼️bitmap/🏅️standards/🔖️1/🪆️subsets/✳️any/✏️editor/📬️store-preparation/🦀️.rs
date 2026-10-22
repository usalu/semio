//! 📬️ Store preparation of the WFC bitmap editor: the one-item publication factory and its preparation. See [`super`].

use super::*;

//#region 📬️StorePreparation
/// 🏷️ Id prefix every staged bitmap document edit is minted under.
const BITMAP_STORE_PREFIX: &str = "wfc-bitmap-artifact-retained";

fn bitmap_mutation_retained_bytes(mutation: &BitmapMutation) -> Result<usize, String> {
    protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| format!("{BITMAP_STORE_PREFIX}-mutation-encode-failed"))
}

fn bitmap_one_item_footprint(mutation: &BitmapMutation, maximum_bytes: usize) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = bitmap_mutation_retained_bytes(mutation)?;
    if retained_bytes > maximum_bytes {
        return Err(format!("{BITMAP_STORE_PREFIX}-mutation-envelope"));
    }
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
}

/// 🏭️ The bitmap document lane's own one-item publication authority.
///
/// It is not the SDK's `bounded_config_store_one_item_preparation_factory` for two reasons, both
/// measured live on the playground: that factory declares every mutation point-invertible (above), and
/// it CONSUMES its mutation owner before the fallible steps, so the store's second admission retry
/// reports `…-mutation-owner-missing` and the real first failure is never printed anywhere.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct BitmapOneItemPreparationFactory {
    pub(super) maximum_bytes: usize,
}

struct BitmapOneItemPreparation {
    owners: store::OneItemOwners<BitmapSnapshot, BitmapMutation>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    pub(super) cancelled: bool,
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<BitmapSnapshot, BitmapMutation> for BitmapOneItemPreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<BitmapMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<BitmapMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &BitmapMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err(format!("{BITMAP_STORE_PREFIX}-lane"));
        }
        bitmap_one_item_footprint(mutation, self.maximum_bytes)
    }

    fn begin_demand(&self, _mutation: &BitmapMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "wfc-bitmap-artifact-retained-lane"));
        }
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<BitmapOneItemPreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<BitmapSnapshot, BitmapMutation, BitmapMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<BitmapSnapshot, BitmapMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<BitmapSnapshot, BitmapMutation, BitmapMutation>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        let retained_bytes = bitmap_mutation_retained_bytes(&request.mutation).unwrap_or(self.maximum_bytes.saturating_add(1));
        if request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > self.maximum_bytes
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "wfc-bitmap-artifact-retained-request-refused"), request));
        }
        Ok((Box::new(BitmapOneItemPreparation {
            owners: store::OneItemOwners::from_request(request),
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            cancelled: false,
        }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<BitmapSnapshot, BitmapMutation> for BitmapOneItemPreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
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
        let authority = self.owners.authority.as_ref().ok_or_else(|| fault("wfc-bitmap-artifact-retained-authority-missing"))?;
        let base = self.owners.base.as_ref().ok_or_else(|| fault("wfc-bitmap-artifact-retained-base-owner-missing"))?;
        let mutation = self.owners.mutation.take().ok_or_else(|| fault("wfc-bitmap-artifact-retained-mutation-owner-missing"))?;
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
    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<BitmapSnapshot, BitmapMutation>> {
        self.owners.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<BitmapSnapshot, BitmapMutation>> {
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
//#endregion 📬️StorePreparation
