//! 🗣️ 🗣️ VCS play app commands command — `set-locale`.

use crate::artifacts::vcs::{op::VcsDemoMutation, VcsSnapshot};
use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub async fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
    Ok(Emit::config(vec![VcsDemoConfigMutation::SetLocale { value: payload.value.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::vcs::testkit::{app, dispatch};
    use crate::editor::vcs::VcsCommand;

    #[semio_framework_async_macros::async_test]
    async fn vcs_demo_command_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::SetLocale(SetLocale { value: "de-DE".into() }));
    }

    /// 🗣️ B1: locale is now `cfg.locale`, set via the typed `SetLocale` config command — no more passing
    /// a `ViewModel` into `render`/`app_labels` for this purpose (mirrors `shooting_ui`'s identical test).
    #[semio_framework_async_macros::async_test]
    async fn vcs_labels_resolve_german_locale() {
        use crate::editor::vcs::{VCS_PLAY_BODY_DOCUMENT, VCS_PLAY_BODY_EDITOR, VCS_PLAY_BODY_INSPECTION};
        let mut instance = app();
        dispatch(&mut instance, VcsCommand::SetLocale(SetLocale { value: "de-DE".into() }));

        let editor = crate::editor::vcs::testkit::render(&mut instance, VCS_PLAY_BODY_EDITOR);
        assert!(editor.contains("Aktionen"));
        assert!(editor.contains("Rückgängig"));
        assert!(editor.contains("Wiederholen"));
        assert!(editor.contains("Zähler"));

        let inspection = crate::editor::vcs::testkit::render(&mut instance, VCS_PLAY_BODY_INSPECTION);
        assert!(inspection.contains("Titel"));
        assert!(inspection.contains("Notizen"));
        assert!(inspection.contains("Schlagwörter"));

        let document_tree = crate::editor::vcs::testkit::render(&mut instance, VCS_PLAY_BODY_DOCUMENT);
        assert!(document_tree.contains("Alternativen"));
        assert!(document_tree.contains("Checkpoints"));
        assert!(!document_tree.contains("\"Alternatives\""));
    }
}
//#endregion 🧪️Tests
