//! 🪜️ Process 3d play app commands — process-step lifecycle (add / remove / move / update / enable).

use crate::editor::process3d::config::{Process3dConfig, Process3dConfigMutation, Process3dConfigSetCursor};
use crate::standards::v1::subsets::any::schema::mutations::change_step_enabled::ChangeStepEnabled;
use crate::standards::v1::subsets::any::schema::mutations::change_step_origin::ChangeStepOrigin;
use crate::standards::v1::subsets::any::schema::mutations::rename_step::RenameStep;
use crate::standards::v1::subsets::any::schema::mutations::reorder_steps::ReorderSteps;
use crate::standards::v1::subsets::any::schema::mutations::replace_step_measure::ReplaceStepMeasure;
use crate::schema::inferences::{capability_for_measure_kind, find_capability, measure_for_capability};
use crate::editor::process3d::commands::cursor::process3d_cursor_moves;
use crate::schema::{insert_step_mutations, next_step_id, process3d_cursor_after_insert, process3d_cursor_after_remove, remove_step_mutations};
use crate::standards::v1::subsets::any::schema::mutations::Process3dMutation;
use crate::{MeasureKind, Process3dSnapshot, ProcessStep, StepOrigin};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️AddStep
pub mod add_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "add-step")]
    pub struct AddStep {
        pub measure: Option<String>,
        pub machine_id: Option<String>,
        pub capability_id: Option<String>,
        #[dsl(coord)]
        pub position: Option<[f64; 3]>,
    }

    pub fn handle(payload: &AddStep, doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut crate::editor::process3d::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
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
        let step = ProcessStep { id: next_step_id(fixture), label: capability.label.clone(), enabled: true, origin: Some(origin), measure: measure_for_capability(&capability, payload.position) };
        Ok(insert_step_emit(fixture, cfg.snapshot, step))
    }
}

/// ➕️ The emit that inserts `step` at the viewer's replay cursor and moves that cursor past it on the config lane.
pub fn insert_step_emit(fixture: &Process3dSnapshot, config: &Process3dConfig, step: ProcessStep) -> Emit<Process3dMutation, Process3dConfigMutation> {
    let cursor = config.resolved_up_to;
    let next = process3d_cursor_after_insert(fixture, cursor);
    let config_mutations = if next == cursor { Vec::new() } else { vec![Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ value: next })] };
    Emit { artifact_mutations: insert_step_mutations(fixture, step, cursor), config_mutations, ..Default::default() }
}

/// ➖️ The emit that removes the step `id` and pulls the viewer's replay cursor back when it sat past it; nothing when
/// no such step exists.
pub fn remove_step_emit(fixture: &Process3dSnapshot, config: &Process3dConfig, id: &str) -> Emit<Process3dMutation, Process3dConfigMutation> {
    match remove_step_mutations(fixture, id) {
        Some(operations) => Emit { artifact_mutations: operations, config_mutations: process3d_cursor_moves(fixture, config, process3d_cursor_after_remove(fixture, id, config.resolved_up_to)), ..Default::default() },
        None => Emit::default(),
    }
}
//#endregion 🔖️AddStep

//#region 🔖️RemoveStep
pub mod remove_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "remove-step")]
    pub struct RemoveStep {
        pub id: String,
    }

    pub fn handle(payload: &RemoveStep, doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut crate::editor::process3d::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(remove_step_emit(doc.snapshot, cfg.snapshot, &payload.id))
    }
}
//#endregion 🔖️RemoveStep

//#region 🔖️RemoveSelectedStep
pub mod remove_selected_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "remove-selected-step")]
    pub struct RemoveSelectedStep {}

    /// 🕹️ Retained selection-operating verb (FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM, 26/08/14):
    /// reads the framework-owned `"geometry"` domain selection (threaded through `ctx.interaction`
    /// by `Process3dPlayApp::handle`, since `dispatch`'s per-payload `handle` never sees
    /// `InteractionView` directly) instead of the deleted `Process3dConfig::selected_id`.
    pub fn handle(
        _payload: &RemoveSelectedStep,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        cfg: &ConfigView<'_, Process3dConfig>,
        ctx: &mut crate::editor::process3d::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(ctx.interaction.ids.first().map_or_else(Emit::default, |id| remove_step_emit(doc.snapshot, cfg.snapshot, id)))
    }
}
//#endregion 🔖️RemoveSelectedStep

//#region 🔖️MoveStep
pub mod move_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "move-step")]
    pub struct MoveStep {
        pub id: String,
        pub index: usize,
    }

    /// 🔀 Existence is validated at diff time (`ReorderSteps`'s own `target-missing` error against
    /// `step_payloads`), so this handler stays a thin, unconditional dispatch.
    pub fn handle(payload: &MoveStep, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut crate::editor::process3d::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
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
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "update-step")]
    pub struct UpdateStep {
        pub step_json: String,
    }

    /// 🔧️ Programmatic full-step edit — each field carries its own semantic mutation
    /// (`RenameStep`/`ChangeStepEnabled`/`ChangeStepOrigin`/`ReplaceStepMeasure`). This always emits
    /// all four unconditionally rather than diffing `payload.step` against the current entity first;
    /// each mutation's own `mutation.no-op` guard against `step_payloads` makes an unchanged field a
    /// harmless warning rather than a spurious write.
    pub fn handle(payload: &UpdateStep, doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut crate::editor::process3d::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let _ = doc;
        let step: ProcessStep = semio_framework_pack_json::from_json_str(&payload.step_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| Fault::from(e.to_string()))?;
        let operations = vec![
            Process3dMutation::RenameStep(RenameStep { id: step.id.clone(), new_label: step.label.clone() }),
            Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: step.id.clone(), new_enabled: step.enabled }),
            Process3dMutation::ChangeStepOrigin(ChangeStepOrigin { id: step.id.clone(), new_origin: step.origin.clone() }),
            Process3dMutation::ReplaceStepMeasure(ReplaceStepMeasure { id: step.id.clone(), new_measure: step.measure }),
        ];
        Ok(Emit::mutations(operations))
    }
}
//#endregion 🔖️UpdateStep

//#region 🔖️SetStepEnabled
pub mod set_step_enabled {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "set-step-enabled")]
    pub struct SetStepEnabled {
        pub id: String,
        pub enabled: bool,
    }

    /// 🔘 Sets the step to exactly `enabled` — the step row target's explicit next state — and emits nothing when the step
    /// already holds it, so a replayed click leaves the one edit the first one made. An unknown id still reaches the
    /// mutation, whose diff validates existence (see `MoveStep::handle`'s doc comment).
    pub fn handle(
        payload: &SetStepEnabled,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        _cfg: &ConfigView<'_, Process3dConfig>,
        _ctx: &mut crate::editor::process3d::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        if doc.snapshot.step_payloads.iter().any(|step| step.id == payload.id && step.enabled == payload.enabled) {
            return Ok(Emit::default());
        }
        Ok(Emit::mutations(vec![Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: payload.id.clone(), new_enabled: payload.enabled })]))
    }
}
//#endregion 🔖️SetStepEnabled
