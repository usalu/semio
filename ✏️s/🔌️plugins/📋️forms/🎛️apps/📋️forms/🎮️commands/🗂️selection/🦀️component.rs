//! 🗂️ Forms play app commands — blueprint/inspector selection.

use crate::apps::forms::config::{FormsConfig, FormsConfigMutation};
use crate::artifacts::forms::{op::FormMutation, FormSpec};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "selection")]
    pub struct SetSelection {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, FormSpec>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        Ok(Emit::config(vec![FormsConfigMutation::SetSelection { ids: payload.ids.clone() }]))
    }
}
//#endregion 🔖️SetSelection

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::forms::testkit::{dispatch, forms_app, render};
    use crate::apps::forms::{FormsCommand, FORMS_PLAY_BODY_BLUEPRINT};

    #[test]
    fn set_selection_reflects_in_the_blueprint_builder_render() {
        let mut app = forms_app();
        let first_question_id = app.projection().expect("projection").steps[0].blocks[0].id.clone();
        dispatch(&mut app, FormsCommand::SetSelection(set_selection::SetSelection { ids: vec![first_question_id.clone()] }));
        let json = render(&mut app, FORMS_PLAY_BODY_BLUEPRINT);
        assert!(json.contains(&format!(r#""selectedId":"{first_question_id}""#)));
    }
}
//#endregion 🧪️Tests
