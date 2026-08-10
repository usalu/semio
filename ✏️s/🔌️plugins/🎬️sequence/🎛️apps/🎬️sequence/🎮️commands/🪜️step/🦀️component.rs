//! 🪜️ Sequence play app commands — step CRUD: add/remove/move/patch/collapse a step, delete the
//! current selection.

use crate::apps::sequence::config::{SequenceConfig, SequenceConfigMutation};
use crate::artifacts::sequence::engine::{host_from_snapshot, ops_from_host_mutation};
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::{SequenceSnapshot, SlotRef};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️AddStep
pub mod add_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-step")]
    pub struct AddStep {
        pub kind: String,
        pub x: f64,
        pub y: f64,
    }

    pub fn handle(payload: &AddStep, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let mut host = host_from_snapshot(fixture);
        let id = host.add_step(&payload.kind, payload.x, payload.y);
        Ok(Emit { artifact_mutations: crate::artifacts::sequence::op::sequence_snapshot_mutations(fixture, &host.snapshot), config_mutations: vec![SequenceConfigMutation::SetSelection { step_ids: vec![id] }], ..Default::default() })
    }
}

pub mod add_step_to_slot {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-step-to-slot")]
    pub struct AddStepToSlot {
        pub kind: String,
        pub x: f64,
        pub y: f64,
        pub owner: String,
        pub slot_name: String,
    }

    pub fn handle(payload: &AddStepToSlot, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let mut host = host_from_snapshot(fixture);
        let id = host.add_step_in_slot(&payload.kind, payload.x, payload.y, Some(SlotRef { owner: payload.owner.clone(), name: payload.slot_name.clone() }));
        Ok(Emit { artifact_mutations: crate::artifacts::sequence::op::sequence_snapshot_mutations(fixture, &host.snapshot), config_mutations: vec![SequenceConfigMutation::SetSelection { step_ids: vec![id] }], ..Default::default() })
    }
}

pub mod add_step_dropped {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-step-dropped")]
    pub struct AddStepDropped {
        pub kind: String,
        pub x: f64,
        pub y: f64,
        pub picked_step_id: Option<String>,
    }

    pub fn handle(payload: &AddStepDropped, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let mut host = host_from_snapshot(fixture);
        let id = host.add_step_dropped(&payload.kind, payload.x, payload.y, payload.picked_step_id.as_deref());
        Ok(Emit { artifact_mutations: crate::artifacts::sequence::op::sequence_snapshot_mutations(fixture, &host.snapshot), config_mutations: vec![SequenceConfigMutation::SetSelection { step_ids: vec![id] }], ..Default::default() })
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

    pub fn handle(payload: &RemoveStep, doc: &ArtifactView<'_, SequenceSnapshot>, cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let ops = ops_from_host_mutation(fixture, |host| {
            host.remove_step(&payload.id);
        });
        if ops.is_empty() {
            Ok(Emit::default())
        } else {
            let step_ids = cfg.snapshot.selected_step_ids.iter().filter(|selected| **selected != payload.id).cloned().collect();
            Ok(Emit { artifact_mutations: ops, config_mutations: vec![SequenceConfigMutation::SetSelection { step_ids }], ..Default::default() })
        }
    }
}

pub mod delete_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "delete-selection")]
    pub struct DeleteSelection {}

    pub fn handle(_payload: &DeleteSelection, doc: &ArtifactView<'_, SequenceSnapshot>, cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let selected = cfg.snapshot.selected_step_ids.clone();
        let ops = ops_from_host_mutation(fixture, |host| {
            for step_id in &selected {
                host.remove_step(step_id);
            }
        });
        if ops.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit { artifact_mutations: ops, config_mutations: vec![SequenceConfigMutation::SetSelection { step_ids: Vec::new() }], ..Default::default() })
        }
    }
}
//#endregion 🔖️RemoveStep

//#region 🔖️MoveStep
pub mod move_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "move-step")]
    pub struct MoveStep {
        pub node_id: String,
        pub x: f64,
        pub y: f64,
    }

    pub fn handle(payload: &MoveStep, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        if !fixture.steps.iter().any(|step| step.id == payload.node_id) {
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-step-collapsed")]
    pub struct SetStepCollapsed {
        pub id: String,
    }

    pub fn handle(payload: &SetStepCollapsed, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let collapsed = fixture.steps.iter().find(|step| step.id == payload.id).is_none_or(|step| !step.collapsed);
        Ok(Emit::mutations(ops_from_host_mutation(fixture, |host| {
            host.set_step_collapsed(&payload.id, collapsed);
        })))
    }
}
//#endregion 🔖️SetStepCollapsed

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::sequence::testkit::{dispatch, new_app};
    use crate::apps::sequence::SequenceCommand;

    use super::add_step::AddStep;
    use super::remove_step::RemoveStep;

    #[test]
    fn add_step_command_appends_step() {
        let mut app = new_app();
        dispatch(&mut app, SequenceCommand::AddStep(AddStep { kind: "log.print".into(), x: 0.0, y: 0.0 }));
        assert!(app.snapshot().expect("projection").steps.len() > 2);
    }

    #[test]
    fn remove_step_command_deletes_step() {
        let mut app = new_app();
        let step_id = app.snapshot().expect("projection").steps[0].id.clone();
        dispatch(&mut app, SequenceCommand::RemoveStep(RemoveStep { id: step_id.clone() }));
        assert!(app.snapshot().expect("projection").steps.iter().all(|step| step.id != step_id));
    }
}
//#endregion 🧪️Tests
