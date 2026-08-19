//! 👁️ Procedural3d viewer — the `view` mode: a single full-pane Preview window, the read-only
//! counterpart of the editor's edit-mode flow-graph + preview split and generate-mode split. Ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1 only requires "≥1 mode with ≥1
//! window" for a viewer packet — a read-only Flow window and a Generations/Form pair are a
//! follow-up, not a purity or completeness requirement.

use crate::viewer::procedural3d::modes::view::windows::preview;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const PROCEDURAL3D_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::procedural3d::create_procedural3d_viewer`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: PROCEDURAL3D_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Preview window — the read-only viewer has no quadrant layout to allocate.
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
