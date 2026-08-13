//! 🗂️ 🗂️ Playbook play app commands command — `set-selection`.

use crate::apps::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::artifacts::playbook::{op::PlaybookMutation, PlaybookSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-selection")]
pub struct SetSelection {
    pub ids: Vec<String>,
}

pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    Ok(Emit::config(vec![PlaybookConfigMutation::SetSelectedIds { ids: payload.ids.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::playbook::testkit::{dispatch, playbook_app, render};
    use crate::apps::playbook::{PlaybookCommand, PLAYBOOK_PLAY_BODY_BUILDER};

    #[test]
    fn set_selection_is_a_view_command_without_operations() {
        let mut app = playbook_app();
        let result = app.dispatch_typed(PlaybookCommand::SetSelection(SetSelection { ids: vec!["block-1".into()] }), &semio_framework_plugin::testkit::meta("local")).expect("set selection");
        assert!(result.mutations.is_empty(), "selection is ephemeral config state, not a document operation");
    }

    #[test]
    fn set_selection_reflects_in_the_builder_render() {
        let mut app = playbook_app();
        dispatch(&mut app, PlaybookCommand::AddBlock(crate::apps::playbook::commands::add_block::AddBlock { kind: "text".into(), step_id: None }));
        let block_id = app.snapshot().expect("projection").steps()[0].blocks[0].id.clone();
        dispatch(&mut app, PlaybookCommand::SetSelection(SetSelection { ids: vec![block_id.clone()] }));
        let json = render(&mut app, PLAYBOOK_PLAY_BODY_BUILDER);
        assert!(json.contains(&format!(r#""selectedId":"{block_id}""#)));
    }
}
//#endregion 🧪️Tests
