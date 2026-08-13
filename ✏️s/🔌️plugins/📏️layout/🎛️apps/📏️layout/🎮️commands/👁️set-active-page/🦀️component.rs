//! 👁️ 👁️ Layout play app commands command — `set-active-page`.

use crate::apps::layout::config::{LayoutConfig, LayoutConfigMutation};
use crate::artifacts::layout::{op::LayoutMutation, LayoutSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "active-page")]
pub struct SetActivePage {
    pub page_id: String,
}

pub fn handle(payload: &SetActivePage, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    Ok(Emit::config(vec![LayoutConfigMutation::SetActivePage { page_id: payload.page_id.clone() }]))
}
