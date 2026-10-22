//! 🗄️ Artifact store preparation of the CAD play app: the bounded admission of snapshot mutations. See [`super`].

use super::*;

//#region 🔖️PlayApp
//#region 📬️ArtifactStorePreparation
const CAD_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 65_536;

const CAD_ARTIFACT_STORE_MAXIMUM_ITEMS: usize = 512;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct CadArtifactStorePreparationFactory;

struct CadArtifactStorePreparation {
    pub(super) owners: store::OneItemOwners<CadSnapshot, CadMutation>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    pub(super) cancelled: bool,
}

fn cad_child_retained_bytes<S>(child: &store::os_store::artifact_child::ArtifactChild<S>) -> usize {
    child.child_id.len()+child.target.artifact_id.len()+child.target.dialect.artifact_kind.len()+child.target.dialect.standard.len()+child.target.dialect.subset.len()
}

fn cad_snapshot_retained_bytes(snapshot: &CadSnapshot) -> usize {
    let fixed_children = [
        snapshot.shape_model.as_ref().map(cad_child_retained_bytes),
        snapshot.building_model.as_ref().map(cad_child_retained_bytes),
        snapshot.energy_model.as_ref().map(cad_child_retained_bytes),
        snapshot.structure_classic_model.as_ref().map(cad_child_retained_bytes),
    ]
    .into_iter()
    .flatten()
    .fold(0usize, usize::saturating_add);
    let drawing_bytes = snapshot.drawings.iter().map(cad_child_retained_bytes).fold(0usize, usize::saturating_add);
    let reference_bytes = snapshot.references_by_model_definition_id.iter().fold(0usize, |bytes, (model_definition_id, references)| {
        references.iter().fold(bytes.saturating_add(model_definition_id.len()), |bytes, reference| bytes.saturating_add(reference.id.len()).saturating_add(reference.source_url.len()).saturating_add(reference.media_kind.len()))
    });
    let node_bytes = snapshot.nodes.iter().fold(0usize, |bytes, node| bytes.saturating_add(node.id.len()).saturating_add(node.label.len()).saturating_add(node.kind.len()));
    snapshot.schema.len().saturating_add(snapshot.id.len()).saturating_add(fixed_children).saturating_add(drawing_bytes).saturating_add(reference_bytes).saturating_add(node_bytes)
}

fn cad_snapshot_items(snapshot: &CadSnapshot) -> usize {
    snapshot
        .drawings
        .len()
        .saturating_add(snapshot.nodes.len())
        .saturating_add(snapshot.references_by_model_definition_id.values().map(Vec::len).sum::<usize>())
        .saturating_add(snapshot.shape_model.is_some() as usize)
        .saturating_add(snapshot.building_model.is_some() as usize)
        .saturating_add(snapshot.energy_model.is_some() as usize)
        .saturating_add(snapshot.structure_classic_model.is_some() as usize)
}

/// 🧺️ Admits a CAD Artifact against its fixed retained item and byte envelope, answering its retained bytes.
pub(super) fn admit_cad_snapshot(snapshot: &CadSnapshot) -> Result<usize, String> {
    let work_items = cad_snapshot_items(snapshot);
    let retained_bytes = cad_snapshot_retained_bytes(snapshot);
    if work_items > CAD_ARTIFACT_STORE_MAXIMUM_ITEMS || retained_bytes > CAD_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("CAD Artifact exceeds its fixed retained preparation envelope".into());
    }
    Ok(retained_bytes)
}

fn admit_cad_artifact_mutation(mutation: &CadMutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = semio_framework_pack_json::to_json_string(mutation).len();
    if retained_bytes > CAD_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("CAD Artifact mutation exceeds its fixed retained byte envelope".into());
    }
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
}

fn prepare_cad_artifact(base: &CadSnapshot, mutation: &CadMutation) -> Result<(CadSnapshot, Vec<CadMutation>), semio_framework_value::ValueError> {
    admit_cad_snapshot(base).map_err(cad_refusal)?;
    admit_cad_artifact_mutation(mutation).map_err(cad_refusal)?;
    let inverse = <CadMutation as protocol::ColdMutation<CadSnapshot>>::inverse(mutation, base)?;
    let outcome = <CadMutation as protocol::ColdMutation<CadSnapshot>>::diff(mutation, base);
    let post = protocol::apply_diff_cold(outcome.diff(), base).map_err(|error| cad_refusal(error.to_string()))?;
    admit_cad_snapshot(&post).map_err(cad_refusal)?;
    Ok((post, inverse))
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<CadSnapshot, CadMutation> for CadArtifactStorePreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<CadMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<CadMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn operation_wire_source<'a>(&self, operation: &'a CadMutation) -> Option<store::ArtifactPreparedOperationSource<'a>> {
        semio_s_artifact_cad_cad_mutations::standards::v1::subsets::any::io::binary::mutations::prepared_operation_wire_source(operation)
    }

    fn operation_schema_parts<'a>(&'a self, operation: &'a CadMutation) -> Option<(&'a str, &'a str)> {
        let semantics = protocol::SemanticMutation::semantics(operation);
        Some((semantics.entity, semantics.kind))
    }

    fn preflight(&self, mutation: &CadMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("CAD Artifact preparation rejected its lane".into());
        }
        admit_cad_artifact_mutation(mutation)
    }

    fn begin_demand(&self, _mutation: &CadMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<CadArtifactStorePreparation>(), depth: 1 })
    }

    fn begin(&self, request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<CadSnapshot, CadMutation, CadMutation>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<CadSnapshot, CadMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<CadSnapshot, CadMutation, CadMutation>)> {
        if request.lane != store::os_store::store_schema::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "CAD Artifact preparation rejected original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        Ok((Box::new(CadArtifactStorePreparation {
            owners: store::OneItemOwners::from_request(request),
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
        }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<CadSnapshot, CadMutation> for CadArtifactStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || self.cancelled { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.owners.refused.is_some() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal")); }
        if self.owners.prepared.is_some() { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default())); }
        let base = self.owners.base.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "CAD Artifact preparation lost its exact base root"))?;
        let mutation = self.owners.mutation.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "CAD Artifact preparation lost its mutation owner"))?;
        let (post, inverse) = prepare_cad_artifact(base.get(), mutation)?;
        let authority = self.owners.authority.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "CAD Artifact preparation lost its Store authority"))?;
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
    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<CadSnapshot, CadMutation>> { self.owners.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<CadSnapshot, CadMutation>> { self.owners.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.owners.begin_close(); }
    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> { self.owners.close_step(grant.retained_grant()) }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.owners.terminal_is_empty() }
}
//#endregion 📬️ArtifactStorePreparation
//#endregion 🔖️PlayApp
