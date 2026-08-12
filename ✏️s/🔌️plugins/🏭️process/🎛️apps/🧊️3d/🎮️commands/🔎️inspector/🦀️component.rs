//! 🔎️ Process 3d play app commands — the generic inspector field-patch dispatcher, addressed by a
//! `target`/`field` pair against the stock, a selected step, or a workshop machine.

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::artifacts::process3d::mutations::change_stock_label::mutation::ChangeStockLabel;
use crate::artifacts::process3d::mutations::move_stock::mutation::MoveStock;
use crate::artifacts::process3d::mutations::rename_machine::mutation::RenameMachine;
use crate::artifacts::process3d::mutations::rename_step::mutation::RenameStep;
use crate::artifacts::process3d::mutations::replace_machine_capabilities::mutation::ReplaceMachineCapabilities;
use crate::artifacts::process3d::mutations::replace_step_measure::mutation::ReplaceStepMeasure;
use crate::artifacts::process3d::mutations::replace_stock_solid::mutation::ReplaceStockSolid;
use crate::artifacts::process3d::{op::Process3dMutation, Pose, Process3dSnapshot, ProcessMeasure, SolidSpec, WorkshopMachine};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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

fn apply_solid_patch(solid: &mut SolidSpec, field: &str, value: f64) -> bool {
    let clamped = value.max(0.001);
    match solid {
        SolidSpec::Box { width, depth, height } => match field {
            "width" => *width = clamped,
            "depth" => *depth = clamped,
            "height" => *height = clamped,
            _ => return false,
        },
        SolidSpec::Cylinder { radius, height } => match field {
            "radius" => *radius = clamped,
            "height" => *height = clamped,
            _ => return false,
        },
        SolidSpec::Sphere { radius } => match field {
            "radius" => *radius = clamped,
            _ => return false,
        },
        SolidSpec::ImportedMesh { .. } | SolidSpec::ImportedSolid { .. } => return false,
    }
    true
}

fn apply_stock_patch(stock: &mut crate::artifacts::process3d::Stock, field: &str, value: Option<&Value>) -> bool {
    if field == "label" {
        return match value.and_then(Value::as_str) {
            Some(label) => {
                stock.label = label.into();
                true
            }
            None => false,
        };
    }
    let Some(number) = value.and_then(Value::as_f64) else { return false };
    apply_pose_patch(&mut stock.pose, field, number) || apply_solid_patch(&mut stock.solid, field, number)
}

/// 🔎️ Generic inspector edit dispatcher for a step's measure — dimension fields are scoped to the
/// measure's own solid ("radius"/"depth" for drill, "toolWidth..." for cut, "radius"/"height" for attach)
/// so field names never collide across measure kinds.
fn apply_step_patch(step: &mut crate::artifacts::process3d::ProcessStep, field: &str, value: Option<&Value>) -> bool {
    if field == "label" {
        return match value.and_then(Value::as_str) {
            Some(label) => {
                step.label = label.into();
                true
            }
            None => false,
        };
    }
    let Some(number) = value.and_then(Value::as_f64) else { return false };
    let clamped = number.max(0.001);
    match &mut step.measure {
        ProcessMeasure::Cut { tool, pose } => {
            if apply_pose_patch(pose, field, number) {
                return true;
            }
            let SolidSpec::Box { width, depth, height } = tool else { return false };
            match field {
                "toolWidth" => *width = clamped,
                "toolDepth" => *depth = clamped,
                "toolHeight" => *height = clamped,
                _ => return false,
            }
            true
        }
        ProcessMeasure::Drill { radius, depth, pose } => {
            if apply_pose_patch(pose, field, number) {
                return true;
            }
            match field {
                "radius" => *radius = clamped,
                "depth" => *depth = clamped,
                _ => return false,
            }
            true
        }
        ProcessMeasure::Attach { component, pose } => {
            if apply_pose_patch(pose, field, number) {
                return true;
            }
            let SolidSpec::Cylinder { radius, height } = component else { return false };
            match field {
                "radius" => *radius = clamped,
                "height" => *height = clamped,
                _ => return false,
            }
            true
        }
    }
}

/// 🔎️ Generic inspector edit dispatcher for a workshop machine's own label or a capability parameter
/// value, addressed as `"{capabilityId}.{parameterId}"` so field names never collide across capabilities.
fn apply_workshop_machine_patch(machine: &mut WorkshopMachine, field: &str, value: Option<&Value>) -> bool {
    if field == "label" {
        return match value.and_then(Value::as_str) {
            Some(label) => {
                machine.label = label.into();
                true
            }
            None => false,
        };
    }
    let Some((capability_id, parameter_id)) = field.split_once('.') else { return false };
    let Some(number) = value.and_then(Value::as_f64) else { return false };
    let clamped = number.max(0.001);
    let Some(capability) = machine.capabilities.iter_mut().find(|capability| capability.id == capability_id) else { return false };
    let Some(parameter) = capability.parameters.iter_mut().find(|parameter| parameter.id == parameter_id) else { return false };
    parameter.value = clamped;
    true
}

/// 🩹️ Builds the `Process3dMutation` for one inspector field edit — clones the target (stock, step, or
/// workshop machine), mutates the clone via `apply_stock_patch`/`apply_step_patch`/
/// `apply_workshop_machine_patch`, then routes the touched field into its own semantic mutation:
/// `label` → `RenameMachine`/`RenameStep`/`ChangeStockLabel`, a spatial stock field →
/// `MoveStock`, a capability parameter → `ReplaceMachineCapabilities`, everything else (the step's
/// measure geometry, the stock's solid dims) → `ReplaceStepMeasure`/`ReplaceStockSolid`.
fn process3d_inspector_patch_operation(fixture: &Process3dSnapshot, target: &str, field: &str, value: Option<&Value>) -> Option<Process3dMutation> {
    if let Some(machine_id) = target.strip_prefix("machine:") {
        let machine = fixture.workshop.machines.iter().find(|machine| machine.id == machine_id)?;
        let mut updated = machine.clone();
        if !apply_workshop_machine_patch(&mut updated, field, value) {
            return None;
        }
        return Some(if field == "label" {
            Process3dMutation::RenameMachine(RenameMachine { id: machine_id.to_string(), new_label: updated.label })
        } else {
            Process3dMutation::ReplaceMachineCapabilities(ReplaceMachineCapabilities { id: machine_id.to_string(), new_capabilities: updated.capabilities })
        });
    }
    if target == fixture.stock.id {
        let mut stock = fixture.stock.clone();
        if !apply_stock_patch(&mut stock, field, value) {
            return None;
        }
        return Some(match field {
            "label" => Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: stock.label }),
            "posX" | "posY" | "posZ" | "angle" => Process3dMutation::MoveStock(MoveStock { new_pose: stock.pose }),
            _ => Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: stock.solid }),
        });
    }
    let step_id = target.strip_prefix("step:")?;
    let step = fixture.steps.iter().find(|step| step.id == step_id)?;
    let mut updated = step.clone();
    if !apply_step_patch(&mut updated, field, value) {
        return None;
    }
    Some(if field == "label" {
        Process3dMutation::RenameStep(RenameStep { id: step_id.to_string(), new_label: updated.label })
    } else {
        Process3dMutation::ReplaceStepMeasure(ReplaceStepMeasure { id: step_id.to_string(), new_measure: updated.measure })
    })
}
//#endregion 🔖️InspectorPatch

//#region 🔖️PatchInspector
pub mod patch_inspector {
    use super::*;

    /// 🩹️ Mirrors the panel's `{ target, field, value }` args — `value` is either a number (most fields)
    /// or text (the `label` field); the two are mutually exclusive at any one call site.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-inspector")]
    pub struct PatchInspector {
        pub target: String,
        pub field: String,
        pub number: Option<f64>,
        pub text: Option<String>,
    }

    pub fn handle(payload: &PatchInspector, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let value = payload.number.map(|n| json!(n)).or_else(|| payload.text.clone().map(Value::String));
        match process3d_inspector_patch_operation(doc.snapshot, &payload.target, &payload.field, value.as_ref()) {
            Some(operation) => Ok(Emit::mutations(vec![operation])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️PatchInspector
