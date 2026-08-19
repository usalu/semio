//! 👁️ 👁️ Flow play app commands command — `replace-image`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🖼️ Opening the host file picker is renderer chrome; the picked media returns as a widget patch.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct ReplaceImage {
    pub id: String,
}

pub async fn handle(_payload: &ReplaceImage, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    Ok(Emit::default())
}
