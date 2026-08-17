//! ✏️ Assembly editor — the `edit` mode: a single full-pane `structure` window over the artifact's
//! own WFC problem spec. Nothing pane-specific lives here; the window binds its own definition/render
//! in its own file.

use crate::editor::assembly::modes::edit::windows::structure;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const ASSEMBLY_EDIT_MODE_ID: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::assembly::create_assembly_editor`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: ASSEMBLY_EDIT_MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane `structure` window — a first pass has no quadrant layout to allocate.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: structure::WINDOW_KIND_ID.into(), title: Some("Structure".into()), instance_id: None, template_id: None }],
        }),
    }
}
//#endregion 🔖️Definition
