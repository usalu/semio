//! 🗂️ Sequence play app commands — selection.

use crate::apps::sequence::config::{SequenceConfig, SequenceConfigMutation};
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::SequenceFixture;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selection")]
    pub struct SetSelection {
        pub step_ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, SequenceFixture>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        Ok(Emit::config(vec![SequenceConfigMutation::SetSelection { step_ids: payload.step_ids.clone() }]))
    }
}
//#endregion 🔖️SetSelection

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::sequence::testkit::{dispatch, new_app};
    use crate::apps::sequence::SequenceCommand;

    use super::set_selection::SetSelection;

    #[test]
    fn set_selection_writes_config_selection() {
        let mut app = new_app();
        dispatch(&mut app, SequenceCommand::SetSelection(SetSelection { step_ids: vec!["step-1".into()] }));
        let node = crate::apps::sequence::testkit::render(&mut app, crate::apps::sequence::panels::document::SEQUENCE_PLAY_BODY_DOCUMENT);
        assert!(node.contains("step-1"));
    }
}
//#endregion 🧪️Tests
