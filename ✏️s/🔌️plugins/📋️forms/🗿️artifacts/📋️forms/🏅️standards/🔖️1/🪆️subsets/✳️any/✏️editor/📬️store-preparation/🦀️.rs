//! 📬️ Forms store preparation: the exact retained-byte accounting, the one-item admission footprint and the store preparation factory.

use super::*;

//#region 📬️StorePreparation
fn forms_store_mutation_retained_bytes<M: protocol::OpBinary>(mutation: &M) -> Result<usize, String> {
    protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "forms-store-mutation-encode-failed".to_string())
}

fn admit_forms_store_mutation<P, M: protocol::OpBinary + protocol::Mutation<P>>(mutation: &M) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = forms_store_mutation_retained_bytes(mutation)?;
    if retained_bytes > FORMS_STORE_MUTATION_MAXIMUM_BYTES {
        return Err("forms-store-mutation-envelope".into());
    }
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf::<P, M>(mutation, retained_bytes))
}

fn prepare_forms_store_mutation<P, M>(base: &P, mutation: &M) -> Result<(P, Vec<M>), String>
where
    M: protocol::Mutation<P> + protocol::OpBinary,
{
    admit_forms_store_mutation::<P, M>(mutation)?;
    let inverse = protocol::Mutation::inverse(mutation, base).map_err(semio_framework_value::ValueError::into_message)?;
    if inverse.iter().any(|step| admit_forms_store_mutation::<P, M>(step).is_err()) {
        return Err("forms-store-inverse-mutation-envelope".into());
    }
    let diff = protocol::Mutation::diff(mutation, base).into_parts().0;
    let post = protocol::apply_diff(&diff, base).map_err(|_| "forms-store-diff-apply-failed".to_string())?;
    Ok((post, inverse))
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct FormsStorePreparationFactory<P, M> {
    prefix: &'static str,
    marker: std::marker::PhantomData<fn() -> (P, M)>,
}

impl<P, M> FormsStorePreparationFactory<P, M> {
    pub(super) fn new(prefix: &'static str) -> Self {
        Self { prefix, marker: std::marker::PhantomData }
    }
}

struct FormsStorePreparation<P: 'static, M: 'static> {
    owners: store::OneItemOwners<P, M>,
    prefix: &'static str,
    pub(super) retained_bytes: usize,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
}

impl<P, M> store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<P, M> for FormsStorePreparationFactory<P, M>
where
    P: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: protocol::Mutation<P> + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + store::ArtifactCanonicalJsonTree + Send + Sync + 'static,
{
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<M>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<M>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> { store::admit_artifact_batch_digest(edit, grant) }

    fn preflight(&self, mutation: &M, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("forms-store-lane".into());
        }
        admit_forms_store_mutation::<P, M>(mutation)
    }

    fn begin_demand(&self, _mutation: &M, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<FormsStorePreparation<P, M>>(), depth: 1 })
    }

    fn begin(&self, request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<P, M, M>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<P, M>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<P, M, M>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        let retained_bytes = forms_store_mutation_retained_bytes(&request.mutation).unwrap_or(FORMS_STORE_MUTATION_MAXIMUM_BYTES.saturating_add(1));
        if request.lane != store::os_store::store_schema::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > FORMS_STORE_MUTATION_MAXIMUM_BYTES
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "preparation rejected original publication authority"), request));
        }
        Ok((Box::new(FormsStorePreparation {
            owners: store::OneItemOwners::from_request(request),
            prefix: self.prefix,
            retained_bytes,
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
        }), progress))
    }
}

impl<P, M> store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<P, M> for FormsStorePreparation<P, M>
where
    P: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: protocol::Mutation<P> + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
{
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()));
        }
        if grant.maximum_copy_bytes < self.retained_bytes {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let base = self.owners.base.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "forms-store-base-owner-missing"))?;
        let mutation = self.owners.mutation.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "forms-store-mutation-owner-missing"))?;
        let (post, inverse) = prepare_forms_store_mutation(base.get(), mutation).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "forms-store-preparation-refused"))?;
        let authority = self.owners.authority.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "forms-store-authority-missing"))?;
        let forward = self.owners.mutation.take().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "forms-store-mutation-owner-missing"))?;
        let prepared = match authority.prepare_one_item(authority.next_edit(forward, inverse), std::sync::Arc::new(post)) {
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
