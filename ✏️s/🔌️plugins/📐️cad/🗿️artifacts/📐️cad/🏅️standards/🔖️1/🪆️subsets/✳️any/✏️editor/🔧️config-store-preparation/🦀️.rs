//! 🔧️ Config store preparation of the CAD play app: the bounded admission of config mutations. See [`super`].

use super::*;

//#region 🔖️PlayApp
//#region 📬️ConfigStorePreparation
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct CadConfigStorePreparationFactory;

struct CadConfigStorePreparation {
    pub(super) owners: store::OneItemOwners<CadConfig, CadConfigMutation>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    pub(super) cancelled: bool,
}

fn cad_config_retained_bytes(config: &CadConfig) -> usize {
    let option_bytes = [
        config.hovered_reference_id.as_deref(),
        config.active_example_id.as_deref(),
        config.selected_reference_model_definition_id.as_deref(),
        config.selected_reference_id.as_deref(),
    ]
    .into_iter()
    .flatten()
    .fold(0usize, |bytes, value| bytes.saturating_add(value.len()));
    config
        .selected_node_ids
        .iter()
        .fold(0usize, |bytes, value| bytes.saturating_add(value.len()))
        .saturating_add(option_bytes)
        .saturating_add(config.contributions_json.len())
}

/// 🧺️ Admits a CAD config against its fixed retained item and byte envelope, answering its retained bytes.
pub(super) fn admit_cad_config(config: &CadConfig) -> Result<usize, String> {
    if config.selected_node_ids.len() > CAD_CONFIG_STORE_MAXIMUM_ITEMS {
        return Err("CAD config exceeds its fixed retained item envelope".into());
    }
    let retained_bytes = cad_config_retained_bytes(config);
    if retained_bytes > CAD_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("CAD config exceeds its fixed retained byte envelope".into());
    }
    Ok(retained_bytes)
}

fn admit_cad_config_mutation(mutation: &CadConfigMutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = match mutation {
        CadConfigMutation::Set(SetEdit { config }) => admit_cad_config(config)?,
        CadConfigMutation::SetContributions(SetContributionsEdit { json }) if json.len() <= CAD_CONFIG_STORE_MAXIMUM_BYTES => json.len(),
        CadConfigMutation::SetContributions(_) => return Err("CAD config mutation exceeds its fixed retained byte envelope".into()),
    };
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
}

pub(super) fn cad_refusal(message: impl Into<String>) -> semio_framework_value::ValueError {
    semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message.into())
}

fn prepare_cad_config(base: &CadConfig, mutation: &CadConfigMutation) -> Result<(CadConfig, Vec<CadConfigMutation>), semio_framework_value::ValueError> {
    admit_cad_config(base).map_err(cad_refusal)?;
    admit_cad_config_mutation(mutation).map_err(cad_refusal)?;
    let inverse = <CadConfigMutation as protocol::Mutation<CadConfig>>::inverse(mutation, base)?;
    let post = protocol::apply_diff(&<CadConfigMutation as protocol::Mutation<CadConfig>>::diff(mutation, base).into_parts().0, base).map_err(|error| cad_refusal(error.to_string()))?;
    admit_cad_config(&post).map_err(cad_refusal)?;
    Ok((post, inverse))
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<CadConfig, CadConfigMutation> for CadConfigStorePreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<CadConfigMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<CadConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &CadConfigMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("CAD config preparation rejected its lane".into());
        }
        admit_cad_config_mutation(mutation)
    }

    fn begin_demand(&self, _mutation: &CadConfigMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<CadConfigStorePreparation>(), depth: 1 })
    }

    fn begin(&self, request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<CadConfig, CadConfigMutation, CadConfigMutation>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<CadConfig, CadConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<CadConfig, CadConfigMutation, CadConfigMutation>)> {
        if request.lane != store::os_store::store_schema::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "CAD config preparation rejected original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        Ok((Box::new(CadConfigStorePreparation {
            owners: store::OneItemOwners::from_request(request),
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
        }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<CadConfig, CadConfigMutation> for CadConfigStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || self.cancelled { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.owners.refused.is_some() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal")); }
        if self.owners.prepared.is_some() { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default())); }
        let base = self.owners.base.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "CAD config preparation lost its exact base root"))?;
        let mutation = self.owners.mutation.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "CAD config preparation lost its mutation owner"))?;
        let (post, inverse) = prepare_cad_config(base.get(), mutation)?;
        let authority = self.owners.authority.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "CAD config preparation lost its Store authority"))?;
        let forward = self.owners.mutation.take().expect("observed original mutation owner");
        let edit = authority.next_edit(forward, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => { *self.owners.refused = Some((edit, post)); return Err(error); }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<CadConfig, CadConfigMutation>> { self.owners.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<CadConfig, CadConfigMutation>> { self.owners.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.owners.begin_close(); }
    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> { self.owners.close_step(grant.retained_grant()) }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.owners.terminal_is_empty() }
}
//#endregion 📬️ConfigStorePreparation
//#endregion 🔖️PlayApp
