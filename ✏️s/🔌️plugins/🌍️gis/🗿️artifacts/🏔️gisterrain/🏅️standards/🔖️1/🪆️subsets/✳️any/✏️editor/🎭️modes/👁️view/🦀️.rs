//! 👁️ GIS 3D play app — the `view` mode: a single full-width terrain window (read-mostly first pass).

use crate::editor::gis3d::modes::view::windows::terrain;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

//#region 🔖️Definition
pub const GIS3D_PLAY_MODE_VIEW: &str = "view";

pub fn definition() -> ModeDefinition {
    ModeDefinition { id: GIS3D_PLAY_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

pub fn layout() -> WindowLayout {
    create_default_layout(&[terrain::GIS3D_PLAY_WINDOW_MAIN.into()], "row", Some(&[100.0]), Some(&["Terrain".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
