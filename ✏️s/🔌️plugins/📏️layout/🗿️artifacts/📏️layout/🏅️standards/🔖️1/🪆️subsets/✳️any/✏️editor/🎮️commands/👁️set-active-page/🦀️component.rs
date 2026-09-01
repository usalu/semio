//! 👁️ 👁️ Layout play app commands command — `set-active-page`.

use crate::artifacts::layout::{op::LayoutMutation, LayoutSnapshot};
use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-page")]
pub struct SetActivePage {
    pub page_id: String,
}

pub async fn handle(payload: &SetActivePage, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    Ok(Emit::config(vec![LayoutConfigMutation::SetActivePage { page_id: payload.page_id.clone() }]))
}
