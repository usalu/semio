//! 👁️ Layout viewer — the `view` mode: a single full-pane Preview window, the read-only counterpart
//! of the editor's `edit` mode (which lays Blueprint/Preview out side by side). Ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1 only requires "at least one real
//! window" for a viewer packet — a second read-only Blueprint-shaped window is a follow-up, not a
//! purity or completeness requirement.

use crate::viewer::layout::modes::view::windows::preview;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const LAYOUT_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::layout::create_layout_viewer`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: LAYOUT_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Preview window — the read-only viewer has no side-by-side layout to allocate.
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn the_default_layout_references_the_preview_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(preview::WINDOW_KIND_ID), "layout must reference the preview window kind: {json}");
    }
}
//#endregion 🧪️Tests
