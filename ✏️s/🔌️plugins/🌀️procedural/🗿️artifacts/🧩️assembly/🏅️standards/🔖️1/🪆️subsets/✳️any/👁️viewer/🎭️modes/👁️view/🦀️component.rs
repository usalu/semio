//! 👁️ Assembly viewer — the `view` mode: a single full-pane `structure` window, the read-only
//! counterpart of the editor's own single-window `edit` mode. Ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1 only requires at least one real window
//! for a viewer packet.

use crate::viewer::assembly::modes::view::windows::structure;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const ASSEMBLY_VIEW_MODE_ID: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::assembly::create_assembly_viewer`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: ASSEMBLY_VIEW_MODE_ID.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane `structure` window — the read-only viewer has no quadrant layout to allocate.
pub async fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: structure::WINDOW_KIND_ID.into(), title: Some("Structure".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition
