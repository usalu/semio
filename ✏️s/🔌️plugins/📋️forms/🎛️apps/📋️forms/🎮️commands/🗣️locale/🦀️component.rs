//! 🗣️ Forms play app commands — the host-pushed locale change.
//!
//! Not declared as a manifest action (locale is host-pushed, not a user-facing app action needing a
//! palette entry), which is why its wire keyword stays the bare `"locale"` rather than the kebab-cased
//! `"set-locale"` its command id would suggest — see the `as` literal in `crate::apps::forms`'s
//! `app_commands!` invocation.

use crate::apps::forms::config::{FormsConfig, FormsConfigMutation};
use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
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

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        Ok(Emit::config(vec![FormsConfigMutation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::forms::testkit::{dispatch, forms_app, render};
    use crate::apps::forms::{FormsCommand, FORMS_PLAY_BODY_BLUEPRINT};

    #[test]
    fn forms_labels_resolve_native_english_and_german() {
        let mut app = forms_app();
        let english = render(&mut app, FORMS_PLAY_BODY_BLUEPRINT);
        assert!(english.contains("Boolean"), "english labels: {english}");
        dispatch(&mut app, FormsCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
        let german = render(&mut app, FORMS_PLAY_BODY_BLUEPRINT);
        assert!(german.contains("Boolescher Wert"), "german labels: {german}");
    }
}
//#endregion 🧪️Tests
