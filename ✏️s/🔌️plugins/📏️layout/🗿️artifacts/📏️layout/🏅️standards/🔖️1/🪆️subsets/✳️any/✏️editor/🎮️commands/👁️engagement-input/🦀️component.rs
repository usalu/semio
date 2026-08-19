//! 👁️ 👁️ Layout play app commands command — `engagement-input`.

use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use crate::artifacts::layout::{op::LayoutMutation, LayoutSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "engagement-input")]
pub struct EngagementInput {
    pub value: String,
}

pub async fn handle(payload: &EngagementInput, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    Ok(Emit::config(vec![LayoutConfigMutation::SetEngagementInput { value: payload.value.clone() }]))
}
