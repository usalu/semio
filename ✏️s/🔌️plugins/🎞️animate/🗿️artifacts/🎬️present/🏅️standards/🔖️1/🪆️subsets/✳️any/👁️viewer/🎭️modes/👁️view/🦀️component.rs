//! 👁️ Animate viewer — the `view` mode: a single full-pane tile-editor window, the read-only
//! counterpart of the editor's single `main` mode.

use crate::viewer::animate::modes::view::windows::tile_editor;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const ANIMATE_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::animate::create_animate_present_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: ANIMATE_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane tile-editor window — the read-only viewer has no quadrant layout to allocate.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: tile_editor::WINDOW_KIND_ID.into(), title: Some("Tile editor".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_view_layout_lists_the_tile_editor_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(tile_editor::WINDOW_KIND_ID), "layout must reference the tile-editor window kind: {json}");
    }
}
//#endregion 🧪️Tests
