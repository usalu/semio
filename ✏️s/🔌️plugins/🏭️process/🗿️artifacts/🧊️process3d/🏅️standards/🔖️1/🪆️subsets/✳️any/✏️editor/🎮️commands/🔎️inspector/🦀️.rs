//! 🔎️ Process 3d play app commands — the generic inspector field-patch dispatcher, addressed by a
//! `target`/`field` pair against the stock, a selected step, or a workshop machine.

use semio_s_artifact_process_process3d_config::{Process3dConfig, Process3dConfigMutation};
use semio_s_artifact_process_process3d_mut_stock::change_stock_label::ChangeStockLabel;
use semio_s_artifact_process_process3d_mut_stock::move_stock::MoveStock;
use semio_s_artifact_process_process3d_mut_stock::resize_stock::ResizeStock;
use semio_s_artifact_process_process3d_mut_machines::rename_machine::RenameMachine;
use semio_s_artifact_process_process3d_mut_machines::replace_machine_capabilities::ReplaceMachineCapabilities;
use semio_s_artifact_process_process3d_mut_steps::{rename_step::RenameStep, replace_step_measure::ReplaceStepMeasure};
use semio_s_artifact_process_process3d_mutations::Process3dMutation;
use semio_s_artifact_process_process3d_model::snapshot::{Pose, Process3dSnapshot, ProcessMeasure, ProcessStep, WorkshopMachine};
use semio_framework::DslValue;
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️InspectorPatch
fn apply_pose_patch(pose: &mut Pose, field: &str, value: f64) -> bool {
    match field {
        "posX" => pose.position[0] = value,
        "posY" => pose.position[1] = value,
        "posZ" => pose.position[2] = value,
        "angle" => pose.angle = value,
        _ => return false,
    }
    true
}

fn inspector_number(value: &DslValue) -> Option<f64> {
    value.as_f64().or_else(|| value.as_str()?.trim().parse().ok()).filter(|number| number.is_finite())
}

fn stock_patch_operation(snapshot: &Process3dSnapshot, field: &str, value: &DslValue) -> Option<Process3dMutation> {
    if field == "label" {
        let label = value.as_str()?;
        return (snapshot.stock_label != label).then(|| Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: label.into() }));
    }
    let number = inspector_number(value)?;
    let mut pose = snapshot.stock_pose.clone();
    if apply_pose_patch(&mut pose, field, number) {
        return (pose != snapshot.stock_pose).then_some(Process3dMutation::MoveStock(MoveStock { new_pose: pose }));
    }
    let mut solid = snapshot.stock_payload.solid.clone();
    let dimension = solid.dimension_mut(field)?;
    (number > 0.0 && *dimension != number).then(|| Process3dMutation::ResizeStock(ResizeStock { field: field.into(), size: number }))
}

/// 🔎️ Generic inspector edit dispatcher for a workshop machine's own label or a capability parameter
/// value, addressed as `"{capabilityId}.{parameterId}"` so field names never collide across capabilities.
fn apply_workshop_machine_patch(machine: &mut WorkshopMachine, field: &str, value: &DslValue) -> bool {
    if field == "label" {
        return match value.as_str() {
            Some(label) => {
                machine.label = label.into();
                true
            }
            None => false,
        };
    }
    let Some((capability_id, parameter_id)) = field.split_once('.') else { return false };
    let Some(number) = inspector_number(value) else { return false };
    let clamped = number.max(0.001);
    let Some(capability) = machine.capabilities.iter_mut().find(|capability| capability.id == capability_id) else { return false };
    let Some(parameter) = capability.parameters.iter_mut().find(|parameter| parameter.id == parameter_id) else { return false };
    parameter.value = clamped;
    true
}

fn step_patch_operation(step: &ProcessStep, field: &str, value: &DslValue) -> Option<Process3dMutation> {
    if field == "label" {
        let label = value.as_str()?;
        return (step.label != label).then(|| Process3dMutation::RenameStep(RenameStep { id: step.id.clone(), new_label: label.into() }));
    }
    let number = inspector_number(value)?;
    let mut measure = step.measure.clone();
    let pose = match &mut measure {
        ProcessMeasure::Cut { pose, .. } | ProcessMeasure::Drill { pose, .. } | ProcessMeasure::Attach { pose, .. } => pose,
    };
    if !apply_pose_patch(pose, field, number) {
        let dimension = match &mut measure {
            ProcessMeasure::Cut { tool, .. } => tool.dimension_mut(field),
            ProcessMeasure::Attach { component, .. } => component.dimension_mut(field),
            ProcessMeasure::Drill { radius, .. } if field == "radius" => Some(radius),
            ProcessMeasure::Drill { depth, .. } if field == "depth" => Some(depth),
            ProcessMeasure::Drill { .. } => None,
        }?;
        if number <= 0.0 {
            return None;
        }
        *dimension = number;
    }
    (measure != step.measure).then(|| Process3dMutation::ReplaceStepMeasure(ReplaceStepMeasure { id: step.id.clone(), new_measure: measure }))
}

/// 🩹️ Routes one inspector field edit to the semantic mutation for its step, stock or workshop machine.
fn process3d_inspector_patch_operation(snapshot: &Process3dSnapshot, target: &str, field: &str, value: Option<&DslValue>) -> Option<Process3dMutation> {
    let value = value?;
    if target == snapshot.stock_id {
        return stock_patch_operation(snapshot, field, value);
    }
    let machine = target.strip_prefix("machine:").and_then(|id| snapshot.workshop.machines.iter().find(|machine| machine.id == id));
    if let Some(machine) = machine {
        let mut updated = machine.clone();
        if !apply_workshop_machine_patch(&mut updated, field, value) || updated == *machine {
            return None;
        }
        return Some(if field == "label" {
            Process3dMutation::RenameMachine(RenameMachine { id: machine.id.clone(), new_label: updated.label })
        } else {
            Process3dMutation::ReplaceMachineCapabilities(ReplaceMachineCapabilities { id: machine.id.clone(), new_capabilities: updated.capabilities })
        });
    }
    let step = snapshot.step_payloads.iter().find(|step| step.id == target)?;
    step_patch_operation(step, field, value)
}
//#endregion 🔖️InspectorPatch

//#region 🔖️PatchInspector
pub mod patch_inspector {
    use super::*;

    /// 🩹️ Mirrors the panel's `{ target, field, value }` args — `value` is either a number (most fields)
    /// or text (the `label` field); the two are mutually exclusive at any one call site.
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "patch-inspector")]
    pub struct PatchInspector {
        pub target: String,
        pub field: String,
        pub number: Option<f64>,
        pub text: Option<String>,
    }

    pub fn handle(
        payload: &PatchInspector,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        _cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut semio_s_artifact_process_process3d_kit::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let value = payload.number.map(DslValue::float).or_else(|| payload.text.clone().map(DslValue::String));
        match process3d_inspector_patch_operation(doc.snapshot, &payload.target, &payload.field, value.as_ref()) {
            Some(operation) => Ok(Emit::mutations(vec![operation])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️PatchInspector
