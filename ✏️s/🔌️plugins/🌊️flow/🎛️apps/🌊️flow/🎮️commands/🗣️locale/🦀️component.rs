//! 🗣️ Flow play app commands — the host-pushed locale change.
//!
//! Not declared as a manifest action (locale is host-pushed, not a user-facing app action needing a
//! palette entry), which is why its wire keyword stays the bare `"locale"` rather than the kebab-cased
//! `"set-locale"` its command id would suggest — see the `as` literals in `crate::apps::flow`'s
//! `app_commands!` invocation.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        Ok(Emit::config(vec![FlowConfigMutation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{dispatch, flow_app, render};
    use crate::apps::flow::{FlowCommand, FLOW_PLAY_BODY_DOCUMENT};

    #[test]
    fn flow_labels_resolve_native_english_and_german() {
        let mut app = flow_app();
        let english = render(&mut app, FLOW_PLAY_BODY_DOCUMENT);
        assert!(english.contains("Widgets") && english.contains("Synapses"), "english labels: {english}");
        dispatch(&mut app, FlowCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
        let german = render(&mut app, FLOW_PLAY_BODY_DOCUMENT);
        assert!(german.contains("Synapsen"), "german labels: {german}");
    }
}
//#endregion 🧪️Tests
