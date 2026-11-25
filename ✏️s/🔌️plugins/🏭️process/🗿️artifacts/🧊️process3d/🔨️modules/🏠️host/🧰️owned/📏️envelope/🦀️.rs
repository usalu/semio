//! 📏️ Process3d document envelope: the fixed retained byte and item measures of a base and of one mutation, and the bounded cold preparation factory built on them.

use super::*;
use semio_s_artifact_process_process3d_model::snapshot::{Capability, CapabilityRule, MeasureRecipe, ProcessMeasure, ProcessStep, StepOrigin, Stock, WorkingSolid, WorkshopMachine};
use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceText as _;
use std::mem::size_of;

/// 🧺️ Fixed envelope for the process3d document's retained one-item publication lane. The document's
/// only unbounded axes are the durable `step_payloads` timeline, its re-minted `tool_solids` handles,
/// and the workshop's machines with their capability leaves — each counted here rather than encoded.
const PROCESS3D_DOCUMENT_MAXIMUM_ITEMS: usize = 4_096;
/// 🔤️ Longest single id, label, handle or recipe parameter name the retained lane admits, in either
/// the base or a mutation payload.
pub const PROCESS3D_DOCUMENT_TEXT_BYTES: usize = 256;
pub const PROCESS3D_DOCUMENT_MAXIMUM_BYTES: usize = 512 * 1_024;
/// 🎟️ What one `advance`/`close_step` turn costs, and the ONLY figure the grant is ever compared
/// against. The host drives this lane with a fixed `ArtifactStoreOneItemGrant { maximum_items: 1,
/// maximum_copy_bytes: TYPED_OPERATION_RESULT_PAGE_BYTES }` (4 KiB), so a gate that scaled with the
/// document — `grant.maximum_copy_bytes < measured_base_bytes` — would go `Blocked` forever the moment a
/// timeline or a workshop outgrew one page, stalling the operation instead of failing it. The base's
/// own size is a VALIDATION (`process3d_document_bytes`, rejected past
/// `PROCESS3D_DOCUMENT_MAXIMUM_BYTES`), never the gate.
pub const PROCESS3D_DOCUMENT_GRANT_BYTES: usize = 4_096;

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
        WorkingSolid::ImportedMesh { mesh_url } if mesh_url.starts_with("data:") => {
            if mesh_url.len() > semio_s_artifact_process_process3d_io::PROCESS3D_MODEL_DATA_URL_BYTES {
                return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Imported mesh exceeds the document geometry budget"));
            }
            Ok(mesh_url.len())
        }
        WorkingSolid::ImportedMesh { mesh_url } => process3d_text_bytes(mesh_url),
        WorkingSolid::ImportedSolid { brep_pack } => {
            if brep_pack.len() > semio_s_artifact_process_process3d_model::snapshot::PROCESS3D_IMPORTED_BREP_PACK_BYTES {
                return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Imported stock exceeds the exact B-Rep pack budget"));
            }
            Ok(brep_pack.len())
        }
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
pub fn process3d_document_bytes(document: &Process3dSnapshot) -> Result<usize, semio_framework_value::ValueError> {
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
pub fn process3d_mutation_retained_bytes(mutation: &Process3dMutation) -> Result<usize, semio_framework_value::ValueError> {
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
        Process3dMutation::ResizeStock(payload) => (1, process3d_text_bytes(&payload.field)?),
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


fn process3d_snapshot_envelope_bytes(snapshot: &Process3dSnapshot) -> usize {
    process3d_document_bytes(snapshot).unwrap_or(usize::MAX)
}

fn process3d_mutation_envelope_bytes(mutation: &Process3dMutation) -> usize {
    process3d_mutation_retained_bytes(mutation).unwrap_or(usize::MAX)
}

/// 🏭️ The one cold preparation factory of the process3d document lane, bounded by the fixed document envelope; it serves publication and replay alike.
pub fn process3d_document_preparation_factory() -> store::ColdMutationApplyFactory<Process3dSnapshot, Process3dMutation> {
    store::ColdMutationApplyFactory::bounded(store::ColdPreparationBounds { maximum_bytes: PROCESS3D_DOCUMENT_MAXIMUM_BYTES, mutation_bytes: process3d_mutation_envelope_bytes, snapshot_bytes: process3d_snapshot_envelope_bytes })
}
