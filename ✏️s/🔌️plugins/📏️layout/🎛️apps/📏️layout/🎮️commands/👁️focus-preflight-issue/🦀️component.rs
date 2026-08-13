//! 👁️ 👁️ Layout play app commands command — `focus-preflight-issue`.

use crate::apps::layout::config::{LayoutConfig, LayoutConfigMutation};
use crate::artifacts::layout::{op::LayoutMutation, LayoutSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "focus-preflight-issue")]
pub struct FocusPreflightIssue {
    pub object_id: Option<String>,
    pub page_id: Option<String>,
}

pub fn handle(payload: &FocusPreflightIssue, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    let mut config_mutations = Vec::new();
    if let Some(object_id) = &payload.object_id {
        config_mutations.push(LayoutConfigMutation::SetSelection { ids: vec![object_id.clone()] });
    }
    if let Some(page_id) = &payload.page_id {
        config_mutations.push(LayoutConfigMutation::SetActivePage { page_id: page_id.clone() });
    }
    Ok(Emit::config(config_mutations))
}
