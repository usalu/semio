//! 🪜️ Sequence play app commands — step CRUD: add/remove/move/patch/collapse a step, delete the
//! current selection.

use crate::editor::sequence::config::{SequenceConfig, SequenceConfigMutation};
use crate::editor::sequence::{host_from_snapshot, ops_from_host_mutation};
use crate::mutations::SequenceMutation;
use crate::{SequenceSnapshot, SlotRef};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️AddStep
pub mod add_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "add-step")]
    pub struct AddStep {
        pub kind: String,
        pub x: f64,
        pub y: f64,
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: auto-selecting the just-added
    /// step is no longer reachable from this dispatch — selection is framework-owned now, written
    /// only through the injected `interactionSelect` verb.
    pub fn handle(payload: &AddStep, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let mut host = host_from_snapshot(fixture);
        let _id = host.add_step(&payload.kind, payload.x, payload.y);
        Ok(Emit::mutations(crate::op::sequence_snapshot_mutations(&fixture.to_fixture(), &host.snapshot)))
    }
}

pub mod add_step_to_slot {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
    pub fn handle(payload: &AddStepToSlot, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let mut host = host_from_snapshot(fixture);
        let _id = host.add_step_in_slot(&payload.kind, payload.x, payload.y, Some(SlotRef { owner: payload.owner.clone(), name: payload.slot_name.clone() }));
        Ok(Emit::mutations(crate::op::sequence_snapshot_mutations(&fixture.to_fixture(), &host.snapshot)))
    }
}

pub mod add_step_dropped {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
    pub fn handle(payload: &AddStepDropped, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let mut host = host_from_snapshot(fixture);
        let _id = host.add_step_dropped(&payload.kind, payload.x, payload.y, payload.picked_step_id.as_deref());
        Ok(Emit::mutations(crate::op::sequence_snapshot_mutations(&fixture.to_fixture(), &host.snapshot)))
    }
}
//#endregion 🔖️AddStep

//#region 🔖️RemoveStep
pub mod remove_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "remove-step")]
    pub struct RemoveStep {
        pub id: String,
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no longer prunes the removed id
    /// out of a config selection field — the framework auto-prunes a deleted step's id out of the
    /// "steps" domain's live selection via `interaction_topology` after this dispatch lands.
    pub fn handle(payload: &RemoveStep, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let ops = ops_from_host_mutation(fixture, |host| {
            host.remove_step(&payload.id);
        });
        Ok(Emit::mutations(ops))
    }
}

pub mod delete_selection {
    use super::*;
    use semio_framework_plugin::app::InteractionView;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "delete-selection")]
    pub struct DeleteSelection {}

    fn delete_selected(fixture: &SequenceSnapshot, selected: &[String]) -> Emit<SequenceMutation, SequenceConfigMutation> {
        let ops = ops_from_host_mutation(fixture, |host| {
            for step_id in selected {
                host.remove_step(step_id);
            }
        });
        Emit::mutations(ops)
    }

    /// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg).await` is framework-fixed at this exact 3-arg
    /// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
    /// reachable only through that macro-generated path (`SequencePlayApp::handle` always routes this
    /// command through `apply` below instead), so it degrades to treating the selection as empty.
    pub fn handle(_payload: &DeleteSelection, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        Ok(delete_selected(doc.snapshot, &[]))
    }

    pub fn apply(_payload: &DeleteSelection, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>, interaction: &InteractionView<'_>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        Ok(delete_selected(doc.snapshot, &interaction.selection(crate::editor::sequence::SEQUENCE_INTERACTION_STEPS).ids))
    }
}
//#endregion 🔖️RemoveStep

//#region 🔖️MoveStep
pub mod move_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "move-step")]
    pub struct MoveStep {
        pub node_id: String,
        pub x: f64,
        pub y: f64,
    }

    pub fn handle(payload: &MoveStep, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        if !fixture.to_fixture().steps.iter().any(|step| step.id == payload.node_id) {
            return Ok(Emit::default());
        }
        Ok(Emit::mutations(ops_from_host_mutation(fixture, |host| {
            let mut next = host.snapshot.clone();
            if let Some(step) = next.steps.iter_mut().find(|step| step.id == payload.node_id) {
                step.x = payload.x;
                step.y = payload.y;
            }
            let _ = host.replace_snapshot(next);
        })))
    }
}
//#endregion 🔖️MoveStep

//#region 🔖️SetStepParams
pub mod set_step_params {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-step-params")]
    pub struct SetStepParams {
        pub id: String,
        pub params_json: String,
    }

    pub fn handle(payload: &SetStepParams, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        Ok(Emit::mutations(ops_from_host_mutation(fixture, |host| {
            let _ = host.set_step_params_json(&payload.id, &payload.params_json);
        })))
    }
}
//#endregion 🔖️SetStepParams

//#region 🔖️SetStepCollapsed
pub mod set_step_collapsed {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-step-collapsed")]
    pub struct SetStepCollapsed {
        pub id: String,
    }

    pub fn handle(payload: &SetStepCollapsed, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let collapsed = fixture.to_fixture().steps.iter().find(|step| step.id == payload.id).is_none_or(|step| !step.collapsed);
        Ok(Emit::mutations(ops_from_host_mutation(fixture, |host| {
            host.set_step_collapsed(&payload.id, collapsed);
        })))
    }
}
//#endregion 🔖️SetStepCollapsed

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
