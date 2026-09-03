//! ✏️ Equation play app — the `edit` mode: the default two-window authoring layout (graph +
//! geometry).

use crate::editor::equation::modes::edit::windows::{geometry, graph};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const MATH_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::equation::create_equation_app`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: MATH_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub async fn layout() -> WindowLayout {
    create_default_layout(&[graph::MATH_PLAY_WINDOW_GRAPH.into(), geometry::MATH_PLAY_WINDOW_GEOMETRY.into()], "row", Some(&[60.0, 40.0]), Some(&["Graph".into(), "Geometry".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn the_default_layout_lists_both_edit_windows() {
        // 🌱️ `WindowLayout` (`semio-framework-plugin`, framework-owned) has not itself gained
        // `ToValue` — `Debug` gives the same "does the layout mention X" substring check.
        let debug = format!("{:?}", layout());
        assert!(debug.contains(graph::MATH_PLAY_WINDOW_GRAPH) && debug.contains(geometry::MATH_PLAY_WINDOW_GEOMETRY), "layout must reference both window kinds: {debug}");
    }
}
//#endregion 🧪️Tests
