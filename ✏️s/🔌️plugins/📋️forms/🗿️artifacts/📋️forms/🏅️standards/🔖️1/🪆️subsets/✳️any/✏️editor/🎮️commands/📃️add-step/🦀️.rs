//! 📃️ 📃️ Forms play app commands command — `add-step`.

use crate::artifacts::forms::schema::create_form_id;
use crate::artifacts::forms::{forms_steps, op::FormMutation, FormStep, FormsSnapshot};
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::reset_try_config_mutations;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-step")]
pub struct AddStep {}

pub async fn handle(_payload: &AddStep, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let spec = doc.snapshot;
    let step = FormStep { id: create_form_id("step"), title: format!("Step {}", forms_steps(spec).len() + 1), description: None, blocks: Vec::new() };
    Ok(Emit { artifact_mutations: vec![FormMutation::CreateStep(crate::artifacts::forms::mutations::create_step::mutation::CreateStep { step, index: None })], config_mutations: reset_try_config_mutations(), ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::forms::commands::move_step::MoveStep;
    use crate::editor::forms::commands::patch_step::PatchStep;
    use crate::editor::forms::commands::remove_step::RemoveStep;
    use crate::editor::forms::commands::update_form::UpdateForm;
    use crate::editor::forms::testkit::{dispatch, forms_app};
    use crate::editor::forms::FormsCommand;
    use AddStep;

    #[semio_framework_async_macros::async_test]
    async fn add_step_action_appends_step() {
        let mut app = forms_app();
        let before = forms_steps(&app.snapshot().expect("projection")).len();
        dispatch(&mut app, FormsCommand::AddStep(AddStep {}));
        assert_eq!(forms_steps(&app.snapshot().expect("projection")).len(), before + 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn patch_step_updates_title_and_description() {
        let mut app = forms_app();
        let step_id = forms_steps(&app.snapshot().expect("projection"))[0].id.clone();
        dispatch(&mut app, FormsCommand::PatchStep(PatchStep { step_id, field: "title".into(), value: "Renamed".into() }));
        assert_eq!(forms_steps(&app.snapshot().expect("projection"))[0].title, "Renamed");
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_and_move_step_actions() {
        let mut app = forms_app();
        dispatch(&mut app, FormsCommand::AddStep(AddStep {}));
        let last_step_id = forms_steps(&app.snapshot().expect("projection")).last().unwrap().id.clone();
        dispatch(&mut app, FormsCommand::MoveStep(MoveStep { step_id: last_step_id.clone(), index: 0 }));
        assert_eq!(forms_steps(&app.snapshot().expect("projection"))[0].id, last_step_id);
        dispatch(&mut app, FormsCommand::RemoveStep(RemoveStep { step_id: last_step_id.clone() }));
        assert!(forms_steps(&app.snapshot().expect("projection")).iter().all(|step| step.id != last_step_id));
    }

    #[semio_framework_async_macros::async_test]
    async fn update_form_action_sets_title() {
        let mut app = forms_app();
        dispatch(&mut app, FormsCommand::UpdateForm(UpdateForm { title: "My Form".into() }));
        assert_eq!(app.snapshot().expect("projection").title.as_deref(), Some("My Form"));
    }
}
//#endregion 🧪️Tests
