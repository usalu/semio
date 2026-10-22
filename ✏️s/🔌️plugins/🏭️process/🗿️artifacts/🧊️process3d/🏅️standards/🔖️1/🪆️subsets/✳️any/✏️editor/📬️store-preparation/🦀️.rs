//! 📬️ Process3d config and artifact store preparation: retained byte measures, grants and the one-item factories.

use super::*;

//#region 📬️ConfigStorePreparation
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct Process3dConfigStorePreparationFactory;

struct Process3dConfigStorePreparation {
    owners: store::OneItemOwners<Process3dConfig, Process3dConfigMutation>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
}

fn process3d_config_retained_bytes(config: &Process3dConfig) -> Option<usize> {
    [config.engagement_input.len(), config.sun_color.len(), config.contributions_json.len()].into_iter().try_fold(0usize, usize::checked_add)
}

fn process3d_config_mutation_retained_bytes(mutation: &Process3dConfigMutation) -> usize {
    match mutation {
        Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ value }) => value.len(),
        Process3dConfigMutation::SetSun(Process3dConfigSetSun{ color, .. }) => color.len(),
        Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ json }) => json.len(),
        Process3dConfigMutation::SetCamera(Process3dConfigSetCamera{ .. }) | Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ .. }) => 0,
    }
}

fn admit_process3d_config_mutation(mutation: &Process3dConfigMutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = process3d_config_mutation_retained_bytes(mutation);
    let envelope = match mutation {
        Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ .. }) => PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES,
        _ => PROCESS3D_RETAINED_RAW_BYTES,
    };
    if retained_bytes > envelope {
        return Err("Process3d config mutation exceeds its fixed retained preparation envelope".into());
    }
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
}

fn prepare_process3d_config(base: &Process3dConfig, mutation: &Process3dConfigMutation) -> Result<(Process3dConfig, Vec<Process3dConfigMutation>), String> {
    admit_process3d_config_mutation(mutation)?;
    if process3d_config_retained_bytes(base).is_none_or(|bytes| bytes > PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES) {
        return Err("Process3d config base exceeds its fixed retained preparation envelope".into());
    }
    let inverse = match mutation {
        Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ .. }) => Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ value: base.engagement_input.clone() }),
        Process3dConfigMutation::SetCamera(Process3dConfigSetCamera{ .. }) => Process3dConfigMutation::SetCamera(Process3dConfigSetCamera{ position: base.camera_position, target: base.camera_target, fov: base.camera_fov }),
        Process3dConfigMutation::SetSun(Process3dConfigSetSun{ .. }) => Process3dConfigMutation::SetSun(Process3dConfigSetSun{ enabled: base.sun_enabled, azimuth: base.sun_azimuth, elevation: base.sun_elevation, intensity: base.sun_intensity, color: base.sun_color.clone() }),
        Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ .. }) => Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ json: base.contributions_json.clone() }),
        Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ .. }) => Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ value: base.resolved_up_to }),
    };
    let mut post = base.clone();
    match mutation {
        Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ value }) => post.engagement_input = value.clone(),
        Process3dConfigMutation::SetCamera(Process3dConfigSetCamera{ position, target, fov }) => {
            post.camera_position = *position;
            post.camera_target = *target;
            post.camera_fov = *fov;
        }
        Process3dConfigMutation::SetSun(Process3dConfigSetSun{ enabled, azimuth, elevation, intensity, color }) => {
            post.sun_enabled = *enabled;
            post.sun_azimuth = *azimuth;
            post.sun_elevation = *elevation;
            post.sun_intensity = *intensity;
            post.sun_color = color.clone();
        }
        Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ json }) => post.contributions_json = json.clone(),
        Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ value }) => post.resolved_up_to = *value,
    }
    if process3d_config_retained_bytes(&post).is_none_or(|bytes| bytes > PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES) {
        return Err("Process3d config post-state exceeds its fixed retained preparation envelope".into());
    }
    Ok((post, vec![inverse]))
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Process3dConfig, Process3dConfigMutation> for Process3dConfigStorePreparationFactory {
    fn begin_batch_digest(
        &self,
        edit: &mut Option<Box<protocol::Edit<Process3dConfigMutation>>>,
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Process3dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &Process3dConfigMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Process3d config preparation rejected its lane".into());
        }
        admit_process3d_config_mutation(mutation)
    }

    fn begin_demand(&self, _mutation: &Process3dConfigMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Process3dConfigStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Process3dConfig, Process3dConfigMutation, Process3dConfigMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<
        (Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Process3dConfig, Process3dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress),
        (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Process3dConfig, Process3dConfigMutation, Process3dConfigMutation>),
    > {
        if request.lane != store::os_store::store_schema::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d config preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(Process3dConfigStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), cancelled: false }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Process3dConfig, Process3dConfigMutation> for Process3dConfigStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use semio_framework_value::{retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if let Some(error) = self.owners.failure.as_ref() {
            return Err(error.clone());
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Process3d config preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Process3d config preparation lost its exact base root"))?;
        let mutation = self.owners.mutation.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Process3d config preparation lost its mutation owner"))?;
        let (post, inverse) = match prepare_process3d_config(base.get(), &mutation) {
            Ok(candidate) => candidate,
            Err(message) => {
                *self.owners.mutation = Some(mutation);
                *self.owners.failure = Some(ValueError::new(ValueRefusalKind::OwnershipLimit, message.clone()));
                return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, message));
            }
        };
        let Some(authority) = self.owners.authority.as_ref() else {
            *self.owners.mutation = Some(mutation);
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Process3d config preparation lost its Store authority"));
        };
        let edit = authority.next_edit(mutation, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Process3dConfig, Process3dConfigMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Process3dConfig, Process3dConfigMutation>> {
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
//#endregion 📬️ConfigStorePreparation

//#region 📬️ArtifactStorePreparation
/// 🧺️ Fixed envelope for the process3d document's retained one-item publication lane. The document's
/// only unbounded axes are the durable `step_payloads` timeline, its re-minted `tool_solids` handles,
/// and the workshop's machines with their capability leaves — each counted here rather than encoded.
const PROCESS3D_DOCUMENT_MAXIMUM_ITEMS: usize = 4_096;
/// 🔤️ Longest single id, label, handle or recipe parameter name the retained lane admits, in either
/// the base or a mutation payload.
const PROCESS3D_DOCUMENT_TEXT_BYTES: usize = 256;
const PROCESS3D_DOCUMENT_MAXIMUM_BYTES: usize = 512 * 1_024;
/// 🎟️ What one `advance`/`close_step` turn costs, and the ONLY figure the grant is ever compared
/// against. The host drives this lane with a fixed `ArtifactStoreOneItemGrant { maximum_items: 1,
/// maximum_copy_bytes: TYPED_OPERATION_RESULT_PAGE_BYTES }` (4 KiB), so a gate that scaled with the
/// document — `grant.maximum_copy_bytes < measured_base_bytes` — would go `Blocked` forever the moment a
/// timeline or a workshop outgrew one page, stalling the operation instead of failing it. The base's
/// own size is a VALIDATION (`process3d_document_bytes`, rejected past
/// `PROCESS3D_DOCUMENT_MAXIMUM_BYTES`), never the gate.
const PROCESS3D_DOCUMENT_GRANT_BYTES: usize = 4_096;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct Process3dArtifactPreparationFactory;

struct Process3dArtifactPreparation {
    owners: store::OneItemOwners<Process3dSnapshot, Process3dMutation>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
}

/// 📏️ One text field's own retained cost — rejected rather than truncated past the fixed envelope.
fn process3d_text_bytes(value: &str) -> Result<usize, semio_framework_value::ValueError> {
    if value.len() > PROCESS3D_DOCUMENT_TEXT_BYTES {
        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Process3d document carries a text field beyond its encoded text envelope"));
    }
    Ok(value.len())
}

fn process3d_child_bytes<S>(child: &store::os_store::artifact_child::ArtifactChild<S>) -> Result<usize, semio_framework_value::ValueError> {
    Ok(process3d_text_bytes(&child.child_id)?.saturating_add(process3d_text_bytes(&child.target.to_uri())?))
}

fn process3d_solid_bytes(solid: &WorkingSolid) -> Result<usize, semio_framework_value::ValueError> {
    match solid {
        WorkingSolid::Box { .. } | WorkingSolid::Cylinder { .. } | WorkingSolid::Sphere { .. } => Ok(0),
        WorkingSolid::ImportedMesh { mesh_url } => process3d_text_bytes(mesh_url),
        WorkingSolid::ImportedSolid { solid_handle } => process3d_text_bytes(solid_handle),
        WorkingSolid::Reference { reference_id } => process3d_text_bytes(reference_id),
    }
}

fn process3d_measure_bytes(measure: &ProcessMeasure) -> Result<usize, semio_framework_value::ValueError> {
    match measure {
        ProcessMeasure::Cut { tool, .. } => process3d_solid_bytes(tool),
        ProcessMeasure::Attach { component, .. } => process3d_solid_bytes(component),
        ProcessMeasure::Drill { .. } => Ok(0),
    }
}

fn process3d_origin_bytes(origin: Option<&StepOrigin>) -> Result<usize, semio_framework_value::ValueError> {
    match origin {
        Some(origin) => Ok(process3d_text_bytes(&origin.machine_id)?.saturating_add(process3d_text_bytes(&origin.capability_id)?)),
        None => Ok(0),
    }
}

fn process3d_step_bytes(step: &ProcessStep) -> Result<usize, semio_framework_value::ValueError> {
    Ok(process3d_text_bytes(&step.id)?.saturating_add(process3d_text_bytes(&step.label)?).saturating_add(process3d_origin_bytes(step.origin.as_ref())?).saturating_add(process3d_measure_bytes(&step.measure)?).saturating_add(size_of::<ProcessStep>()))
}

fn process3d_recipe_bytes(recipe: &MeasureRecipe) -> Result<usize, semio_framework_value::ValueError> {
    let parts: [&str; 3] = match recipe {
        MeasureRecipe::DiscCut { diameter, kerf } => [diameter, kerf, ""],
        MeasureRecipe::BladeCut { kerf, length, depth } => [kerf, length, depth],
        MeasureRecipe::PocketCut { diameter, depth } => [diameter, depth, ""],
        MeasureRecipe::BoreDrill { radius, depth } => [radius, depth, ""],
        MeasureRecipe::CylinderAttach { radius, length } => [radius, length, ""],
        MeasureRecipe::BoxAttach { width, depth, height } => [width, depth, height],
    };
    parts.iter().try_fold(0usize, |bytes, part| -> Result<usize, semio_framework_value::ValueError> { Ok(bytes.saturating_add(process3d_text_bytes(part)?)) })
}

fn process3d_capability_bytes(capability: &Capability) -> Result<usize, semio_framework_value::ValueError> {
    let parameters = capability.parameters.iter().try_fold(0usize, |bytes, parameter| -> Result<usize, semio_framework_value::ValueError> { Ok(bytes.saturating_add(process3d_text_bytes(&parameter.id)?).saturating_add(process3d_text_bytes(&parameter.label)?)) })?;
    let rules = capability.rules.iter().try_fold(0usize, |bytes, rule| -> Result<usize, semio_framework_value::ValueError> {
        let (CapabilityRule::Min { parameter, .. } | CapabilityRule::Max { parameter, .. }) = rule;
        Ok(bytes.saturating_add(process3d_text_bytes(parameter)?))
    })?;
    Ok(process3d_text_bytes(&capability.id)?
        .saturating_add(process3d_text_bytes(&capability.label)?)
        .saturating_add(process3d_text_bytes(&capability.icon_id)?)
        .saturating_add(process3d_recipe_bytes(&capability.recipe)?)
        .saturating_add(parameters)
        .saturating_add(rules)
        .saturating_add(size_of::<Capability>()))
}

fn process3d_capability_items(capability: &Capability) -> usize {
    1usize.saturating_add(capability.parameters.len()).saturating_add(capability.rules.len())
}

/// 📏️ One capability set costs its owner plus one work item per capability leaf it carries.
fn process3d_capabilities_items(capabilities: &[Capability]) -> usize {
    capabilities.iter().fold(1usize, |items, capability| items.saturating_add(process3d_capability_items(capability)))
}

fn process3d_capabilities_bytes(capabilities: &[Capability]) -> Result<usize, semio_framework_value::ValueError> {
    capabilities.iter().try_fold(0usize, |bytes, capability| -> Result<usize, semio_framework_value::ValueError> { Ok(bytes.saturating_add(process3d_capability_bytes(capability)?)) })
}

fn process3d_machine_bytes(machine: &WorkshopMachine) -> Result<usize, semio_framework_value::ValueError> {
    Ok(process3d_text_bytes(&machine.id)?
        .saturating_add(process3d_text_bytes(&machine.label)?)
        .saturating_add(process3d_text_bytes(&machine.icon_id)?)
        .saturating_add(machine.catalog_id.as_deref().map_or(Ok(0), process3d_text_bytes)?)
        .saturating_add(process3d_capabilities_bytes(&machine.capabilities)?)
        .saturating_add(size_of::<WorkshopMachine>()))
}

fn process3d_machine_items(machine: &WorkshopMachine) -> usize {
    process3d_capabilities_items(&machine.capabilities)
}

fn process3d_stock_bytes(stock: &Stock) -> Result<usize, semio_framework_value::ValueError> {
    Ok(process3d_text_bytes(&stock.id)?.saturating_add(process3d_text_bytes(&stock.label)?).saturating_add(process3d_solid_bytes(&stock.solid)?))
}

/// 📏️ The retained footprint of one document base — rejected rather than truncated when the timeline,
/// the workshop, or any single id/label/handle outgrows the fixed envelope.
fn process3d_document_bytes(document: &Process3dSnapshot) -> Result<usize, semio_framework_value::ValueError> {
    let items = document.step_payloads.len().saturating_add(document.tool_solids.len()).saturating_add(document.workshop.machines.iter().fold(0usize, |items, machine| items.saturating_add(process3d_machine_items(machine))));
    if items > PROCESS3D_DOCUMENT_MAXIMUM_ITEMS {
        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit, "Process3d document base exceeds its retained item envelope"));
    }
    let steps = document.step_payloads.iter().try_fold(0usize, |bytes, step| -> Result<usize, semio_framework_value::ValueError> { Ok(bytes.saturating_add(process3d_step_bytes(step)?)) })?;
    let machines = document.workshop.machines.iter().try_fold(0usize, |bytes, machine| -> Result<usize, semio_framework_value::ValueError> { Ok(bytes.saturating_add(process3d_machine_bytes(machine)?)) })?;
    let tools = document.tool_solids.iter().try_fold(0usize, |bytes, tool| -> Result<usize, semio_framework_value::ValueError> { Ok(bytes.saturating_add(process3d_child_bytes(tool)?)) })?;
    let bytes = process3d_text_bytes(&document.stock_id)?
        .saturating_add(process3d_text_bytes(&document.stock_label)?)
        .saturating_add(process3d_stock_bytes(&document.stock_payload)?)
        .saturating_add(process3d_child_bytes(&document.stock_solid)?)
        .saturating_add(process3d_child_bytes(&document.steps)?)
        .saturating_add(steps)
        .saturating_add(machines)
        .saturating_add(tools)
        .saturating_add(size_of::<Process3dSnapshot>());
    if bytes > PROCESS3D_DOCUMENT_MAXIMUM_BYTES {
        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Process3d document base exceeds its retained byte envelope"));
    }
    Ok(bytes)
}

/// 📏️ One semantic mutation's own retained bytes, shaped like what it actually addresses: a single step, machine,
/// stock field or cursor is one item carrying that target's own text, while a created machine or a replaced capability
/// set is one item per capability leaf it carries — the items bounded by `PROCESS3D_DOCUMENT_MAXIMUM_ITEMS`.
fn process3d_mutation_retained_bytes(mutation: &Process3dMutation) -> Result<usize, semio_framework_value::ValueError> {
    let (items, retained_bytes) = match mutation {
        Process3dMutation::CreateStep(payload) => (1, process3d_step_bytes(&payload.step)?),
        Process3dMutation::DeleteStep(payload) => (1, process3d_text_bytes(&payload.id)?),
        Process3dMutation::RenameStep(payload) => (1, process3d_text_bytes(&payload.id)?.saturating_add(process3d_text_bytes(&payload.new_label)?)),
        Process3dMutation::ChangeStepEnabled(payload) => (1, process3d_text_bytes(&payload.id)?),
        Process3dMutation::ChangeStepOrigin(payload) => (1, process3d_text_bytes(&payload.id)?.saturating_add(process3d_origin_bytes(payload.new_origin.as_ref())?)),
        Process3dMutation::ReplaceStepMeasure(payload) => (1, process3d_text_bytes(&payload.id)?.saturating_add(process3d_measure_bytes(&payload.new_measure)?)),
        Process3dMutation::ReorderSteps(payload) => (1, process3d_text_bytes(&payload.id)?),
        Process3dMutation::CreateMachine(payload) => (process3d_machine_items(&payload.machine), process3d_machine_bytes(&payload.machine)?),
        Process3dMutation::DeleteMachine(payload) => (1, process3d_text_bytes(&payload.id)?),
        Process3dMutation::RenameMachine(payload) => (1, process3d_text_bytes(&payload.id)?.saturating_add(process3d_text_bytes(&payload.new_label)?)),
        Process3dMutation::ChangeMachineIcon(payload) => (1, process3d_text_bytes(&payload.id)?.saturating_add(process3d_text_bytes(&payload.new_icon_id)?)),
        Process3dMutation::ReplaceMachineCapabilities(payload) => (process3d_capabilities_items(&payload.new_capabilities), process3d_text_bytes(&payload.id)?.saturating_add(process3d_capabilities_bytes(&payload.new_capabilities)?)),
        Process3dMutation::MoveStock(_) => (1, 0),
        Process3dMutation::ChangeStockLabel(payload) => (1, process3d_text_bytes(&payload.new_label)?),
        Process3dMutation::ReplaceStockSolid(payload) => (1, process3d_child_bytes(&payload.new_solid)?),
    };
    let retained_bytes = retained_bytes.saturating_add(size_of::<Process3dMutation>());
    if items > PROCESS3D_DOCUMENT_MAXIMUM_ITEMS {
        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit, "Process3d document mutation exceeds its fixed one-item preparation envelope"));
    }
    if retained_bytes > PROCESS3D_DOCUMENT_MAXIMUM_BYTES {
        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Process3d document mutation exceeds its fixed one-item preparation envelope"));
    }
    Ok(retained_bytes)
}

/// 🧮️ Runs the mutation's own semantic `diff`/`inverse` against `base` and applies the resulting diff,
/// so the retained lane and the batch lane can never diverge. An `Error`/`Fatal` outcome (a duplicate
/// step id, a missing target) is a REJECTION here, not a silent no-op: `MutationOutcome::error`/
/// `fatal` force an EMPTY diff, and publishing anyway would write a no-op edit into history.
fn prepare_process3d_document(base: &Process3dSnapshot, mutation: &Process3dMutation) -> Result<(Process3dSnapshot, Vec<Process3dMutation>), String> {
    process3d_mutation_retained_bytes(mutation).map_err(semio_framework_value::ValueError::into_message)?;
    process3d_document_bytes(base).map_err(semio_framework_value::ValueError::into_message)?;
    let outcome = <Process3dMutation as protocol::ColdMutation<Process3dSnapshot>>::diff(mutation, base);
    if let Some(message) = outcome.messages().iter().find(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)) {
        return Err(format!("Process3d document mutation was refused by its own vocabulary: {}", message.message));
    }
    let inverse = <Process3dMutation as protocol::ColdMutation<Process3dSnapshot>>::inverse(mutation, base).map_err(semio_framework_value::ValueError::into_message)?;
    let post = protocol::apply_diff_cold(outcome.diff(), base).map_err(|error| format!("Process3d document mutation could not apply onto its exact base: {}", error.message))?;
    process3d_document_bytes(&post).map_err(semio_framework_value::ValueError::into_message)?;
    Ok((post, inverse))
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Process3dSnapshot, Process3dMutation> for Process3dArtifactPreparationFactory {
    fn begin_batch_digest(
        &self,
        edit: &mut Option<Box<protocol::Edit<Process3dMutation>>>,
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Process3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &Process3dMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Process3d document preparation rejected its lane".into());
        }
        process3d_mutation_retained_bytes(mutation).map_err(semio_framework_value::ValueError::into_message)?;
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, PROCESS3D_DOCUMENT_GRANT_BYTES))
    }

    fn begin_demand(&self, _mutation: &Process3dMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Process3dArtifactPreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Process3dSnapshot, Process3dMutation, Process3dMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<
        (Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Process3dSnapshot, Process3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress),
        (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Process3dSnapshot, Process3dMutation, Process3dMutation>),
    > {
        if self.preflight(&request.mutation, request.lane).is_err()
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d document preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((
            Box::new(Process3dArtifactPreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), retained_bytes: 0, cancelled: false }),
            progress,
        ))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Process3dSnapshot, Process3dMutation> for Process3dArtifactPreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use semio_framework_value::{retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if let Some(error) = self.owners.failure.as_ref() {
            return Err(error.clone());
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Process3d document preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        if self.owners.candidate.is_none() {
            let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Process3d document preparation lost its exact base root"))?.get();
            process3d_document_bytes(base)?;
            if grant.maximum_copy_bytes < PROCESS3D_DOCUMENT_GRANT_BYTES {
                return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
            }
            let mutation = self.owners.mutation.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Process3d document preparation lost its mutation owner"))?;
            match prepare_process3d_document(base, &mutation) {
                Ok((post, inverse)) => *self.owners.candidate = Some((post, inverse, mutation)),
                Err(message) => {
                    *self.owners.mutation = Some(mutation);
                    *self.owners.failure = Some(ValueError::new(ValueRefusalKind::OwnershipLimit, message.clone()));
                    return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, message));
                }
            }
            self.retained_bytes = PROCESS3D_DOCUMENT_GRANT_BYTES;
            self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: PROCESS3D_DOCUMENT_GRANT_BYTES as u64, digest: [0; 32] };
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress::default()));
        }
        if grant.maximum_copy_bytes < self.retained_bytes {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.authority.is_none() {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Process3d document preparation lost its Store authority"));
        }
        let (post, inverse, forward) = self.owners.candidate.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Process3d document preparation lost its candidate"))?;
        let authority = self.owners.authority.as_ref().expect("observed original Store authority");
        let prepared = match authority.prepare_one_item(authority.next_edit(forward, inverse), std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Process3dSnapshot, Process3dMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Process3dSnapshot, Process3dMutation>> {
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
