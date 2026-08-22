//! 🐚️ 🐚️ Layout play app commands command — `engagement-submit`.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;
use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use semio_framework::kernel::Effect;
use semio_framework_plugin::{engagement_token_matches, ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "engagement-submit")]
pub struct EngagementSubmit {
    pub value: String,
}

/// 🐚️ Redispatches typed export intents so the exact public action enters its resumable job.
pub async fn handle(payload: &EngagementSubmit, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    let typed = payload.value.trim();
    let action = if engagement_token_matches(typed, "export png") || engagement_token_matches(typed, "png") {
        Some(("exportPng", Some(serde_json::json!({ "pageId": null }))))
    } else if engagement_token_matches(typed, "export svg") || engagement_token_matches(typed, "svg") {
        Some(("exportSvg", Some(serde_json::json!({ "pageId": null }))))
    } else if engagement_token_matches(typed, "export pdf") || engagement_token_matches(typed, "pdf") {
        Some(("exportPdf", Some(serde_json::json!({ "pageId": null }))))
    } else if engagement_token_matches(typed, "export package") || engagement_token_matches(typed, "package") {
        Some(("exportPackage", Some(serde_json::json!({}))))
    } else {
        None
    };
    Ok(action.map_or_else(Emit::default, |(action, args)| Emit {
        effects: vec![Effect::DispatchAction { req: semio_framework_plugin::RequestId(116), action: action.into(), args: semio_framework::optional_json_to_dsl(args), delay_ms: 0 }],
        ..Default::default()
    }))
}
