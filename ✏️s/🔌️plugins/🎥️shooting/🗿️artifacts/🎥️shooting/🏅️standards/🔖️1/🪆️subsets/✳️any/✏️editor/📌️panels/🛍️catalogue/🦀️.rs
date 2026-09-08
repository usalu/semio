//! 🛍️ Shooting play app panel — the create catalogue: shot presets and the GLB asset preset.

use crate::editor::shooting::terminology::ShootingLabels;
use semio_framework_plugin::{Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const SHOOTING_PLAY_BODY_CATALOGUE: &str = "shooting.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(SHOOTING_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn catalog_shot_item(id: &str, label: impl TryInto<Label>, format: &str, shape: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let args = crate::editor::shooting::ui_value_map([
        ("format", crate::editor::shooting::ui_value_text(format)?),
        ("shape", crate::editor::shooting::ui_value_text(shape)?),
    ])?;
    crate::editor::shooting::tree_item_with_icon(format!("shooting-play-catalogue.{id}"), label, "camera", crate::editor::shooting::shooting_action("addShot", Some(args)))
}

pub fn render(labels: &ShootingLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let shot_items = crate::editor::shooting::ui_node_list([
        catalog_shot_item("svg-rect", labels.svg_rectangle, "svg", "rectangle"),
        catalog_shot_item("png-rect", labels.png_rectangle, "png", "rectangle"),
        catalog_shot_item("svg-ellipse", labels.svg_ellipse, "svg", "ellipse"),
        catalog_shot_item("png-ellipse", labels.png_ellipse, "png", "ellipse"),
    ])?;
    let asset_args = crate::editor::shooting::ui_value_map([("format", crate::editor::shooting::ui_value_text("glb")?)])?;
    let asset_items = crate::editor::shooting::ui_node_list([crate::editor::shooting::tree_item_with_icon(
        "shooting-play-catalogue.asset.glb",
        labels.glb_asset,
        "box",
        crate::editor::shooting::shooting_action("addAsset", Some(asset_args)),
    )])?;
    PanelTreeBuilder::new("shooting-play-catalogue")?.section("shooting-play-catalogue.shots", Some(crate::editor::shooting::ui_label(labels.add_shot.as_str())?), true, shot_items)?.section("shooting-play-catalogue.assets", Some(crate::editor::shooting::ui_label(labels.add_asset.as_str())?), true, asset_items)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
