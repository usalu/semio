//! 👁️ 2D-grid viewer — the `view` mode: one full-pane `preview` window, the read-only counterpart of
//! the editor's two-pane `edit` mode. A viewer never solves, so the pane draws the authored grid
//! itself: the cell rects plus every pinned cell's tile media, which is the whole persisted state.

use crate::viewer::grid2d::modes::view::windows::preview;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const GRID2D_VIEW_MODE_ID: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::grid2d::create_grid2d_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: GRID2D_VIEW_MODE_ID.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane `preview` window — the read-only viewer has no quadrant layout to allocate.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: preview::WINDOW_KIND_ID.into(), title: Some("Preview".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition
