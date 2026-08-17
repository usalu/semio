//! 🔎️ Process 3d play app commands — the generic inspector field-patch dispatcher, addressed by a
//! `target`/`field` pair against the stock, a selected step, or a workshop machine.

use crate::editor::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::artifacts::process3d::mutations::change_stock_label::mutation::ChangeStockLabel;
use crate::artifacts::process3d::mutations::move_stock::mutation::MoveStock;
use crate::artifacts::process3d::mutations::rename_machine::mutation::RenameMachine;
use crate::artifacts::process3d::mutations::replace_machine_capabilities::mutation::ReplaceMachineCapabilities;
use crate::artifacts::process3d::{op::Process3dMutation, Pose, Process3dSnapshot, WorkshopMachine};
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

/// 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: dimension edits (`width`/`depth`/
/// `height`/`radius`) need to read the stock's CURRENT `WorkingSolid` shape to patch a single field
/// — but `fixture.stock_solid` is a composed `s.stdio.semio.brep` CHILD HANDLE now, with no
/// resolvable content (no `LinkResolver` — see `ProcessWorkingScene`'s doc comment). This is a
/// documented gap: only `label`/pose fields (real, inline persisted fields) remain patchable; a
/// dimension-only patch returns `None` (no mutation) rather than guessing at unknown geometry.
fn apply_stock_patch(stock_pose: &mut Pose, stock_label: &mut String, field: &str, value: Option<&Value>) -> bool {
    if field == "label" {
        return match value.and_then(Value::as_str) {
            Some(label) => {
                *stock_label = label.into();
                true
            }
            None => false,
        };
    }
    let Some(number) = value.and_then(Value::as_f64) else { return false };
    apply_pose_patch(stock_pose, field, number)
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

/// 🩹️ Builds the `Process3dMutation` for one inspector field edit — clones the target (stock or
/// workshop machine), mutates the clone via `apply_stock_patch`/`apply_workshop_machine_patch`,
/// then routes the touched field into its own semantic mutation: `label` →
/// `RenameMachine`/`ChangeStockLabel`, a spatial stock field → `MoveStock`, a capability parameter →
/// `ReplaceMachineCapabilities`. A step-addressed target (`step:<id>`) and a stock dimension-only
/// patch are both a DOCUMENTED NO-OP (see `apply_stock_patch`'s doc comment and
/// `RenameStep`/`ReplaceStepMeasure`'s own triads) — `fixture.steps`/`fixture.stock_solid` carry no
/// resolvable content without a `LinkResolver` this ticket doesn't add.
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
    if target == fixture.stock_id {
        let mut stock_pose = fixture.stock_pose.clone();
        let mut stock_label = fixture.stock_label.clone();
        if !apply_stock_patch(&mut stock_pose, &mut stock_label, field, value) {
            return None;
        }
        return Some(match field {
            "label" => Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: stock_label }),
            _ => Process3dMutation::MoveStock(MoveStock { new_pose: stock_pose }),
        });
    }
    None
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

    pub fn handle(payload: &PatchInspector, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut crate::editor::process3d::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let value = payload.number.map(|n| json!(n)).or_else(|| payload.text.clone().map(Value::String));
        match process3d_inspector_patch_operation(doc.snapshot, &payload.target, &payload.field, value.as_ref()) {
            Some(operation) => Ok(Emit::mutations(vec![operation])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️PatchInspector
