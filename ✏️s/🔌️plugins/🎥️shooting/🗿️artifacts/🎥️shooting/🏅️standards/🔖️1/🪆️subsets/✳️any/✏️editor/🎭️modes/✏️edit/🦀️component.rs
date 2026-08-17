//! ✏️ Shooting play app — the `edit` mode: the default two-window studio layout (3D scene + icon
//! preview).

use crate::editor::shooting::modes::edit::windows::{icon, scene};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const SHOOTING_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::shooting::create_shooting_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: SHOOTING_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[scene::SHOOTING_PLAY_WINDOW_SCENE.into(), icon::SHOOTING_PLAY_WINDOW_ICON.into()], "row", Some(&[68.0, 32.0]), Some(&["Model".into(), "Icon".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_both_edit_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(scene::SHOOTING_PLAY_WINDOW_SCENE) && json.contains(icon::SHOOTING_PLAY_WINDOW_ICON), "layout must reference both window kinds: {json}");
    }
}
//#endregion 🧪️Tests
