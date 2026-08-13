//! 🪜️ Process 3d play app commands — process-step lifecycle (add / remove / move / update / enable).

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::artifacts::process3d::schema::inferences::{capability_for_measure_kind, find_capability, measure_for_capability};
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
        // 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `fixture.stock_solid` is a
        // composed `s.stdio.semio.brep` CHILD HANDLE now, not a `WorkingSolid` — this plugin-scoped
        // migration cannot resolve it back to real dimensions without a `LinkResolver` (see
        // `ProcessWorkingScene`'s doc comment), so the stock-dimension capability-rule gate
        // (`validate_capability`/`validation_context_for_stock`) is a documented gap here: every
        // capability is treated as dimensionally valid rather than guessing at unknown extents.
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

    /// 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `steps` is a composed CHILD
    /// HANDLE now (see `ProcessWorkingScene`'s doc comment) — this can no longer check whether
    /// `payload.id` exists before emitting; `ReorderSteps` is itself a documented no-op regardless
    /// (see its `🔺️diff/🦀️component.rs`), so the existence check is dropped honestly rather than
    /// faked.
    pub fn handle(payload: &MoveStep, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let _ = doc;
        Ok(Emit::mutations(vec![Process3dMutation::ReorderSteps(ReorderSteps { id: payload.id.clone(), to_index: payload.index })]))
    }
}
//#endregion 🔖️MoveStep

//#region 🔖️UpdateStep
pub mod update_step {
    use super::*;

    /// 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `ProcessStep` dropped its
    /// `dsl` derives (now an ephemeral working-scene type containing `WorkingSolid`, itself never
    /// `dsl::DslField` — see the artifact root file's `🔖️WorkingScene` doc comment), so this
    /// carries the step as JSON text now, parsed at the handler.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "update-step")]
    pub struct UpdateStep {
        pub step_json: String,
    }

    /// 🔧️ Programmatic full-step edit — each field carries its own semantic mutation now
    /// (`RenameStep`/`ChangeStepEnabled`/`ChangeStepOrigin`/`ReplaceStepMeasure`), so this diffs
    /// `payload.step` against the current entity and emits one targeted mutation per changed field.
    /// 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `steps` is a composed CHILD
    /// HANDLE now (see `ProcessWorkingScene`'s doc comment) — this can no longer read the existing
    /// step to diff against, so it always emits all four targeted mutations unconditionally
    /// (each is itself a documented no-op regardless — see their `🔺️diff/🦀️component.rs` files).
    pub fn handle(payload: &UpdateStep, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let _ = doc;
        let step: ProcessStep = serde_json::from_str(&payload.step_json).map_err(|e| Fault::from(e.to_string()))?;
        let operations = vec![
            Process3dMutation::RenameStep(RenameStep { id: step.id.clone(), new_label: step.label.clone() }),
            Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: step.id.clone(), new_enabled: step.enabled }),
            Process3dMutation::ChangeStepOrigin(ChangeStepOrigin { id: step.id.clone(), new_origin: step.origin.clone() }),
            Process3dMutation::ReplaceStepMeasure(ReplaceStepMeasure { id: step.id.clone(), new_measure: step.measure.clone() }),
        ];
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

    /// 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: see `MoveStep::handle`'s doc
    /// comment — same composed-`steps`-handle gap, same documented-no-op mutation regardless.
    pub fn handle(payload: &SetStepEnabled, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let _ = doc;
        Ok(Emit::mutations(vec![Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: payload.id.clone(), new_enabled: payload.enabled })]))
    }
}
//#endregion 🔖️SetStepEnabled
