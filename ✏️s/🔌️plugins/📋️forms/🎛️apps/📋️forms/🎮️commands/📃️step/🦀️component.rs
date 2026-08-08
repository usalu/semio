//! 📃️ Forms play app commands — step lifecycle (add / patch / remove / move) and the form title.

use crate::apps::forms::config::{FormsConfig, FormsConfigMutation};
use crate::apps::forms::reset_try_config_mutations;
use crate::artifacts::forms::engine::create_form_id;
use crate::artifacts::forms::{op::FormMutation, FormSpec, FormStep};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️AddStep
pub mod add_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-step")]
    pub struct AddStep {}

    pub fn handle(_payload: &AddStep, doc: &DocumentView<'_, FormSpec>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        let spec = doc.projection;
        let step = FormStep { id: create_form_id("step"), title: format!("Step {}", spec.steps.len() + 1), description: None, blocks: Vec::new() };
        Ok(Emit { document_mutations: vec![FormMutation::AddStep { step, index: None }], config_mutations: reset_try_config_mutations(), ..Default::default() })
    }
}
//#endregion 🔖️AddStep

//#region 🔖️PatchStep
pub mod patch_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-step")]
    pub struct PatchStep {
        pub step_id: String,
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchStep, doc: &DocumentView<'_, FormSpec>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        let spec = doc.projection;
        let Some(step) = spec.steps.iter().find(|step| step.id == payload.step_id).cloned() else {
            return Ok(Emit::default());
        };
        let step = match payload.field.as_str() {
            "title" => FormStep { title: payload.value.clone(), ..step },
            "description" => FormStep { description: Some(payload.value.clone()).filter(|description| !description.is_empty()), ..step },
            _ => return Ok(Emit::default()),
        };
        Ok(Emit { document_mutations: vec![FormMutation::UpdateStep { step }], config_mutations: reset_try_config_mutations(), coalesce_key: Some(format!("patch-step:{}:{}", payload.step_id, payload.field)), ..Default::default() })
    }
}
//#endregion 🔖️PatchStep

//#region 🔖️RemoveStep
pub mod remove_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-step")]
    pub struct RemoveStep {
        pub step_id: String,
    }

    pub fn handle(payload: &RemoveStep, doc: &DocumentView<'_, FormSpec>, cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        if payload.step_id.is_empty() {
            return Ok(Emit::default());
        }
        let spec = doc.projection;
        let config = cfg.projection;
        let removed_ids: Vec<String> = spec.steps.iter().filter(|step| step.id == payload.step_id).flat_map(|step| step.blocks.iter().map(|question| question.id.clone())).collect();
        let mut config_mutations = reset_try_config_mutations();
        config_mutations.push(FormsConfigMutation::SetSelection { ids: config.selected_ids.iter().filter(|id| !removed_ids.contains(id)).cloned().collect() });
        Ok(Emit { document_mutations: vec![FormMutation::RemoveStep { step_id: payload.step_id.clone() }], config_mutations, ..Default::default() })
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
        pub index: u64,
    }

    pub fn handle(payload: &MoveStep, _doc: &DocumentView<'_, FormSpec>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        if payload.step_id.is_empty() {
            return Ok(Emit::default());
        }
        Ok(Emit { document_mutations: vec![FormMutation::MoveStep { step_id: payload.step_id.clone(), index: payload.index as usize }], config_mutations: reset_try_config_mutations(), ..Default::default() })
    }
}
//#endregion 🔖️MoveStep

//#region 🔖️UpdateForm
pub mod update_form {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "update-form")]
    pub struct UpdateForm {
        pub title: String,
    }

    pub fn handle(payload: &UpdateForm, _doc: &DocumentView<'_, FormSpec>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        Ok(Emit { document_mutations: vec![FormMutation::UpdatePlaybook { title: Some(payload.title.clone()).filter(|title| !title.is_empty()) }], coalesce_key: Some("update-playbook".into()), ..Default::default() })
    }
}
//#endregion 🔖️UpdateForm

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use add_step::AddStep;
    use crate::apps::forms::testkit::{dispatch, forms_app};
    use crate::apps::forms::FormsCommand;
    use move_step::MoveStep;
    use patch_step::PatchStep;
    use remove_step::RemoveStep;
    use update_form::UpdateForm;

    #[test]
    fn add_step_action_appends_step() {
        let mut app = forms_app();
        let before = app.projection().expect("projection").steps.len();
        dispatch(&mut app, FormsCommand::AddStep(AddStep {}));
        assert_eq!(app.projection().expect("projection").steps.len(), before + 1);
    }

    #[test]
    fn patch_step_updates_title_and_description() {
        let mut app = forms_app();
        let step_id = app.projection().expect("projection").steps[0].id.clone();
        dispatch(&mut app, FormsCommand::PatchStep(PatchStep { step_id, field: "title".into(), value: "Renamed".into() }));
        assert_eq!(app.projection().expect("projection").steps[0].title, "Renamed");
    }

    #[test]
    fn remove_and_move_step_actions() {
        let mut app = forms_app();
        dispatch(&mut app, FormsCommand::AddStep(AddStep {}));
        let last_step_id = app.projection().expect("projection").steps.last().unwrap().id.clone();
        dispatch(&mut app, FormsCommand::MoveStep(MoveStep { step_id: last_step_id.clone(), index: 0 }));
        assert_eq!(app.projection().expect("projection").steps[0].id, last_step_id);
        dispatch(&mut app, FormsCommand::RemoveStep(RemoveStep { step_id: last_step_id.clone() }));
        assert!(app.projection().expect("projection").steps.iter().all(|step| step.id != last_step_id));
    }

    #[test]
    fn update_form_action_sets_title() {
        let mut app = forms_app();
        dispatch(&mut app, FormsCommand::UpdateForm(UpdateForm { title: "My Form".into() }));
        assert_eq!(app.projection().expect("projection").title.as_deref(), Some("My Form"));
    }
}
//#endregion 🧪️Tests
