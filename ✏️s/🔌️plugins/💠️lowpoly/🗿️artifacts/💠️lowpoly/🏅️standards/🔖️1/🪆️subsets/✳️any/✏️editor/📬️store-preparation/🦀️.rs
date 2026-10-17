//! 📬️ Lowpoly store preparation: the exact retained-byte accounting, the one-item admission footprints and the artifact and config store preparation factories.

use super::*;

//#region 📬️StorePreparation
const LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 16 * 1024 * 1024;

pub(super) const LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES: usize = 16_384;

fn lowpoly_paint_layer_retained_bytes(layer: &semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolyPaintLayer) -> usize {
    layer.name.len().saturating_add(layer.blend_mode.len()).saturating_add(layer.pixels.len())
}

/// 🧱️ Exact retained-byte footprint of one persisted object — the id/name/mesh-handle/paint-layers
/// accounting `lowpoly_snapshot_retained_bytes`'s fold used to inline; shared with
/// `lowpoly_artifact_mutation_retained_bytes`'s `CreateObject` arm below, since a `CreateObject`
/// mutation's payload IS one whole `LowpolyObject`.
fn lowpoly_object_retained_bytes(object: &LowpolyObject) -> usize {
    object
        .id
        .len()
        .saturating_add(object.name.len())
        .saturating_add(object.mesh.as_ref().map_or(0, |mesh| mesh.child_id.len().saturating_add(mesh.target.to_uri().len())))
        .saturating_add(object.mesh_content.len())
        .saturating_add(object.paint_layers.iter().fold(0, |bytes, layer| bytes.saturating_add(lowpoly_paint_layer_retained_bytes(layer))))
}

fn lowpoly_snapshot_retained_bytes(snapshot: &LowpolySnapshot) -> usize {
    snapshot.schema.len().saturating_add(snapshot.objects.iter().fold(0, |bytes, object| bytes.saturating_add(lowpoly_object_retained_bytes(object))))
}

/// 📬️ Exact per-variant byte accounting for every one of `LowpolyMutation`'s 21 declared variants —
/// fail-closed BY CONSTRUCTION: this match is exhaustive over the enum (no `_` arm), so a future
/// variant added to `LowpolyMutation` without a matching arm here is a compile error, not a silent
/// runtime admission. `CreateMesh.mesh_workspace` (the whole half-edge mesh JSON) is accounted for at
/// its exact length — `admit_lowpoly_artifact_mutation`'s `LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES` (16
/// MiB) cap below is what keeps that field's admission meaningful, not a separate per-field cap.
fn lowpoly_artifact_mutation_retained_bytes(mutation: &LowpolyMutation) -> Result<usize, String> {
    match mutation {
        LowpolyMutation::CreateObject(payload) => Ok(lowpoly_object_retained_bytes(&payload.object)),
        LowpolyMutation::DeleteObject(payload) => Ok(payload.id.len()),
        LowpolyMutation::ReorderObjects(payload) => Ok(payload.id.len()),
        LowpolyMutation::RenameObject(payload) => Ok(payload.id.len().saturating_add(payload.new_name.len())),
        LowpolyMutation::ChangeObjectSmoothShading(payload) => Ok(payload.id.len()),
        LowpolyMutation::MoveObject(payload) => Ok(payload.id.len()),
        LowpolyMutation::RotateObject(payload) => Ok(payload.id.len()),
        LowpolyMutation::ScaleObject(payload) => Ok(payload.id.len()),
        LowpolyMutation::CreateMesh(payload) => Ok(payload.id.len().saturating_add(payload.child_id.len()).saturating_add(payload.target.to_uri().len()).saturating_add(payload.mesh_workspace.len())),
        LowpolyMutation::DeleteMesh(payload) => Ok(payload.id.len()),
        LowpolyMutation::SetVertexPositions(payload) => Ok(payload.object_id.len().saturating_add(payload.positions.len().saturating_mul(size_of::<semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyVertexPosition>())).saturating_add(payload.channels.iter().map(|channel| channel.name.len().saturating_add(channel.values.len().saturating_mul(size_of::<semio_framework_value::DslValue>())).saturating_add(channel.indices.as_ref().map_or(0, |indices| indices.len().saturating_mul(4)))).fold(0usize, usize::saturating_add))),
        LowpolyMutation::InsertPaintLayer(payload) => Ok(payload.object_id.len().saturating_add(lowpoly_paint_layer_retained_bytes(&payload.layer))),
        LowpolyMutation::RemovePaintLayer(payload) => Ok(payload.object_id.len()),
        LowpolyMutation::RenamePaintLayer(payload) => Ok(payload.object_id.len().saturating_add(payload.new_name.len())),
        LowpolyMutation::ChangePaintLayerVisible(payload) => Ok(payload.object_id.len()),
        LowpolyMutation::ChangePaintLayerOpacity(payload) => Ok(payload.object_id.len()),
        LowpolyMutation::ChangePaintLayerBlendMode(payload) => Ok(payload.object_id.len().saturating_add(payload.new_blend_mode.len())),
        LowpolyMutation::EditPaintLayer(payload) if payload.runs.len() <= LOWPOLY_RETAINED_PAINT_RUNS => {
            Ok(payload.object_id.len().saturating_add(payload.runs.len().saturating_mul(size_of::<semio_s_artifact_lowpoly_lowpoly_model::PixelRun>())).saturating_add(payload.runs.iter().fold(0_usize, |bytes, run| bytes.saturating_add(run.bytes.len()))))
        }
        LowpolyMutation::EditPaintLayer(_) => Err("Lowpoly paint edit exceeds its fixed run envelope".into()),
        LowpolyMutation::ApplyPaintStroke(payload) => Ok(payload.object_id.len().saturating_add(payload.points.len().saturating_mul(size_of::<[f32; 2]>())).saturating_add(16)),
        LowpolyMutation::MoveSelection(payload) => Ok(payload.object_id.len().saturating_add(payload.vertex_ids.len().saturating_mul(4)).saturating_add(12)),
        LowpolyMutation::RotateSelection(payload) => Ok(payload.object_id.len().saturating_add(payload.vertex_ids.len().saturating_mul(4)).saturating_add(28)),
        LowpolyMutation::ScaleSelection(payload) => Ok(payload.object_id.len().saturating_add(payload.vertex_ids.len().saturating_mul(4)).saturating_add(24)),
    }
}

/// 🧺️ One forward row plus the leaf's derived inverse rows. A relative leaf's inverse carries what the base held — a
/// stroke's overwritten pixels, a selection motion's prior mesh content — which preflight never sees, so it declares the
/// lane's whole one-item byte envelope; every other inverse is bounded by its forward twin.
fn admit_lowpoly_artifact_mutation(mutation: &LowpolyMutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = lowpoly_artifact_mutation_retained_bytes(mutation)?;
    if retained_bytes > LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("Lowpoly Artifact mutation exceeds its fixed retained preparation envelope".into());
    }
    let inverse_bytes = match mutation {
        LowpolyMutation::ApplyPaintStroke(_) | LowpolyMutation::MoveSelection(_) | LowpolyMutation::RotateSelection(_) | LowpolyMutation::ScaleSelection(_) => store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES.saturating_sub(retained_bytes),
        _ => retained_bytes,
    };
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes.saturating_add(inverse_bytes)))
}

fn lowpoly_refusal(message: impl Into<String>) -> semio_framework_value::ValueError {
    semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message.into())
}

fn prepare_lowpoly_artifact(base: &LowpolySnapshot, mutation: &LowpolyMutation) -> Result<(LowpolySnapshot, Vec<LowpolyMutation>), semio_framework_value::ValueError> {
    admit_lowpoly_artifact_mutation(mutation).map_err(lowpoly_refusal)?;
    if !lowpoly_snapshot_admitted(base) || lowpoly_snapshot_retained_bytes(base) > LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err(lowpoly_refusal("Lowpoly Artifact base exceeds its fixed retained preparation envelope"));
    }
    let inverse = mutation.inverse(base)?;
    let diff = mutation.diff(base).into_parts().0;
    let post = protocol::apply_diff(&diff, base).map_err(|_| lowpoly_refusal("Lowpoly Artifact preparation could not apply its exact sparse diff"))?;
    if !lowpoly_snapshot_admitted(&post) || lowpoly_snapshot_retained_bytes(&post) > LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err(lowpoly_refusal("Lowpoly Artifact result exceeds its fixed retained preparation envelope"));
    }
    Ok((post, inverse))
}

pub(super) fn lowpoly_config_retained_bytes(config: &LowpolyConfig) -> usize {
    config.active_object_id.len().saturating_add(config.paint_utility.len()).saturating_add(config.utility_params_json.len()).saturating_add(config.engagement_input.len()).saturating_add(config.sun_color.len())
}

fn lowpoly_config_mutation_retained_bytes(mutation: &LowpolyConfigMutation) -> usize {
    match mutation {
        LowpolyConfigMutation::SetActiveObject(SetActiveObjectEdit { object_id }) => object_id.len(),
        LowpolyConfigMutation::SetPaintUtility(SetPaintUtilityEdit { value }) | LowpolyConfigMutation::SetEngagementInput(SetEngagementInputEdit { value }) => value.len(),
        LowpolyConfigMutation::SetUtilityParams(SetUtilityParamsEdit { json }) => json.len(),
        LowpolyConfigMutation::SetSun(SetSunEdit { color, .. }) => color.len(),
        LowpolyConfigMutation::SetActivePaintLayer(_) | LowpolyConfigMutation::SetPaintColor(_) | LowpolyConfigMutation::SetWorldCamera(_) | LowpolyConfigMutation::SetShowEdges(_) => 0,
    }
}

/// 🧺️ One forward row plus its point inverse, whose bytes (for `create-mesh` the prior content too) the forward twin bounds.
fn admit_lowpoly_config_mutation(mutation: &LowpolyConfigMutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = lowpoly_config_mutation_retained_bytes(mutation);
    if retained_bytes > LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Lowpoly config mutation exceeds its fixed retained preparation envelope".into());
    }
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes.saturating_mul(2)))
}

fn prepare_lowpoly_config(base: &LowpolyConfig, mutation: &LowpolyConfigMutation) -> Result<(LowpolyConfig, Vec<LowpolyConfigMutation>), semio_framework_value::ValueError> {
    admit_lowpoly_config_mutation(mutation).map_err(lowpoly_refusal)?;
    if lowpoly_config_retained_bytes(base) > LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES {
        return Err(lowpoly_refusal("Lowpoly config base exceeds its fixed retained preparation envelope"));
    }
    let inverse = mutation.inverse(base)?;
    let post = protocol::apply_diff(&<LowpolyConfigMutation as Mutation<LowpolyConfig>>::diff(mutation, base).into_parts().0, base).map_err(|error| lowpoly_refusal(error.to_string()))?;
    if lowpoly_config_retained_bytes(&post) > LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES {
        return Err(lowpoly_refusal("Lowpoly config result exceeds its fixed retained preparation envelope"));
    }
    Ok((post, inverse))
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct LowpolyArtifactStorePreparationFactory;

struct LowpolyArtifactStorePreparation {
    owners: store::OneItemOwners<LowpolySnapshot, LowpolyMutation>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<LowpolySnapshot, LowpolyMutation> for LowpolyArtifactStorePreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<LowpolyMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<LowpolyMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &LowpolyMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Lowpoly Artifact preparation rejected its lane".into());
        }
        admit_lowpoly_artifact_mutation(mutation)
    }

    fn begin_demand(&self, _mutation: &LowpolyMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<LowpolyArtifactStorePreparation>(), depth: 1 })
    }

    fn begin(&self, request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<LowpolySnapshot, LowpolyMutation, LowpolyMutation>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<LowpolySnapshot, LowpolyMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<LowpolySnapshot, LowpolyMutation, LowpolyMutation>)> {
        if request.lane != store::os_store::store_schema::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES || lowpoly_artifact_mutation_retained_bytes(&request.mutation).map_or(true, |bytes| bytes > LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES) || !lowpoly_snapshot_admitted(request.base.get()) {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Lowpoly Artifact preparation rejected original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        Ok((Box::new(LowpolyArtifactStorePreparation {
            owners: store::OneItemOwners::from_request(request),
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
        }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<LowpolySnapshot, LowpolyMutation> for LowpolyArtifactStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || self.cancelled { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.owners.refused.is_some() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal")); }
        if self.owners.prepared.is_some() { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default())); }
        let base = self.owners.base.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Lowpoly Artifact preparation lost its exact base root"))?;
        let mutation = self.owners.mutation.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Lowpoly Artifact preparation lost its mutation owner"))?;
        let retained_bytes = lowpoly_artifact_mutation_retained_bytes(mutation).unwrap_or(0);
        let (post, inverse) = prepare_lowpoly_artifact(base.get(), mutation)?;
        let authority = self.owners.authority.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Lowpoly Artifact preparation lost its Store authority"))?;
        let forward = self.owners.mutation.take().expect("observed original mutation owner");
        let edit = authority.next_edit(forward, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => { *self.owners.refused = Some((edit, post)); return Err(error); }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: retained_bytes as u64, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<LowpolySnapshot, LowpolyMutation>> { self.owners.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<LowpolySnapshot, LowpolyMutation>> { self.owners.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.owners.begin_close(); }
    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> { self.owners.close_step(grant.retained_grant()) }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.owners.terminal_is_empty() }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct LowpolyConfigStorePreparationFactory;

struct LowpolyConfigStorePreparation {
    owners: store::OneItemOwners<LowpolyConfig, LowpolyConfigMutation>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<LowpolyConfig, LowpolyConfigMutation> for LowpolyConfigStorePreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<LowpolyConfigMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<LowpolyConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &LowpolyConfigMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Lowpoly config preparation rejected its lane".into());
        }
        admit_lowpoly_config_mutation(mutation)
    }

    fn begin_demand(&self, _mutation: &LowpolyConfigMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<LowpolyConfigStorePreparation>(), depth: 1 })
    }

    fn begin(&self, request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<LowpolyConfig, LowpolyConfigMutation, LowpolyConfigMutation>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<LowpolyConfig, LowpolyConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<LowpolyConfig, LowpolyConfigMutation, LowpolyConfigMutation>)> {
        if request.lane != store::os_store::store_schema::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES || lowpoly_config_mutation_retained_bytes(&request.mutation) > LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES || lowpoly_config_retained_bytes(request.base.get()) > LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Lowpoly config preparation rejected original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        Ok((Box::new(LowpolyConfigStorePreparation {
            owners: store::OneItemOwners::from_request(request),
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
        }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<LowpolyConfig, LowpolyConfigMutation> for LowpolyConfigStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || self.cancelled { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.owners.refused.is_some() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal")); }
        if self.owners.prepared.is_some() { return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default())); }
        let base = self.owners.base.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Lowpoly config preparation lost its exact base root"))?;
        let mutation = self.owners.mutation.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Lowpoly config preparation lost its mutation owner"))?;
        let retained_bytes = lowpoly_config_mutation_retained_bytes(mutation);
        let (post, inverse) = prepare_lowpoly_config(base.get(), mutation)?;
        let authority = self.owners.authority.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Lowpoly config preparation lost its Store authority"))?;
        let forward = self.owners.mutation.take().expect("observed original mutation owner");
        let edit = authority.next_edit(forward, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => { *self.owners.refused = Some((edit, post)); return Err(error); }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: retained_bytes as u64, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<LowpolyConfig, LowpolyConfigMutation>> { self.owners.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<LowpolyConfig, LowpolyConfigMutation>> { self.owners.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.owners.begin_close(); }
    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> { self.owners.close_step(grant.retained_grant()) }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.owners.terminal_is_empty() }
}
//#endregion 📬️StorePreparation
