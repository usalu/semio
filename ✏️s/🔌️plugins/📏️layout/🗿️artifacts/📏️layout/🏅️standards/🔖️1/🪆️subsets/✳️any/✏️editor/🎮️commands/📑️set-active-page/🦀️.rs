//! 👁️ 👁️ Layout play app commands command — `set-active-page`.

use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use crate::{op::LayoutMutation, LayoutSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-page")]
pub struct SetActivePage {
    pub page_id: String,
}

pub fn handle(payload: &SetActivePage, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    Ok(Emit::config(vec![LayoutConfigMutation::SetActivePage(crate::editor::layout::config::SetActivePage { page_id: payload.page_id.clone() })]))
}
