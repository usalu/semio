//! 👁️ Procedural2d viewer — the `view` mode: a single full-pane Preview window, the read-only
//! counterpart of the editor's two-window `edit` mode. Ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1 only requires a genuinely independent,
//! minimal viewer with ≥1 real window — a Generate-mode read-only twin is a follow-up, not a purity or
//! completeness requirement.

use crate::viewer::procedural2d::modes::view::windows::preview;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const PROCEDURAL2D_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::procedural2d::create_procedural2d_viewer`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: PROCEDURAL2D_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Preview window — the read-only viewer has no multi-window layout to allocate.
pub async fn layout() -> WindowLayout {
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
