//! 👁️ 👁️ Layout play app commands command — `focus-preflight-issue`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::{op::LayoutMutation, LayoutSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "focus-preflight-issue")]
pub struct FocusPreflightIssue {
    pub object_id: Option<String>,
    pub page_id: Option<String>,
}

/// 👁️ `object_id` used to write straight into `NoConfigMutation::SetSelection`; selection is
/// framework-owned now (domain "elements"), so a hit asks the host to redispatch `interactionSelect`
/// via an effect instead — see `crate::editor::layout::layout_select_effect`.
pub fn handle(payload: &FocusPreflightIssue, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    let mut effects = Vec::new();
    if let Some(object_id) = &payload.object_id {
        effects.push(crate::editor::layout::layout_select_effect(std::slice::from_ref(object_id), "replace"));
    }
    Ok(Emit { effects, ..Emit::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
