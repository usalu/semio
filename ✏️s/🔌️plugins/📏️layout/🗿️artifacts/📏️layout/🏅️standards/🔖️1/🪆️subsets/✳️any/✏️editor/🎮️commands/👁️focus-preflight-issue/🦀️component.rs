//! 👁️ 👁️ Layout play app commands command — `focus-preflight-issue`.

use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use crate::artifacts::layout::{op::LayoutMutation, LayoutSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "focus-preflight-issue")]
pub struct FocusPreflightIssue {
    pub object_id: Option<String>,
    pub page_id: Option<String>,
}

/// 👁️ `object_id` used to write straight into `LayoutConfigMutation::SetSelection`; selection is
/// framework-owned now (domain "elements"), so a hit asks the host to redispatch `interactionSelect`
/// via an effect instead — see `crate::editor::layout::layout_select_effect`.
pub async fn handle(payload: &FocusPreflightIssue, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    let mut config_mutations = Vec::new();
    let mut effects = Vec::new();
    if let Some(object_id) = &payload.object_id {
        effects.push(crate::editor::layout::layout_select_effect(std::slice::from_ref(object_id), "replace"));
    }
    if let Some(page_id) = &payload.page_id {
        config_mutations.push(LayoutConfigMutation::SetActivePage { page_id: page_id.clone() });
    }
    Ok(Emit { config_mutations, effects, ..Emit::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::layout::testkit::{dispatch, layout_app};
    use crate::editor::layout::LayoutCommand;
    use semio_framework::kernel::Effect;
    use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;

    #[test]
    async fn focus_preflight_issue_requests_a_select_effect_and_sets_active_page() {
        let mut app = layout_app();
        let result = dispatch(&mut app, LayoutCommand::FocusPreflightIssue(FocusPreflightIssue { object_id: Some("frame-1".into()), page_id: Some("page-2".into()) }));
        assert!(result.mutations.is_empty(), "preflight focus is config/effect-only, never a document operation");
        assert!(result.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == INTERACTION_SELECT_ACTION_ID)), "must ask the host to redispatch interactionSelect");
    }

    #[test]
    async fn focus_preflight_issue_without_an_object_id_only_sets_active_page() {
        let mut app = layout_app();
        let result = dispatch(&mut app, LayoutCommand::FocusPreflightIssue(FocusPreflightIssue { object_id: None, page_id: Some("page-2".into()) }));
        assert!(result.requested_effects.is_empty(), "no object id means no select effect");
    }
}
//#endregion 🧪️Tests
