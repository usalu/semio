//! 👁️ Mathematical viewer — the `view` mode: a single full-pane Geometry window, the read-only
//! counterpart of the editor's two-window `edit` mode. Ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1 only requires at least one real
//! window for a viewer packet — a read-only Graph window is a follow-up, not a purity or
//! completeness requirement.

use crate::viewer::mathematical::modes::view::windows::geometry;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const MATH_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::mathematical::create_mathematical_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: MATH_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Geometry window — the read-only viewer has no side-by-side layout to allocate.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: geometry::WINDOW_KIND_ID.into(), title: Some("Geometry".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition
