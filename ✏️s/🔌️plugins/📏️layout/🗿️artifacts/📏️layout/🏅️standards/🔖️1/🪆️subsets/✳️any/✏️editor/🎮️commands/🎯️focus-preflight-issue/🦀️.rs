//! 👁️ 👁️ Layout play app commands command — `focus-preflight-issue`.

use crate::{op::LayoutMutation, LayoutSnapshot};
use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "focus-preflight-issue")]
pub struct FocusPreflightIssue {
    pub object_id: Option<String>,
    pub page_id: Option<String>,
}

/// 👁️ `object_id` used to write straight into `LayoutConfigMutation::SetSelection`; selection is
/// framework-owned now (domain "elements"), so a hit asks the host to redispatch `interactionSelect`
/// via an effect instead — see `crate::editor::layout::layout_select_effect`.
pub fn handle(payload: &FocusPreflightIssue, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    let mut config_mutations = Vec::new();
    let mut effects = Vec::new();
    if let Some(object_id) = &payload.object_id {
        effects.push(crate::editor::layout::layout_select_effect(std::slice::from_ref(object_id), "replace"));
    }
    if let Some(page_id) = &payload.page_id {
        config_mutations.push(LayoutConfigMutation::SetActivePage(crate::editor::layout::config::SetActivePage { page_id: page_id.clone() }));
    }
    Ok(Emit { config_mutations, effects, ..Emit::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
