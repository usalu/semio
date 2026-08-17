//! 👁️ Lowpoly viewer — the `view` mode: a single full-pane Model window, the read-only counterpart
//! of the editor's `edit`/`paint` modes. Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
//! contract §1 only requires "one real view mode with ≥1 real window" for a viewer packet — a
//! read-only UV/paint-layer window is a follow-up, not a purity or completeness requirement.

use crate::viewer::lowpoly::modes::view::windows::model;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const LOWPOLY_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::lowpoly::create_lowpoly_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: LOWPOLY_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Model window — the read-only viewer has no quadrant layout to allocate.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: model::WINDOW_KIND_ID.into(), title: Some("Model".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition
