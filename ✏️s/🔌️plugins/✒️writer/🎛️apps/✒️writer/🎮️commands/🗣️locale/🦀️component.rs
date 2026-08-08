//! 🗣️ Writer play app commands — the host-pushed locale change.
//!
//! Not declared as a manifest action (locale is host-pushed, not a user-facing app action needing a
//! palette entry), which is why its wire keyword stays the bare `"locale"` rather than the kebab-cased
//! `"set-locale"` its command id would suggest — see the `as` literals in `crate::apps::writer`'s
//! `app_commands!` invocation.

use crate::apps::writer::config::{WriterConfig, WriterConfigMutation};
use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::WriterProjection;
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

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, WriterProjection>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
        Ok(Emit::config(vec![WriterConfigMutation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::set_locale;
    use crate::apps::writer::testkit::{dispatch, new_app, render};
    use crate::apps::writer::{WriterCommand, WRITER_PLAY_BODY_INSPECTION};

    #[test]
    fn writer_labels_resolve_native_english_and_german() {
        let mut app = new_app();
        let english = render(&mut app, WRITER_PLAY_BODY_INSPECTION);
        assert!(english.contains("\"Document\"") && english.contains("\"Camera\""), "english labels: {english}");
        dispatch(&mut app, WriterCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
        let german = render(&mut app, WRITER_PLAY_BODY_INSPECTION);
        assert!(german.contains("Dokument") && german.contains("Kamera"), "german labels: {german}");
    }
}
//#endregion 🧪️Tests
