//! 📄️ Shooting play app panel — the document tree: shots and assets of the current snapshot.

use crate::editor::shooting::terminology::ShootingLabels;
use crate::editor::shooting::SHOOTING_INTERACTION_DOMAIN;
use crate::artifacts::shooting::ShootingSnapshot;
use semio_framework_plugin::{Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const SHOOTING_PLAY_BODY_DOCUMENT: &str = "shooting.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"), group: PanelGroup::Workbench, body_key: Some(SHOOTING_PLAY_BODY_DOCUMENT.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ Builds an `interactionSelect` dispatch for one `"assets"`-domain target — replaces the deleted
/// `setSelection` action's asset half. This tree mixes shot AND asset rows under namespaced ids
/// (`"shooting-shot:…"`/`"shooting-asset:…"`), not the domain's raw ids, so it stays un-bound to
/// `.interaction_domain(...)` (matches `cad`'s `document_tree_selected_ids` precedent) — the asset row's
/// click action is built manually instead, one target per click (`"replace"` merge, matching the old
/// `setSelection` row-click semantics).
async fn asset_select_action(asset_id: &str) -> semio_framework_plugin::ActionDescriptor {
    let targets = serde_json::to_string(&json!([{ "granularity": "asset", "id": asset_id }])).unwrap_or_default();
    crate::editor::shooting::shooting_action("interactionSelect", Some(json!({ "domainId": SHOOTING_INTERACTION_DOMAIN, "targets": targets, "merge": "replace" })))
}

pub async fn render(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> UiNode {
    let shot_items: Vec<semio_framework_plugin::UiTreeItemNode> = snapshot
        .shots
        .iter()
        .map(|shot| crate::editor::shooting::tree_item_with_icon(format!("shooting-shot:{}", shot.id), Label::data(shot.label.clone()), "camera", crate::editor::shooting::shooting_action("setShotSelection", Some(json!({ "shotIds": [shot.id] })))))
        .collect();
    let asset_items: Vec<semio_framework_plugin::UiTreeItemNode> = snapshot
        .assets
        .iter()
        .map(|asset| crate::editor::shooting::tree_item_with_icon(format!("shooting-asset:{}", asset.id), Label::data(asset.name.clone()), "box", asset_select_action(&asset.id)))
        .collect();
    PanelTreeBuilder::new("shooting-play-document").section("shooting-play-document.shots", Some(labels.shots.into()), true, shot_items).section("shooting-play-document.assets", Some(labels.assets.into()), true, asset_items).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::testkit::{render as render_body, shooting_app};

    #[semio_framework_async_macros::async_test]
    async fn document_lists_shots_and_assets() {
        let mut app = shooting_app();
        let json = render_body(&mut app, SHOOTING_PLAY_BODY_DOCUMENT);
        assert!(json.contains("Overview Svg"));
        assert!(json.contains("Base"));
    }
}
//#endregion 🧪️Tests
