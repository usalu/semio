//! 🪜️ Process 3d play app commands — process-step lifecycle (add / remove / move / update / enable).

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::artifacts::process3d::schema::inferences::{capability_for_measure_kind, find_capability, measure_for_capability, validate_capability, validation_context_for_stock};
use crate::artifacts::process3d::schema::{insert_step_mutations, next_step_id, remove_step_mutations};
use crate::artifacts::process3d::mutations::change_step_enabled::mutation::ChangeStepEnabled;
use crate::artifacts::process3d::mutations::change_step_origin::mutation::ChangeStepOrigin;
use crate::artifacts::process3d::mutations::rename_step::mutation::RenameStep;
use crate::artifacts::process3d::mutations::reorder_steps::mutation::ReorderSteps;
use crate::artifacts::process3d::mutations::replace_step_measure::mutation::ReplaceStepMeasure;
use crate::artifacts::process3d::{op::Process3dMutation, MeasureKind, Process3dSnapshot, ProcessStep, StepOrigin};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
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

    pub fn handle(payload: &AddStep, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fixture = doc.snapshot;
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
        let origin = StepOrigin { machine_id: machine.id, capability_id: capability.id.clone() };
        let step = ProcessStep { id: next_step_id(), label: capability.label.clone(), enabled: true, origin: Some(origin), measure: measure_for_capability(&capability, payload.position) };
        let step_id = step.id.clone();
        Ok(Emit { artifact_mutations: insert_step_mutations(fixture, step), config_mutations: vec![Process3dConfigMutation::SetSelectedId { value: Some(step_id) }], ..Default::default() })
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

    pub fn handle(payload: &RemoveStep, doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let config = cfg.snapshot;
        match remove_step_mutations(fixture, &payload.id) {
            Some(operations) => {
                let mut config_mutations = Vec::new();
                if config.selected_id.as_deref() == Some(payload.id.as_str()) {
                    config_mutations.push(Process3dConfigMutation::SetSelectedId { value: None });
                }
                Ok(Emit { artifact_mutations: operations, config_mutations, ..Default::default() })
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

    pub fn handle(_payload: &RemoveSelectedStep, doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        match cfg.snapshot.selected_id.clone() {
            Some(id) => match remove_step_mutations(fixture, &id) {
                Some(operations) => Ok(Emit { artifact_mutations: operations, config_mutations: vec![Process3dConfigMutation::SetSelectedId { value: None }], ..Default::default() }),
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

    pub fn handle(payload: &MoveStep, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        if doc.snapshot.steps.iter().any(|step| step.id == payload.id) {
            Ok(Emit::mutations(vec![Process3dMutation::ReorderSteps(ReorderSteps { id: payload.id.clone(), to_index: payload.index })]))
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

    /// 🔧️ Programmatic full-step edit — each field carries its own semantic mutation now
    /// (`RenameStep`/`ChangeStepEnabled`/`ChangeStepOrigin`/`ReplaceStepMeasure`), so this diffs
    /// `payload.step` against the current entity and emits one targeted mutation per changed field.
    pub fn handle(payload: &UpdateStep, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let Some(existing) = doc.snapshot.steps.iter().find(|existing| existing.id == payload.step.id) else {
            return Ok(Emit::default());
        };
        let mut operations = Vec::new();
        if existing.label != payload.step.label {
            operations.push(Process3dMutation::RenameStep(RenameStep { id: payload.step.id.clone(), new_label: payload.step.label.clone() }));
        }
        if existing.enabled != payload.step.enabled {
            operations.push(Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: payload.step.id.clone(), new_enabled: payload.step.enabled }));
        }
        if existing.origin != payload.step.origin {
            operations.push(Process3dMutation::ChangeStepOrigin(ChangeStepOrigin { id: payload.step.id.clone(), new_origin: payload.step.origin.clone() }));
        }
        if existing.measure != payload.step.measure {
            operations.push(Process3dMutation::ReplaceStepMeasure(ReplaceStepMeasure { id: payload.step.id.clone(), new_measure: payload.step.measure.clone() }));
        }
        Ok(Emit::mutations(operations))
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

    pub fn handle(payload: &SetStepEnabled, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        if doc.snapshot.steps.iter().any(|step| step.id == payload.id) {
            Ok(Emit::mutations(vec![Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: payload.id.clone(), new_enabled: payload.enabled })]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️SetStepEnabled
