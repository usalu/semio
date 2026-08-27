//! 🗣️ 🗣️ Flow play app commands command — `set-locale`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct SetLocale {
    pub value: String,
}

pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    Ok(Emit::config(vec![FlowConfigMutation::SetLocale { value: payload.value.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{dispatch, flow_app, render};
    use crate::editor::flow::{FlowCommand, FLOW_PLAY_BODY_DOCUMENT};

    #[semio_framework_async_macros::async_test]
    async fn flow_labels_resolve_native_english_and_german() {
        let mut app = flow_app();
        let english = render(&mut app, FLOW_PLAY_BODY_DOCUMENT);
        assert!(english.contains("Widgets") && english.contains("Synapses"), "english labels: {english}");
        dispatch(&mut app, FlowCommand::SetLocale(SetLocale { value: "de-DE".into() }));
        let german = render(&mut app, FLOW_PLAY_BODY_DOCUMENT);
        assert!(german.contains("Synapsen"), "german labels: {german}");
    }
}
//#endregion 🧪️Tests
