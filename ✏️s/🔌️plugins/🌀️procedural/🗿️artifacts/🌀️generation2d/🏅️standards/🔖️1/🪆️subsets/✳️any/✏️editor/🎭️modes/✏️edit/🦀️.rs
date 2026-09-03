//! ✏️ Generation2d play app — the `edit` mode: the default two-window authoring layout (flow graph +
//! preview).

use crate::editor::generation2d::modes::edit::windows::{flow, preview};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const GENERATION2D_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::generation2d::create_generation2d_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: GENERATION2D_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[flow::GENERATION2D_PLAY_WINDOW_MAIN.into(), preview::GENERATION2D_PLAY_WINDOW_PREVIEW.into()], "row", Some(&[55.0, 45.0]), Some(&["Main".into(), "Preview".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_both_edit_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(flow::GENERATION2D_PLAY_WINDOW_MAIN) && json.contains(preview::GENERATION2D_PLAY_WINDOW_PREVIEW));
    }
}
//#endregion 🧪️Tests
