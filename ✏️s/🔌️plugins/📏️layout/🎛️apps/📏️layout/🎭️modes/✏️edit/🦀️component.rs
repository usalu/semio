//! ✏️ Layout play app — the `edit` mode: the only mode, laying out the Blueprint (authoring) and
//! Preview (read-only) surfaces side by side.

use crate::apps::layout::modes::edit::windows::{blueprint, preview};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const LAYOUT_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::layout::create_layout_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: LAYOUT_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[blueprint::LAYOUT_PLAY_WINDOW_BLUEPRINT.into(), preview::LAYOUT_PLAY_WINDOW_PREVIEW.into()], "row", Some(&[55.0, 45.0]), Some(&["Blueprint".into(), "Preview".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_both_edit_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(blueprint::LAYOUT_PLAY_WINDOW_BLUEPRINT) && json.contains(preview::LAYOUT_PLAY_WINDOW_PREVIEW), "layout must reference both window kinds: {json}");
    }
}
//#endregion 🧪️Tests
