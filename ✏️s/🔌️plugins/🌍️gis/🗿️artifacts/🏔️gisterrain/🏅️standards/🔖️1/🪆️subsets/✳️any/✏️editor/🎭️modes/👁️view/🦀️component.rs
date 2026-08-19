//! 👁️ GIS 3D play app — the `view` mode: a single full-width terrain window (read-mostly first pass).

use crate::editor::gis3d::modes::view::windows::terrain;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

//#region 🔖️Definition
pub const GIS3D_PLAY_MODE_VIEW: &str = "view";

pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: GIS3D_PLAY_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

pub async fn layout() -> WindowLayout {
    create_default_layout(&[terrain::GIS3D_PLAY_WINDOW_MAIN.into()], "row", Some(&[100.0]), Some(&["Terrain".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn the_default_layout_lists_the_single_terrain_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(terrain::GIS3D_PLAY_WINDOW_MAIN));
    }

    #[semio_framework_async_macros::async_test]
    async fn the_mode_is_the_apps_only_and_default_mode() {
        assert_eq!(definition().id, GIS3D_PLAY_MODE_VIEW);
    }
}
//#endregion 🧪️Tests
