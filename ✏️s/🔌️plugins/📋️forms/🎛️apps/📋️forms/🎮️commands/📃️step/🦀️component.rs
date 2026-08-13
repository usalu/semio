//! 📃️ Forms play app commands — step lifecycle (add / patch / remove / move) and the form title.

use crate::apps::forms::config::{FormsConfig, FormsConfigMutation};
use crate::apps::forms::reset_try_config_mutations;
use crate::artifacts::forms::schema::create_form_id;
use crate::artifacts::forms::{forms_steps, op::FormMutation, FormsSnapshot, FormStep};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️AddStep
pub mod add_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-step")]
    pub struct AddStep {}

    pub fn handle(_payload: &AddStep, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        let spec = doc.snapshot;
        let step = FormStep { id: create_form_id("step"), title: format!("Step {}", forms_steps(spec).len() + 1), description: None, blocks: Vec::new() };
        Ok(Emit {
            artifact_mutations: vec![FormMutation::CreateStep(crate::artifacts::forms::mutations::create_step::mutation::CreateStep { step, index: None })],
            config_mutations: reset_try_config_mutations(),
            ..Default::default()
        })
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

    /// ✏️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM: `FormMutation` has no whole-step-replace
    /// variant (the old `UpdateStep{step}` is banned `SetSnapshot`-shaped vocabulary at the
    /// per-collection scale) — emits the granular `RenameStep`/`ChangeStepDescription` verb the field
    /// actually maps onto instead of building a whole replacement `FormStep`.
    pub fn handle(payload: &PatchStep, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        let spec = doc.snapshot;
        if !forms_steps(spec).iter().any(|step| step.id == payload.step_id) {
            return Ok(Emit::default());
        }
        let mutation = match payload.field.as_str() {
            "title" => FormMutation::RenameStep(crate::artifacts::forms::mutations::rename_step::mutation::RenameStep { id: payload.step_id.clone(), new_title: payload.value.clone() }),
            "description" => FormMutation::ChangeStepDescription(crate::artifacts::forms::mutations::change_step_description::mutation::ChangeStepDescription {
                id: payload.step_id.clone(),
                new_description: Some(payload.value.clone()).filter(|description| !description.is_empty()),
            }),
            _ => return Ok(Emit::default()),
        };
        Ok(Emit { artifact_mutations: vec![mutation], config_mutations: reset_try_config_mutations(), coalesce_key: Some(format!("patch-step:{}:{}", payload.step_id, payload.field)), ..Default::default() })
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

    pub fn handle(payload: &RemoveStep, doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        if payload.step_id.is_empty() {
            return Ok(Emit::default());
        }
        let spec = doc.snapshot;
        let config = cfg.snapshot;
        let removed_ids: Vec<String> = forms_steps(spec).iter().filter(|step| step.id == payload.step_id).flat_map(|step| step.blocks.iter().map(|question| question.id.clone())).collect();
        let mut config_mutations = reset_try_config_mutations();
        config_mutations.push(FormsConfigMutation::SetSelection { ids: config.selected_ids.iter().filter(|id| !removed_ids.contains(id)).cloned().collect() });
        Ok(Emit { artifact_mutations: vec![FormMutation::DeleteStep(crate::artifacts::forms::mutations::delete_step::mutation::DeleteStep { id: payload.step_id.clone() })], config_mutations, ..Default::default() })
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

    pub fn handle(payload: &MoveStep, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        if payload.step_id.is_empty() {
            return Ok(Emit::default());
        }
        Ok(Emit {
            artifact_mutations: vec![FormMutation::ReorderStep(crate::artifacts::forms::mutations::reorder_step::mutation::ReorderStep { id: payload.step_id.clone(), to_index: payload.index as usize })],
            config_mutations: reset_try_config_mutations(),
            ..Default::default()
        })
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

    pub fn handle(payload: &UpdateForm, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        Ok(Emit {
            artifact_mutations: vec![FormMutation::ChangeFormTitle(crate::artifacts::forms::mutations::change_form_title::mutation::ChangeFormTitle { new_title: Some(payload.title.clone()).filter(|title| !title.is_empty()) })],
            coalesce_key: Some("change-form-title".into()),
            ..Default::default()
        })
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
        let before = forms_steps(&app.snapshot().expect("projection")).len();
        dispatch(&mut app, FormsCommand::AddStep(AddStep {}));
        assert_eq!(forms_steps(&app.snapshot().expect("projection")).len(), before + 1);
    }

    #[test]
    fn patch_step_updates_title_and_description() {
        let mut app = forms_app();
        let step_id = forms_steps(&app.snapshot().expect("projection"))[0].id.clone();
        dispatch(&mut app, FormsCommand::PatchStep(PatchStep { step_id, field: "title".into(), value: "Renamed".into() }));
        assert_eq!(forms_steps(&app.snapshot().expect("projection"))[0].title, "Renamed");
    }

    #[test]
    fn remove_and_move_step_actions() {
        let mut app = forms_app();
        dispatch(&mut app, FormsCommand::AddStep(AddStep {}));
        let last_step_id = forms_steps(&app.snapshot().expect("projection")).last().unwrap().id.clone();
        dispatch(&mut app, FormsCommand::MoveStep(MoveStep { step_id: last_step_id.clone(), index: 0 }));
        assert_eq!(forms_steps(&app.snapshot().expect("projection"))[0].id, last_step_id);
        dispatch(&mut app, FormsCommand::RemoveStep(RemoveStep { step_id: last_step_id.clone() }));
        assert!(forms_steps(&app.snapshot().expect("projection")).iter().all(|step| step.id != last_step_id));
    }

    #[test]
    fn update_form_action_sets_title() {
        let mut app = forms_app();
        dispatch(&mut app, FormsCommand::UpdateForm(UpdateForm { title: "My Form".into() }));
        assert_eq!(app.snapshot().expect("projection").title.as_deref(), Some("My Form"));
    }
}
//#endregion 🧪️Tests
