//! 👁️ 👁️ Flow play app commands command — `open-spotlight`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🔦️ Opening the spotlight is renderer chrome; the commit comes back as `spotlightCommit`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct OpenSpotlight {}

pub async fn handle(_payload: &OpenSpotlight, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    Ok(Emit::default())
}
