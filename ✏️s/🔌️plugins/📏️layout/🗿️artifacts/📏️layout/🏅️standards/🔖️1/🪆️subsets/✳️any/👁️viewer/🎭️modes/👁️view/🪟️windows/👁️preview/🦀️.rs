//! 👁️ Layout viewer — the Preview window: a read-only render of the document's first page, built
//! from pure artifact-level page resolution (`crate::standards::v1::subsets::any::schema::resolve_page`) — this
//! file itself imports nothing from the sibling editor surface (`policyViewerPurityBreaches` forbids
//! it outright). No camera persistence (a viewer has no per-session config — `Config = NoConfig`, a
//! fixed default camera every render), no chrome (guides/margins/dashed inherited-frame strokes —
//! those are the editor's Blueprint authoring affordances), no text glyph layout: a viewer renders
//! frame geometry (fill/stroke rects, text/image outline placeholders) without the parley/fontique-
//! backed layout engine the sibling editor's `⚙️engine/🎬️scene` module carries — a documented
//! simplification for a first-pass viewer, not a bug, mirroring cad's viewer "default camera/sun,
//! fallback-box mesh" documented gap.

use crate::standards::v1::subsets::any::schema::resolve_page;
use crate::{Frame, LayoutSnapshot};
use semio_framework_plugin::{Canvas2dScene, LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "layout-view-preview";
pub const BODY_KEY: &str = "layout.view.preview";
pub const SURFACE_ID: &str = "layout.view.preview";

//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::layout::create_layout_viewer`.
pub fn definition() -> WindowKindDefinition {
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
fn rect_segments(x: f64, y: f64, width: f64, height: f64) -> Value {
    json!([
        { "kind": "move", "to": [x, y] },
        { "kind": "line", "to": [x + width, y] },
        { "kind": "line", "to": [x + width, y + height] },
        { "kind": "line", "to": [x, y + height] },
        { "kind": "close" },
    ])
}

fn host_layer(id: impl Into<String>, segments: &Value, fill: Option<[f32; 4]>, stroke: Option<[f32; 4]>) -> Value {
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
fn viewer_canvas_layers(doc: &LayoutSnapshot) -> String {
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
pub fn render(doc: &LayoutSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    semio_framework_plugin::scene_surface(SURFACE_ID, semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::Canvas2d, &Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: viewer_canvas_layers(doc), snapshot: None })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
