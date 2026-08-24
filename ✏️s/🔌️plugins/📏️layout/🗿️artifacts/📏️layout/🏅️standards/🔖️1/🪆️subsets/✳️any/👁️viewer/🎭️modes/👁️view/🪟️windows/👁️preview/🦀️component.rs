//! 👁️ Layout viewer — the Preview window: a read-only render of the document's first page, built
//! from pure artifact-level page resolution (`crate::artifacts::layout::schema::resolve_page`) — this
//! file itself imports nothing from the sibling editor surface (`policyViewerPurityBreaches` forbids
//! it outright). No camera persistence (a viewer has no per-session config — `Config = NoConfig`, a
//! fixed default camera every render), no chrome (guides/margins/dashed inherited-frame strokes —
//! those are the editor's Blueprint authoring affordances), no text glyph layout: a viewer renders
//! frame geometry (fill/stroke rects, text/image outline placeholders) without the parley/fontique-
//! backed layout engine the sibling editor's `⚙️engine/🎬️scene` module carries — a documented
//! simplification for a first-pass viewer, not a bug, mirroring cad's viewer "default camera/sun,
//! fallback-box mesh" documented gap.

use crate::artifacts::layout::schema::resolve_page;
use crate::artifacts::layout::{Frame, LayoutSnapshot};
use semio_framework_plugin::{build_canvas_2d_scene, Canvas2dScene, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "layout-view-preview";
pub const BODY_KEY: &str = "layout.view.preview";
pub const SURFACE_ID: &str = "layout.view.preview";
/// 👁️ Read-only counterpart of the editor's `LAYOUT_PLAY_APP_ID` controller id — kept distinct so a
/// viewer session's canvas controller can never be mistaken for an editor session's.
const LAYOUT_VIEW_CONTROLLER_ID: &str = "layout-view";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::layout::create_layout_viewer`.
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "preview".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
async fn rect_segments(x: f64, y: f64, width: f64, height: f64) -> Value {
    json!([
        { "kind": "move", "to": [x, y] },
        { "kind": "line", "to": [x + width, y] },
        { "kind": "line", "to": [x + width, y + height] },
        { "kind": "line", "to": [x, y + height] },
        { "kind": "close" },
    ])
}

async fn host_layer(id: impl Into<String>, segments: &Value, fill: Option<[f32; 4]>, stroke: Option<[f32; 4]>) -> Value {
    let mut layer = json!({ "id": id.into(), "segments": segments });
    if let Some(color) = fill {
        layer["fill"] = json!({ "color": color });
    }
    if let Some(color) = stroke {
        layer["stroke"] = json!({ "color": color, "width": 1.0 });
    }
    layer
}

/// 👁️ Pure `LayoutSnapshot -> host canvas-2d layer JSON` read of the document's first page: a white
/// page background, one rect layer per visible resolved frame — real fill/stroke for `Frame::Rect`,
/// an outline rect for `Frame::Text` (no glyph layout, see this file's own doc), a placeholder tint
/// for `Frame::Image` (matches the editor's own unresolved-link placeholder color).
async fn viewer_canvas_layers(doc: &LayoutSnapshot) -> String {
    let Some(page) = doc.pages.first() else {
        return "[]".into();
    };
    let mut layers = vec![host_layer("layout.page-bg", &rect_segments(0.0, 0.0, page.width, page.height), Some([1.0, 1.0, 1.0, 1.0]), None)];
    for item in resolve_page(doc, page) {
        if !item.frame.visible() {
            continue;
        }
        let bounds = item.frame.bounds();
        let segments = rect_segments(bounds.x, bounds.y, bounds.width, bounds.height);
        match &item.frame {
            Frame::Rect { id, fill, stroke, .. } => layers.push(host_layer(id.clone(), &segments, *fill, *stroke)),
            Frame::Text { id, .. } => layers.push(host_layer(id.clone(), &segments, None, Some([0.2, 0.55, 0.9, 0.9]))),
            Frame::Image { id, .. } => layers.push(host_layer(id.clone(), &segments, Some([0.92, 0.88, 0.84, 1.0]), Some([0.75, 0.35, 0.2, 1.0]))),
        }
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}

/// 👁️ Fixed default camera every render — a viewer has no persisted per-session camera (`Config =
/// NoConfig`), matching cad's viewer's documented "default camera/sun" simplification.
pub async fn render(doc: &LayoutSnapshot) -> UiNode {
    build_canvas_2d_scene(SURFACE_ID, LAYOUT_VIEW_CONTROLLER_ID, Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: viewer_canvas_layers(doc), snapshot: None })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_a_canvas_2d_preview_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert!(matches!(def.surface_kind, SurfaceKind::Canvas2d));
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_scene_node_for_the_default_document() {
        let document = crate::artifacts::layout::schema::default_document();
        let _node = render(&document);
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_canvas_layers_renders_the_page_background() {
        let document = crate::artifacts::layout::schema::default_document();
        let json = viewer_canvas_layers(&document);
        assert!(json.contains("layout.page-bg"));
    }
}
//#endregion 🧪️Tests
