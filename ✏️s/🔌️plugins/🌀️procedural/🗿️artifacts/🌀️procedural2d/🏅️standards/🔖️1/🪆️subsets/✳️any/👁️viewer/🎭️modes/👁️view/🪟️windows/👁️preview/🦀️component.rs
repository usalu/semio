//! 👁️ Procedural2d viewer — the Preview window: a read-only schematic render of every widget's
//! position, built from the same pure artifact-level `widget_id` helper the editor's own preview
//! window (`🎭️modes/✏️edit/🪟️windows/👁️preview`) uses for its "wire" overlay — this file itself
//! imports nothing from the sibling editor surface (`policyViewerPurityBreaches` forbids it outright).
//! No live flow-evaluation session, no camera persistence (a viewer has no config store — a fixed
//! default camera is used instead), no selection: a viewer emits no mutations by construction
//! (`ViewEmit`), so this render duplicates the small, pure structural layer instead of sharing it.

use crate::artifacts::procedural2d::{widget_id, Procedural2dSnapshot};
use semio_framework_plugin::{build_canvas_2d_scene, Canvas2dScene, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "procedural2d-view-preview";
pub const BODY_KEY: &str = "procedural2d.view.preview";
const SURFACE_ID: &str = "procedural2d.view.preview";
/// 👁️ Read-only counterpart of the editor's `PROCEDURAL2D_PLAY_APP_ID` controller id — kept distinct
/// so a viewer session's canvas controller can never be mistaken for an editor session's.
const PROCEDURAL2D_VIEW_CONTROLLER_ID: &str = "procedural2d-view";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::procedural2d::create_procedural2d_viewer`.
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
/// 👁️ Pure `Procedural2dSnapshot -> UiNode` read: a fixed default camera (a viewer has no persisted
/// per-session camera — `Config = NoConfig`), one schematic box per widget at its stored layout
/// position — no evaluated drawing-handle overlay (that needs a live `flow::FlowEvalSession`, an
/// editor-dispatch-time concept a stateless viewer render never has access to).
pub fn render(document: &Procedural2dSnapshot) -> UiNode {
    let fixture = &document.fixture;
    let layers: Vec<serde_json::Value> = fixture
        .widgets
        .iter()
        .map(|widget| {
            let id = widget_id(widget).to_string();
            let (x, y) = fixture.layout.get(&id).map_or((48.0, 240.0), |layout| (layout.x, layout.y));
            serde_json::json!({
                "id": format!("widget-{id}"),
                "kind": "node",
                "name": id,
                "x": x,
                "y": y,
                "width": 96.0,
                "height": 48.0,
            })
        })
        .collect();
    build_canvas_2d_scene(SURFACE_ID, PROCEDURAL2D_VIEW_CONTROLLER_ID, Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into()) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_the_canvas_2d_surface_and_body_key() {
        let def = definition();
        assert_eq!(def.body_key, BODY_KEY);
        assert!(matches!(def.surface_kind, SurfaceKind::Canvas2d));
    }

    #[test]
    fn render_produces_a_scene_node_for_the_default_document() {
        let document = crate::artifacts::procedural2d::schema::default_snapshot();
        let json = serde_json::to_string(&render(&document)).expect("render json");
        assert!(json.contains("canvas-2d"));
    }
}
//#endregion 🧪️Tests
