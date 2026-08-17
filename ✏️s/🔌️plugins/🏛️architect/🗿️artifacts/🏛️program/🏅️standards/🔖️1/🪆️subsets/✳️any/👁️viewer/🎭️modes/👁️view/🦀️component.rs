//! 👁️ Architect viewer — the `view` mode: a single full-pane Register Overview window, the read-only
//! counterpart of the sibling editor surface's five-window `edit` mode.

use crate::viewer::architect::modes::view::windows::register;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const ARCHITECT_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🏛️ Stitched into the viewer manifest by `crate::viewer::architect::create_architect_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: ARCHITECT_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Register Overview window — the read-only viewer has no side-by-side five-window
/// authoring layout to allocate (the editor's own Adjacency/Graph/Register/Report/Trace windows are
/// editor-only).
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: register::ARCHITECT_VIEW_WINDOW_REGISTER.into(), title: Some("Register Overview".into()), instance_id: None, template_id: None }],
        }),
    }
}
//#endregion 🔖️Definition
