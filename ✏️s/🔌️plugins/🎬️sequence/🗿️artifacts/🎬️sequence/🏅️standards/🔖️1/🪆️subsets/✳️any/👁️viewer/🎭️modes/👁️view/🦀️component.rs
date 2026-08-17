//! 👁️ Sequence viewer — the `view` mode: a single full-pane Main window, the read-only counterpart
//! of the editor's three-window `edit` mode (graph canvas + compiled script + DSL). Contract §1 only
//! requires "at least one real window" for a viewer packet — Script/Compiled read-only windows are a
//! follow-up, not a purity or completeness requirement.

use crate::viewer::sequence::modes::view::windows::main;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const SEQUENCE_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::sequence::create_sequence_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: SEQUENCE_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Main window — the read-only viewer has no three-column layout to allocate.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: main::SEQUENCE_VIEW_WINDOW_MAIN.into(), title: Some("Sequence".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition
