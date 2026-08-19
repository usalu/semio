//! 👁️ CAD viewer — the `view` mode: a single full-pane Shape window, the read-only counterpart of
//! the editor's quad `edit` mode. Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1
//! only requires "at least the `📐️shape` world-3d window" for a viewer packet — Building/Energy/
//! Structure Classic read-only windows are a follow-up, not a purity or completeness requirement.

use crate::viewer::cad::modes::view::windows::shape;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const CAD_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::cad::create_cad_viewer`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: CAD_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Shape window — the read-only viewer has no quadrant layout to allocate.
pub async fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: shape::WINDOW_KIND_ID.into(), title: Some("Shape".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition
