//! 📄️ Shooting play app panel — the document tree: shots and assets of the current snapshot.

use crate::apps::shooting::terminology::ShootingLabels;
use crate::artifacts::shooting::ShootingSnapshot;
use semio_framework_plugin::{Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const SHOOTING_PLAY_BODY_DOCUMENT: &str = "shooting.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_DOCUMENT_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"), group: PanelGroup::Workbench, body_key: Some(SHOOTING_PLAY_BODY_DOCUMENT.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> UiNode {
    let shot_items: Vec<semio_framework_plugin::UiTreeItemNode> = snapshot
        .shots
        .iter()
        .map(|shot| crate::apps::shooting::tree_item_with_icon(format!("shooting-shot:{}", shot.id), Label::data(shot.label.clone()), "camera", crate::apps::shooting::shooting_action("setSelection", Some(json!({ "shotIds": [shot.id], "assetIds": [] })))))
        .collect();
    let asset_items: Vec<semio_framework_plugin::UiTreeItemNode> = snapshot
        .assets
        .iter()
        .map(|asset| crate::apps::shooting::tree_item_with_icon(format!("shooting-asset:{}", asset.id), Label::data(asset.name.clone()), "box", crate::apps::shooting::shooting_action("setSelection", Some(json!({ "shotIds": [], "assetIds": [asset.id] })))))
        .collect();
    PanelTreeBuilder::new("shooting-play-document").section("shooting-play-document.shots", Some(labels.shots.into()), true, shot_items).section("shooting-play-document.assets", Some(labels.assets.into()), true, asset_items).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::shooting::testkit::{render as render_body, shooting_app};

    #[test]
    fn document_lists_shots_and_assets() {
        let mut app = shooting_app();
        let json = render_body(&mut app, SHOOTING_PLAY_BODY_DOCUMENT);
        assert!(json.contains("Overview Svg"));
        assert!(json.contains("Base"));
    }
}
//#endregion 🧪️Tests
