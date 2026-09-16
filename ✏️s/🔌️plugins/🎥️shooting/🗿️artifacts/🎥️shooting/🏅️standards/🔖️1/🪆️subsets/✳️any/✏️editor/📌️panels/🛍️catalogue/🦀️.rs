//! 🛍️ Shooting play app panel — the create catalogue: shot presets and the GLB asset preset.

use crate::editor::shooting::terminology::ShootingLabels;
use semio_framework_plugin::{LabelText, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

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
/// 🛍️ One catalogue preset row: the shot format/shape pair a click adds to the document.
struct ShotPreset {
    id: &'static str,
    label: LabelText,
    format: &'static str,
    shape: &'static str,
}

fn catalog_shot_item(preset: &ShotPreset) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let args = crate::editor::shooting::ui_value_map([("format", crate::editor::shooting::ui_value_text(preset.format)?), ("shape", crate::editor::shooting::ui_value_text(preset.shape)?)])?;
    crate::editor::shooting::tree_item_with_icon(format!("shooting-play-catalogue.{}", preset.id), preset.label, "camera", crate::editor::shooting::shooting_action("addShot", Some(args)))
}

pub fn render(labels: &ShootingLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let presets = [
        ShotPreset { id: "svg-rect", label: labels.svg_rectangle, format: "svg", shape: "rectangle" },
        ShotPreset { id: "png-rect", label: labels.png_rectangle, format: "png", shape: "rectangle" },
        ShotPreset { id: "svg-ellipse", label: labels.svg_ellipse, format: "svg", shape: "ellipse" },
        ShotPreset { id: "png-ellipse", label: labels.png_ellipse, format: "png", shape: "ellipse" },
    ];
    let assets = [labels.glb_asset];
    PanelTreeBuilder::new("shooting-play-catalogue")?
        .window_section(windows, "shooting-play-catalogue.shots", Some(crate::editor::shooting::ui_label(labels.add_shot.as_str())?), true, &presets, catalog_shot_item)?
        .window_section(windows, "shooting-play-catalogue.assets", Some(crate::editor::shooting::ui_label(labels.add_asset.as_str())?), true, &assets, |label| {
            let args = crate::editor::shooting::ui_value_map([("format", crate::editor::shooting::ui_value_text("glb")?)])?;
            crate::editor::shooting::tree_item_with_icon("shooting-play-catalogue.asset.glb", *label, "box", crate::editor::shooting::shooting_action("addAsset", Some(args)))
        })?
        .build()
}

//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
