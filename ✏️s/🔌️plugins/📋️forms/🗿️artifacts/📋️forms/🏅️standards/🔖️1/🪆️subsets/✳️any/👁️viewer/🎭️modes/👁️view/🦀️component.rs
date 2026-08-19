//! 👁️ Forms viewer — the `view` mode: a single full-pane Try window, the read-only counterpart of
//! the editor's `blueprint` mode's two-window authoring layout.

use crate::viewer::forms::modes::view::windows::try_wizard;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const FORMS_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::forms::create_forms_viewer`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: FORMS_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Try window — the read-only viewer has no side-by-side authoring layout to
/// allocate (the editor's own Builder window is editor-only).
pub async fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: try_wizard::WINDOW_KIND_ID.into(), title: Some("Try".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition
