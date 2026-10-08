//! 🪜️ Sequence play app commands — step CRUD: add/remove/move/patch/collapse a step, delete the
//! current selection.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::sequence::{edit_rules, sequence_edit_emit, sequence_scene_edit};
use crate::mutations::SequenceMutation;
use crate::{SequenceSnapshot, SlotRef, StepParams};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️AddStep
pub mod add_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "add-step")]
    pub struct AddStep {
        pub kind: String,
        pub x: f64,
        pub y: f64,
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: auto-selecting the just-added
    /// step is no longer reachable from this dispatch — selection is framework-owned now, written
    /// only through the injected `interactionSelect` verb.
    pub fn handle(payload: &AddStep, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        let mut edit = sequence_scene_edit(doc)?;
        edit.add_step(&payload.kind, payload.x, payload.y, None);
        Ok(sequence_edit_emit(doc, edit))
    }
}

pub mod add_step_to_slot {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "add-step-to-slot")]
    pub struct AddStepToSlot {
        pub kind: String,
        pub x: f64,
        pub y: f64,
        pub owner: String,
        pub slot_name: String,
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: auto-selecting the just-added
    /// step is no longer reachable from this dispatch — selection is framework-owned now, written
    /// only through the injected `interactionSelect` verb.
    pub fn handle(payload: &AddStepToSlot, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        let mut edit = sequence_scene_edit(doc)?;
        edit.add_step(&payload.kind, payload.x, payload.y, Some(SlotRef { owner: payload.owner.clone(), name: payload.slot_name.clone() }));
        Ok(sequence_edit_emit(doc, edit))
    }
}

pub mod add_step_dropped {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "add-step-dropped")]
    pub struct AddStepDropped {
        pub kind: String,
        pub x: f64,
        pub y: f64,
        pub picked_step_id: Option<String>,
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: auto-selecting the just-added
    /// step is no longer reachable from this dispatch — selection is framework-owned now, written
    /// only through the injected `interactionSelect` verb.
    pub fn handle(payload: &AddStepDropped, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        let mut edit = sequence_scene_edit(doc)?;
        edit.add_step_dropped(&payload.kind, payload.x, payload.y, payload.picked_step_id.as_deref());
        Ok(sequence_edit_emit(doc, edit))
    }
}
//#endregion 🔖️AddStep

//#region 🔖️RemoveStep
pub mod remove_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "remove-step")]
    pub struct RemoveStep {
        pub id: String,
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no longer prunes the removed id
    /// out of a config selection field — the framework auto-prunes a deleted step's id out of the
    /// "steps" domain's live selection via `interaction_topology` after this dispatch lands.
    pub fn handle(payload: &RemoveStep, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        let mut edit = sequence_scene_edit(doc)?;
        let ids = edit_rules::removal_closure(&edit.scene, [payload.id.clone()]);
        edit.remove_steps(&ids);
        Ok(sequence_edit_emit(doc, edit))
    }
}

pub mod delete_selection {
    use super::*;
    use semio_framework_plugin::app::InteractionView;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "delete-selection")]
    pub struct DeleteSelection {}

    fn delete_selected(doc: &ArtifactView<'_, SequenceSnapshot>, selected: &[String]) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        let mut edit = sequence_scene_edit(doc)?;
        let ids = edit_rules::removal_closure(&edit.scene, selected.iter().cloned());
        edit.remove_steps(&ids);
        Ok(sequence_edit_emit(doc, edit))
    }

    /// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg).await` is framework-fixed at this exact 3-arg
    /// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
    /// reachable only through that macro-generated path (`SequencePlayApp::handle` always routes this
    /// command through `apply` below instead), so it degrades to treating the selection as empty.
    pub fn handle(_payload: &DeleteSelection, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        delete_selected(doc, &[])
    }

    pub fn apply(_payload: &DeleteSelection, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>, interaction: &InteractionView<'_>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        delete_selected(doc, &interaction.selection(crate::editor::sequence::SEQUENCE_INTERACTION_STEPS).ids)
    }
}
//#endregion 🔖️RemoveStep

//#region 🔖️MoveStep
pub mod move_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "move-step")]
    pub struct MoveStep {
        pub node_id: String,
        pub x: f64,
        pub y: f64,
    }

    /// 🧭️ Moving one step to an absolute position is the ONE concrete relative `drag-nodes` leaf by the offset between the step's published position and the requested one;
    /// an absent step or an unchanged position publishes nothing.
    pub fn handle(payload: &MoveStep, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        let mut edit = sequence_scene_edit(doc)?;
        if let Some(step) = edit.scene.steps.iter().find(|step| step.id == payload.node_id) {
            let (dx, dy) = (payload.x - step.x, payload.y - step.y);
            edit.drag(std::slice::from_ref(&payload.node_id), dx, dy);
        }
        Ok(sequence_edit_emit(doc, edit))
    }
}
//#endregion 🔖️MoveStep

//#region 🔖️SetStepParams
pub mod set_step_params {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "set-step-params")]
    pub struct SetStepParams {
        pub id: String,
        pub params_json: String,
    }

    pub fn handle(payload: &SetStepParams, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        let mut edit = sequence_scene_edit(doc)?;
        if let Ok(params) = semio_framework_pack_json::from_json_str::<StepParams>(&payload.params_json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
            let _ = edit.set_params(&payload.id, params);
        }
        Ok(sequence_edit_emit(doc, edit))
    }
}
//#endregion 🔖️SetStepParams

//#region 🔖️SetStepCollapsed
pub mod set_step_collapsed {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "set-step-collapsed")]
    pub struct SetStepCollapsed {
        pub id: String,
    }

    pub fn handle(payload: &SetStepCollapsed, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        let mut edit = sequence_scene_edit(doc)?;
        edit.toggle_collapsed(&payload.id);
        Ok(sequence_edit_emit(doc, edit))
    }
}
//#endregion 🔖️SetStepCollapsed

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
