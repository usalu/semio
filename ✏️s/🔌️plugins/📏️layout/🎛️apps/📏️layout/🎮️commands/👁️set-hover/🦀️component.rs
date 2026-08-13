//! 👁️ 👁️ Layout play app commands command — `set-hover`.

use crate::apps::layout::config::{LayoutConfig, LayoutConfigMutation};
use crate::artifacts::layout::{op::LayoutMutation, LayoutSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "hover")]
pub struct SetHover {
    pub id: Option<String>,
}

pub fn handle(payload: &SetHover, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    Ok(Emit::config(vec![LayoutConfigMutation::SetHover { id: payload.id.clone() }]))
}
