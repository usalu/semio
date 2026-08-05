//! ✏️ Lowpoly play app — the `edit` mode: the default single-window mesh-editing layout (Model only).
//! The `paint` mode (sibling `🎨️paint/`) adds the UV window via its own named layout.

use crate::apps::lowpoly::modes::edit::windows::model;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const LOWPOLY_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::lowpoly::create_lowpoly_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: LOWPOLY_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[model::LOWPOLY_PLAY_WINDOW_MAIN.into()], "row", Some(&[100.0]), Some(&["Model".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_the_model_window_only() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(model::LOWPOLY_PLAY_WINDOW_MAIN));
    }
}
//#endregion 🧪️Tests
