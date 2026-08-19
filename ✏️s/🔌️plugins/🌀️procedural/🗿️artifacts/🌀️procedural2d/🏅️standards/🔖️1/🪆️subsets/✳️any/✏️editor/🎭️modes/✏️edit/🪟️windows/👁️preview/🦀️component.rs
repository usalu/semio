//! 👁️ Procedural2d play app — the preview window: the evaluated 2D canvas.

use crate::editor::procedural2d::config::Procedural2dConfig;
use crate::editor::procedural2d::PROCEDURAL2D_PLAY_APP_ID;
use crate::artifacts::procedural2d::schema::{collect_drawing_handles_from_eval, scene_layers_from_drawing_handle};
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{build_canvas_2d_scene, Canvas2dScene, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};
use serde_json::Value;

//#region 🔖️Constants
pub const PROCEDURAL2D_PLAY_WINDOW_PREVIEW: &str = "procedural2d-preview";
pub const PROCEDURAL2D_PLAY_BODY_PREVIEW: &str = "procedural2d.play.preview";
const PROCEDURAL2D_PLAY_SURFACE_PREVIEW: &str = "procedural2d.play.preview";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PROCEDURAL2D_PLAY_WINDOW_PREVIEW.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: PROCEDURAL2D_PLAY_BODY_PREVIEW.into(),
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
        capabilities: Vec::new()}
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Overlays evaluated draw-handle layers, plus (in `"wire"` show mode) a schematic node box per
/// visible widget.
pub async fn render(document: &Procedural2dSnapshot, config: &Procedural2dConfig, session: &FlowEvalSession) -> UiNode {
    let fixture = &document.fixture;
    let eval_json = session.eval_json();
    let prefix = "procedural2d-preview";
    let mut layers = Vec::new();
    if let Ok(outputs) = serde_json::from_str::<Value>(eval_json) {
        let mut handles = Vec::new();
        collect_drawing_handles_from_eval(&outputs, &mut handles);
        handles.sort();
        handles.dedup();
        for handle in handles {
            layers.extend(scene_layers_from_drawing_handle(&handle, prefix));
        }
    }
    // 🕹️ `render` carries no `InteractionView` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM),
    // so the schematic wire overlay always shows every widget now (the pre-migration "nothing
    // selected" fallback), rather than filtering to a selection it can no longer read.
    if config.show_mode == "wire" {
        for widget in &fixture.widgets {
            let id = crate::artifacts::procedural2d::widget_id(widget).to_string();
            let (x, y) = fixture.layout.get(&id).map_or((48.0, 240.0), |layout| (layout.x, layout.y));
            layers.push(serde_json::json!({
                "id": format!("widget-{id}"),
                "kind": "node",
                "name": id,
                "x": x,
                "y": y,
                "width": 96.0,
                "height": 48.0}));
        }
    }
    build_canvas_2d_scene(
        PROCEDURAL2D_PLAY_SURFACE_PREVIEW,
        PROCEDURAL2D_PLAY_APP_ID,
        Canvas2dScene { camera_x: config.camera.x, camera_y: config.camera.y, zoom: config.camera.zoom, layers_json: serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into()) },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural2d::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_preview_canvas_scene() {
        let mut app = app();
        assert!(render_body(&mut app, PROCEDURAL2D_PLAY_BODY_PREVIEW).contains("canvas-2d"));
    }
}
//#endregion 🧪️Tests
