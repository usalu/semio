//! 🗣️ 🗣️ Forms play app commands command — `set-locale`.

use crate::apps::forms::config::{FormsConfig, FormsConfigMutation};
use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    Ok(Emit::config(vec![FormsConfigMutation::SetLocale { value: payload.value.clone() }]))
}

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
        dispatch(&mut app, FormsCommand::SetLocale(SetLocale { value: "de-DE".into() }));
        let german = render(&mut app, FORMS_PLAY_BODY_BLUEPRINT);
        assert!(german.contains("Boolescher Wert"), "german labels: {german}");
    }
}
//#endregion 🧪️Tests
