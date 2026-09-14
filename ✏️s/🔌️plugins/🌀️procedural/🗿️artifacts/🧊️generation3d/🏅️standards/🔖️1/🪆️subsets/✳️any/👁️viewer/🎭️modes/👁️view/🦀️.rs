//! 👁️ Generation3d viewer — the `view` mode: a single full-pane Preview window, the read-only
//! counterpart of the editor's edit-mode flow-graph + preview split and generate-mode split. Ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1 only requires "≥1 mode with ≥1
//! window" for a viewer packet — a read-only Flow window and a Generations/Form pair are a
//! follow-up, not a purity or completeness requirement.

use crate::viewer::generation3d::modes::view::windows::preview;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, ToolRef, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const GENERATION3D_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::generation3d::create_generation3d_viewer`.
///
/// ⏯️ The viewer's only mode mounts the only window that starts the `previewEval` run, so it owns the
/// tool reference the plugin builder requires of every declared tool.
pub fn definition() -> ModeDefinition {
    ModeDefinition {
        id: GENERATION3D_VIEW_MODE_VIEW.into(),
        label: LocalizedLabel::native("View", "Ansicht"),
        icon_id: "eye".into(),
        tools: vec![semio_framework::io::resolve_ready(ToolRef::new(crate::preview_eval::PREVIEW_EVAL_TOOL_ID))],
        layout_id: None,
        commands: Vec::new(),
    }
}

/// 🪟️ Single full-pane Preview window — the read-only viewer has no quadrant layout to allocate.
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
