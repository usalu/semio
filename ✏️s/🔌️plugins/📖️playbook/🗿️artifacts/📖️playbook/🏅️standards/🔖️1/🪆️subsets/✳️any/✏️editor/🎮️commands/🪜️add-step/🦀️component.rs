//! 🪜️ 🪜️ Playbook play app commands command — `add-step`.

use crate::artifacts::playbook::op::{add_step_operation, PlaybookMutation};
use crate::artifacts::playbook::PlaybookSnapshot;
use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-step")]
pub struct AddStep {}

pub async fn handle(_payload: &AddStep, doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    let step_id = format!("step-{}", doc.snapshot.steps().len() + 1);
    Ok(Emit::mutations(vec![add_step_operation(doc.snapshot, step_id)]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::playbook::commands::move_step::MoveStep;
    use crate::editor::playbook::commands::remove_step::RemoveStep;
    use crate::editor::playbook::commands::update_playbook::UpdatePlaybook;
    use crate::editor::playbook::testkit::{dispatch, playbook_app};
    use crate::editor::playbook::PlaybookCommand;
    use semio_framework_plugin::PluginApp;
    use AddStep;

    #[semio_framework_async_macros::async_test]
    async fn add_step_action_appends_step() {
        let mut app = playbook_app();
        let before = app.snapshot().expect("projection").steps().len();
        dispatch(&mut app, PlaybookCommand::AddStep(AddStep {}));
        assert_eq!(app.snapshot().expect("projection").steps().len(), before + 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_and_move_step_actions() {
        let mut app = playbook_app();
        dispatch(&mut app, PlaybookCommand::AddStep(AddStep {}));
        let last_step_id = app.snapshot().expect("projection").steps().last().unwrap().id.clone();
        dispatch(&mut app, PlaybookCommand::MoveStep(MoveStep { step_id: last_step_id.clone(), index: 0 }));
        assert_eq!(app.snapshot().expect("projection").steps()[0].id, last_step_id);
        dispatch(&mut app, PlaybookCommand::RemoveStep(RemoveStep { step_id: last_step_id.clone() }));
        assert!(app.snapshot().expect("projection").steps().iter().all(|step| step.id != last_step_id));
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_step_with_empty_id_is_a_no_op() {
        let mut app = playbook_app();
        let before = app.snapshot().expect("projection").steps().len();
        dispatch(&mut app, PlaybookCommand::RemoveStep(RemoveStep { step_id: String::new() }));
        assert_eq!(app.snapshot().expect("projection").steps().len(), before);
    }

    #[semio_framework_async_macros::async_test]
    async fn update_playbook_title_coalesces_into_one_undo_step() {
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
