//! 🪜️ Playbook play app commands — step lifecycle (add / remove / move) and the playbook title.

use crate::apps::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::artifacts::playbook::op::{add_step_operation, move_step_operation, remove_step_operation, update_playbook_title_operation, PlaybookMutation};
use crate::artifacts::playbook::PlaybookSnapshot;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️AddStep
pub mod add_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-step")]
    pub struct AddStep {}

    pub fn handle(_payload: &AddStep, doc: &DocumentView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
        let kernel = doc.snapshot.as_kernel();
        let step_id = format!("step-{}", kernel.steps.len() + 1);
        Ok(Emit::mutations(vec![add_step_operation(&kernel, step_id)]))
    }
}
//#endregion 🔖️AddStep

//#region 🔖️RemoveStep
pub mod remove_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-step")]
    pub struct RemoveStep {
        pub step_id: String,
    }

    pub fn handle(payload: &RemoveStep, _doc: &DocumentView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
        if payload.step_id.is_empty() {
            return Ok(Emit::default());
        }
        Ok(Emit::mutations(vec![remove_step_operation(&payload.step_id)]))
    }
}
//#endregion 🔖️RemoveStep

//#region 🔖️MoveStep
pub mod move_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "move-step")]
    pub struct MoveStep {
        pub step_id: String,
        pub index: usize,
    }

    pub fn handle(payload: &MoveStep, _doc: &DocumentView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
        if payload.step_id.is_empty() {
            return Ok(Emit::default());
        }
        Ok(Emit::mutations(vec![move_step_operation(&payload.step_id, payload.index)]))
    }
}
//#endregion 🔖️MoveStep

//#region 🔖️UpdatePlaybook
pub mod update_playbook {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "update-playbook")]
    pub struct UpdatePlaybook {
        pub value: String,
    }

    pub fn handle(payload: &UpdatePlaybook, _doc: &DocumentView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
        Ok(Emit::amend(vec![update_playbook_title_operation(Some(payload.value.clone()).filter(|title| !title.is_empty()))], "playbook.title"))
    }
}
//#endregion 🔖️UpdatePlaybook

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use add_step::AddStep;
    use crate::apps::playbook::testkit::{dispatch, playbook_app};
    use crate::apps::playbook::PlaybookCommand;
    use move_step::MoveStep;
    use remove_step::RemoveStep;
    use semio_framework_plugin::PluginApp;
    use update_playbook::UpdatePlaybook;

    #[test]
    fn add_step_action_appends_step() {
        let mut app = playbook_app();
        let before = app.snapshot().expect("projection").steps.len();
        dispatch(&mut app, PlaybookCommand::AddStep(AddStep {}));
        assert_eq!(app.snapshot().expect("projection").steps.len(), before + 1);
    }

    #[test]
    fn remove_and_move_step_actions() {
        let mut app = playbook_app();
        dispatch(&mut app, PlaybookCommand::AddStep(AddStep {}));
        let last_step_id = app.snapshot().expect("projection").steps.last().unwrap().id.clone();
        dispatch(&mut app, PlaybookCommand::MoveStep(MoveStep { step_id: last_step_id.clone(), index: 0 }));
        assert_eq!(app.snapshot().expect("projection").steps[0].id, last_step_id);
        dispatch(&mut app, PlaybookCommand::RemoveStep(RemoveStep { step_id: last_step_id.clone() }));
        assert!(app.snapshot().expect("projection").steps.iter().all(|step| step.id != last_step_id));
    }

    #[test]
    fn remove_step_with_empty_id_is_a_no_op() {
        let mut app = playbook_app();
        let before = app.snapshot().expect("projection").steps.len();
        dispatch(&mut app, PlaybookCommand::RemoveStep(RemoveStep { step_id: String::new() }));
        assert_eq!(app.snapshot().expect("projection").steps.len(), before);
    }

    #[test]
    fn update_playbook_title_coalesces_into_one_undo_step() {
        let mut app = playbook_app();
        for title in ["R", "Re", "Recipe"] {
            dispatch(&mut app, PlaybookCommand::UpdatePlaybook(UpdatePlaybook { value: title.into() }));
        }
        assert_eq!(app.snapshot().expect("projection").title.as_deref(), Some("Recipe"));
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        assert_eq!(app.snapshot().expect("projection").title, None, "coalesced typing is one undo step");
    }
}
//#endregion 🧪️Tests
