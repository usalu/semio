//! ✏️ GIS 2D play app — the `edit` mode: a single full-width map window.

use crate::editor::gis2d::modes::edit::windows::map;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

//#region 🔖️Definition
pub const GIS2D_PLAY_MODE_EDIT: &str = "edit";

pub fn definition() -> ModeDefinition {
    ModeDefinition { id: GIS2D_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

pub fn layout() -> WindowLayout {
    create_default_layout(&[map::GIS2D_PLAY_WINDOW_MAIN.into()], "row", Some(&[100.0]), Some(&["Map".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_the_single_map_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(map::GIS2D_PLAY_WINDOW_MAIN));
    }

    #[test]
    fn the_mode_is_the_apps_only_and_default_mode() {
        assert_eq!(definition().id, GIS2D_PLAY_MODE_EDIT);
    }
}
//#endregion 🧪️Tests
