//! 🛍️ Shooting play app panel — the create catalogue: shot presets and the GLB asset preset.

use crate::editor::shooting::terminology::ShootingLabels;
use semio_framework_plugin::{Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const SHOOTING_PLAY_BODY_CATALOGUE: &str = "shooting.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), group: PanelGroup::Workbench, body_key: Some(SHOOTING_PLAY_BODY_CATALOGUE.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
async fn catalog_shot_item(id: &str, label: impl Into<Label>, format: &str, shape: &str) -> UiTreeItemNode {
    crate::editor::shooting::tree_item_with_icon(format!("shooting-play-catalogue.{id}"), label, "camera", crate::editor::shooting::shooting_action("addShot", Some(json!({ "format": format, "shape": shape }))))
}

pub async fn render(labels: &ShootingLabels) -> UiNode {
    let shot_items = vec![
        catalog_shot_item("svg-rect", labels.svg_rectangle, "svg", "rectangle"),
        catalog_shot_item("png-rect", labels.png_rectangle, "png", "rectangle"),
        catalog_shot_item("svg-ellipse", labels.svg_ellipse, "svg", "ellipse"),
        catalog_shot_item("png-ellipse", labels.png_ellipse, "png", "ellipse"),
    ];
    let asset_items = vec![crate::editor::shooting::tree_item_with_icon("shooting-play-catalogue.asset.glb", labels.glb_asset, "box", crate::editor::shooting::shooting_action("addAsset", Some(json!({ "format": "glb" }))))];
    PanelTreeBuilder::new("shooting-play-catalogue").section("shooting-play-catalogue.shots", Some(labels.add_shot.into()), true, shot_items).section("shooting-play-catalogue.assets", Some(labels.add_asset.into()), true, asset_items).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::testkit::{render as render_body, shooting_app};

    #[test]
    async fn catalogue_lists_the_shot_presets_and_glb_asset() {
        let mut app = shooting_app();
        let json = render_body(&mut app, SHOOTING_PLAY_BODY_CATALOGUE);
        assert!(json.contains("Add Shot"));
        assert!(json.contains("Add Asset"));
        assert!(json.contains("SVG Rectangle"));
        assert!(json.contains("GLB Asset"));
    }
}
//#endregion 🧪️Tests
