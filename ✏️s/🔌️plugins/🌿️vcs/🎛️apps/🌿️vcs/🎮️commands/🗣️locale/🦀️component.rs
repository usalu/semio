//! 🗣️ VCS play app commands — the host-pushed locale change.
//!
//! Not declared as a manifest action (locale is host-pushed, not a user-facing app action needing a
//! palette entry), which is why its wire keyword stays the bare `"locale"` rather than the kebab-cased
//! `"set-locale"` its command id would suggest — see the `as` literal in `crate::apps::vcs`'s
//! `app_commands!` invocation.

use crate::apps::vcs::config::{VcsDemoConfig, VcsDemoConfigOperation};
use crate::artifacts::vcs::{op::VcsDemoOperation, VcsDemoProjection};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, VcsDemoProjection>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoOperation, VcsDemoConfigOperation>, Fault> {
        Ok(Emit::config(vec![VcsDemoConfigOperation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::vcs::testkit::{app, dispatch};
    use crate::apps::vcs::VcsCommand;

    #[test]
    fn vcs_demo_command_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&VcsCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
    }

    /// 🗣️ B1: locale is now `cfg.locale`, set via the typed `SetLocale` config command — no more passing
    /// a `ViewState` into `render`/`app_labels` for this purpose (mirrors `shooting_ui`'s identical test).
    #[test]
    fn vcs_labels_resolve_german_locale() {
        use crate::apps::vcs::{VCS_PLAY_BODY_DOCUMENT, VCS_PLAY_BODY_EDITOR, VCS_PLAY_BODY_INSPECTION};
        let mut instance = app();
        dispatch(&mut instance, VcsCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));

        let editor = crate::apps::vcs::testkit::render(&mut instance, VCS_PLAY_BODY_EDITOR);
        assert!(editor.contains("Aktionen"));
        assert!(editor.contains("Rückgängig"));
        assert!(editor.contains("Wiederholen"));
        assert!(editor.contains("Zähler"));

        let inspection = crate::apps::vcs::testkit::render(&mut instance, VCS_PLAY_BODY_INSPECTION);
        assert!(inspection.contains("Titel"));
        assert!(inspection.contains("Notizen"));
        assert!(inspection.contains("Schlagwörter"));

        let document_tree = crate::apps::vcs::testkit::render(&mut instance, VCS_PLAY_BODY_DOCUMENT);
        assert!(document_tree.contains("Alternativen"));
        assert!(document_tree.contains("Checkpoints"));
        assert!(!document_tree.contains("\"Alternatives\""));
    }
}
//#endregion 🧪️Tests
