//! 👁️ GIS terrain viewer — the `view` mode: a single full-pane Terrain window, the read-only
//! counterpart of the editor's own `view` mode.

use crate::viewer::gisterrain::modes::view::windows::terrain;
use semio_framework_plugin::{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

pub const GIS_TERRAIN_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::gisterrain::create_gisterrain_viewer`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: GIS_TERRAIN_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Terrain window — the read-only viewer has no chrome to allocate beyond the viewport.
pub async fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: terrain::WINDOW_KIND_ID.into(), title: Some("Terrain".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition
