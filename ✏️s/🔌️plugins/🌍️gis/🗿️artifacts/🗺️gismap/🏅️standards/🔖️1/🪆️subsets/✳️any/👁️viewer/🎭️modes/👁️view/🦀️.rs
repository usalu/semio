//! 👁️ GIS map viewer — the `view` mode: a single full-pane Map window, the read-only counterpart of
//! the editor's `edit` mode.

use crate::viewer::gismap::modes::view::windows::map;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const GIS_MAP_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::gismap::create_gismap_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: GIS_MAP_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Map window — the read-only viewer has no chrome to allocate beyond the canvas.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: map::WINDOW_KIND_ID.into(), title: Some("Map".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition
