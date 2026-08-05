//! 🪜️ Sequence play app commands — step CRUD: add/remove/move/patch/collapse a step, delete the
//! current selection.

use crate::apps::sequence::config::{SequenceConfig, SequenceConfigOperation};
use crate::artifacts::sequence::engine::{host_from_fixture, ops_from_host_mutation};
use crate::artifacts::sequence::op::SequenceOperation;
use crate::artifacts::sequence::{SequenceFixture, SlotRef};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
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

    pub fn handle(payload: &AddStep, doc: &DocumentView<'_, SequenceFixture>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceOperation, SequenceConfigOperation>, Fault> {
        let fixture = doc.projection;
        let mut host = host_from_fixture(fixture);
        let id = host.add_step(&payload.kind, payload.x, payload.y);
        Ok(Emit { document_operations: crate::artifacts::sequence::op::sequence_fixture_operations(fixture, &host.fixture), config_operations: vec![SequenceConfigOperation::SetSelection { step_ids: vec![id] }], ..Default::default() })
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

    pub fn handle(payload: &AddStepToSlot, doc: &DocumentView<'_, SequenceFixture>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceOperation, SequenceConfigOperation>, Fault> {
        let fixture = doc.projection;
        let mut host = host_from_fixture(fixture);
        let id = host.add_step_in_slot(&payload.kind, payload.x, payload.y, Some(SlotRef { owner: payload.owner.clone(), name: payload.slot_name.clone() }));
        Ok(Emit { document_operations: crate::artifacts::sequence::op::sequence_fixture_operations(fixture, &host.fixture), config_operations: vec![SequenceConfigOperation::SetSelection { step_ids: vec![id] }], ..Default::default() })
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

    pub fn handle(payload: &AddStepDropped, doc: &DocumentView<'_, SequenceFixture>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceOperation, SequenceConfigOperation>, Fault> {
        let fixture = doc.projection;
        let mut host = host_from_fixture(fixture);
        let id = host.add_step_dropped(&payload.kind, payload.x, payload.y, payload.picked_step_id.as_deref());
        Ok(Emit { document_operations: crate::artifacts::sequence::op::sequence_fixture_operations(fixture, &host.fixture), config_operations: vec![SequenceConfigOperation::SetSelection { step_ids: vec![id] }], ..Default::default() })
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

    pub fn handle(payload: &RemoveStep, doc: &DocumentView<'_, SequenceFixture>, cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceOperation, SequenceConfigOperation>, Fault> {
        let fixture = doc.projection;
        let ops = ops_from_host_mutation(fixture, |host| {
            host.remove_step(&payload.id);
        });
        if ops.is_empty() {
            Ok(Emit::default())
        } else {
            let step_ids = cfg.projection.selected_step_ids.iter().filter(|selected| **selected != payload.id).cloned().collect();
            Ok(Emit { document_operations: ops, config_operations: vec![SequenceConfigOperation::SetSelection { step_ids }], ..Default::default() })
        }
    }
}

pub mod delete_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "delete-selection")]
    pub struct DeleteSelection {}

    pub fn handle(_payload: &DeleteSelection, doc: &DocumentView<'_, SequenceFixture>, cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceOperation, SequenceConfigOperation>, Fault> {
        let fixture = doc.projection;
        let selected = cfg.projection.selected_step_ids.clone();
        let ops = ops_from_host_mutation(fixture, |host| {
            for step_id in &selected {
                host.remove_step(step_id);
            }
        });
        if ops.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit { document_operations: ops, config_operations: vec![SequenceConfigOperation::SetSelection { step_ids: Vec::new() }], ..Default::default() })
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

    pub fn handle(payload: &MoveStep, doc: &DocumentView<'_, SequenceFixture>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceOperation, SequenceConfigOperation>, Fault> {
        let fixture = doc.projection;
        if !fixture.steps.iter().any(|step| step.id == payload.node_id) {
            return Ok(Emit::default());
        }
        Ok(Emit::operations(ops_from_host_mutation(fixture, |host| {
            let mut next = host.fixture.clone();
            if let Some(step) = next.steps.iter_mut().find(|step| step.id == payload.node_id) {
                step.x = payload.x;
                step.y = payload.y;
            }
            let _ = host.replace_fixture(next);
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

    pub fn handle(payload: &SetStepParams, doc: &DocumentView<'_, SequenceFixture>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceOperation, SequenceConfigOperation>, Fault> {
        let fixture = doc.projection;
        Ok(Emit::operations(ops_from_host_mutation(fixture, |host| {
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

    pub fn handle(payload: &SetStepCollapsed, doc: &DocumentView<'_, SequenceFixture>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceOperation, SequenceConfigOperation>, Fault> {
        let fixture = doc.projection;
        let collapsed = fixture.steps.iter().find(|step| step.id == payload.id).map(|step| !step.collapsed).unwrap_or(true);
        Ok(Emit::operations(ops_from_host_mutation(fixture, |host| {
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
        assert!(app.projection().expect("projection").steps.len() > 2);
    }

    #[test]
    fn remove_step_command_deletes_step() {
        let mut app = new_app();
        let step_id = app.projection().expect("projection").steps[0].id.clone();
        dispatch(&mut app, SequenceCommand::RemoveStep(RemoveStep { id: step_id.clone() }));
        assert!(app.projection().expect("projection").steps.iter().all(|step| step.id != step_id));
    }
}
//#endregion 🧪️Tests
