//! 🪜️ Process 3d play app commands — process-step lifecycle (add / remove / move / update / enable).

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigOperation};
use crate::artifacts::process3d::engine::{capability_for_measure_kind, find_capability, insert_step_operations, measure_for_capability, next_step_id, remove_step_operations, validate_capability, validation_context_for_stock};
use crate::artifacts::process3d::{op::Process3dOperation, MeasureKind, Process3dDocument, ProcessStep, ProcessStepPatch, StepOrigin};
use protocol::CollectionOperation;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️AddStep
pub mod add_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-step")]
    pub struct AddStep {
        pub measure: Option<String>,
        pub machine_id: Option<String>,
        pub capability_id: Option<String>,
        #[dsl(coord)]
        pub position: Option<[f64; 3]>,
    }

    pub fn handle(payload: &AddStep, doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        let fixture = doc.projection;
        let resolved = if let (Some(machine_id), Some(capability_id)) = (payload.machine_id.as_deref(), payload.capability_id.as_deref()) {
            find_capability(&fixture.workshop, machine_id, capability_id).map(|(machine, capability)| (machine.clone(), capability.clone()))
        } else {
            let measure_kind = match payload.measure.as_deref().unwrap_or("cut") {
                "drill" => MeasureKind::Drill,
                "attach" => MeasureKind::Attach,
                _ => MeasureKind::Cut,
            };
            Some(capability_for_measure_kind(&fixture.workshop, measure_kind))
        };
        let Some((machine, capability)) = resolved else {
            return Ok(Emit::default());
        };
        let failures = validate_capability(&capability, &validation_context_for_stock(&fixture.stock));
        if !failures.is_empty() {
            return Ok(Emit::default());
        }
        let origin = StepOrigin { machine_id: machine.id.clone(), capability_id: capability.id.clone() };
        let step = ProcessStep { id: next_step_id(), label: capability.label.clone(), enabled: true, origin: Some(origin), measure: measure_for_capability(&capability, payload.position) };
        let step_id = step.id.clone();
        Ok(Emit { document_operations: insert_step_operations(fixture, step), config_operations: vec![Process3dConfigOperation::SetSelectedId { value: Some(step_id) }], ..Default::default() })
    }
}
//#endregion 🔖️AddStep

//#region 🔖️RemoveStep
pub mod remove_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-step")]
    pub struct RemoveStep {
        pub id: String,
    }

    pub fn handle(payload: &RemoveStep, doc: &DocumentView<'_, Process3dDocument>, cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        let fixture = doc.projection;
        let config = cfg.projection;
        match remove_step_operations(fixture, &payload.id) {
            Some(operations) => {
                let mut config_operations = Vec::new();
                if config.selected_id.as_deref() == Some(payload.id.as_str()) {
                    config_operations.push(Process3dConfigOperation::SetSelectedId { value: None });
                }
                Ok(Emit { document_operations: operations, config_operations, ..Default::default() })
            }
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️RemoveStep

//#region 🔖️RemoveSelectedStep
pub mod remove_selected_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-selected-step")]
    pub struct RemoveSelectedStep {}

    pub fn handle(_payload: &RemoveSelectedStep, doc: &DocumentView<'_, Process3dDocument>, cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        let fixture = doc.projection;
        match cfg.projection.selected_id.clone() {
            Some(id) => match remove_step_operations(fixture, &id) {
                Some(operations) => Ok(Emit { document_operations: operations, config_operations: vec![Process3dConfigOperation::SetSelectedId { value: None }], ..Default::default() }),
                None => Ok(Emit::default()),
            },
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️RemoveSelectedStep

//#region 🔖️MoveStep
pub mod move_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "move-step")]
    pub struct MoveStep {
        pub id: String,
        pub index: usize,
    }

    pub fn handle(payload: &MoveStep, doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        if doc.projection.steps.iter().any(|step| step.id == payload.id) {
            Ok(Emit::operations(vec![Process3dOperation::Steps { collection: CollectionOperation::Move { id: payload.id.clone(), to: payload.index } }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️MoveStep

//#region 🔖️UpdateStep
pub mod update_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "update-step")]
    pub struct UpdateStep {
        #[dsl(block)]
        pub step: ProcessStep,
    }

    pub fn handle(payload: &UpdateStep, doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        if doc.projection.steps.iter().any(|existing| existing.id == payload.step.id) {
            let patch = ProcessStepPatch { label: Some(payload.step.label.clone()), enabled: Some(payload.step.enabled), measure: Some(payload.step.measure.clone()), origin: Some(payload.step.origin.clone()) };
            Ok(Emit::operations(vec![Process3dOperation::Steps { collection: CollectionOperation::Patch { id: payload.step.id.clone(), patch } }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️UpdateStep

//#region 🔖️SetStepEnabled
pub mod set_step_enabled {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-step-enabled")]
    pub struct SetStepEnabled {
        pub id: String,
        pub enabled: bool,
    }

    pub fn handle(payload: &SetStepEnabled, doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        if doc.projection.steps.iter().any(|step| step.id == payload.id) {
            let patch = ProcessStepPatch { enabled: Some(payload.enabled), ..Default::default() };
            Ok(Emit::operations(vec![Process3dOperation::Steps { collection: CollectionOperation::Patch { id: payload.id.clone(), patch } }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️SetStepEnabled
