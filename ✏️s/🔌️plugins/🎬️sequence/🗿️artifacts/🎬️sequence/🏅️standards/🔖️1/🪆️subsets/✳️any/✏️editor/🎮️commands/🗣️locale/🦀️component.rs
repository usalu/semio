//! 🗣️ Sequence play app commands — host-pushed locale.

use crate::editor::sequence::config::{SequenceConfig, SequenceConfigMutation};
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::SequenceSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        Ok(Emit::config(vec![SequenceConfigMutation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::sequence::testkit::{dispatch, new_app, render};
    use crate::editor::sequence::SequenceCommand;

    use super::set_locale::SetLocale;

    #[test]
    fn sequence_labels_render_native_english_by_default() {
        let mut app = new_app();
        let document_json = render(&mut app, crate::editor::sequence::panels::document::SEQUENCE_PLAY_BODY_DOCUMENT);
        assert!(document_json.contains("\"Steps\""));
        assert!(document_json.contains("\"Flow edges\""));
    }

    #[test]
    fn sequence_labels_render_german_locale() {
        let mut app = new_app();
        dispatch(&mut app, SequenceCommand::SetLocale(SetLocale { value: "de".into() }));
        let document_json = render(&mut app, crate::editor::sequence::panels::document::SEQUENCE_PLAY_BODY_DOCUMENT);
        assert!(document_json.contains("Schritte"));
        assert!(document_json.contains("Ablaufkanten"));
        assert!(!document_json.contains("\"Steps\""));
    }
}
//#endregion 🧪️Tests
