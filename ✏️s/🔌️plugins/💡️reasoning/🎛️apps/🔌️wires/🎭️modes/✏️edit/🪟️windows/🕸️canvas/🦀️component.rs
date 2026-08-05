//! 🕸️ Wires play app — the canvas window: the editable WIRES mindmap board.

use crate::apps::wires::WIRES_PLAY_APP_ID;
use crate::artifacts::wires::engine::{dsl_to_json, fixture_camera, fixture_edges, fixture_nodes, wires_relationships};
use dsl::DslValue;
use semio_framework_plugin::{build_canvas_2d_scene, Canvas2dScene, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const WIRES_PLAY_WINDOW_CANVAS: &str = "reasoning-wires-composite";
pub const WIRES_PLAY_BODY_COMPOSITE: &str = "reasoning.wires.composite";
const WIRES_PLAY_SURFACE_ID: &str = "reasoning.wires.composite";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::wires::create_wires_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WIRES_PLAY_WINDOW_CANVAS.into(),
        label: LocalizedLabel::native("Canvas", "Leinwand"),
        body_key: WIRES_PLAY_BODY_COMPOSITE.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "git-branch".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        document_projection_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🔗️ Turns `wires_fixture.relationships` into board-edge-shaped scene layers, synthesizing a
/// degenerate edge for any relationship whose board edge is missing.
fn relationship_edge_layers(wires: &DslValue, board: &DslValue) -> Vec<Value> {
    let mut layers = Vec::new();
    for relationship in wires_relationships(wires) {
        let edge_id = relationship.get("edgeId").and_then(|value| value.as_str()).unwrap_or("");
        if edge_id.is_empty() {
            continue;
        }
        let edge = fixture_edges(board).iter().find(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(edge_id));
        if let Some(edge) = edge {
            layers.push(dsl_to_json(edge));
        } else {
            layers.push(json!({
                "id": edge_id,
                "kind": "edge",
                "edgeKind": relationship.get("kind").map_or_else(|| json!("relationship"), dsl_to_json),
                "source": relationship.get("sourceIdentityId").map(|value| value.as_f64().map(|n| n.to_string()).unwrap_or_default()).unwrap_or_default(),
                "target": relationship.get("targetIdentityId").map(|value| value.as_f64().map(|n| n.to_string()).unwrap_or_default()).unwrap_or_default(),
            }));
        }
    }
    layers
}

pub fn render(board: &DslValue, wires: &DslValue) -> UiNode {
    let (camera_x, camera_y, zoom) = fixture_camera(board);
    let mut layers: Vec<Value> = fixture_nodes(board).iter().map(dsl_to_json).collect();
    layers.extend(fixture_edges(board).iter().map(dsl_to_json));
    layers.extend(relationship_edge_layers(wires, board));
    build_canvas_2d_scene(WIRES_PLAY_SURFACE_ID, WIRES_PLAY_APP_ID, Canvas2dScene { camera_x, camera_y, zoom, layers_json: serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into()) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::wires::testkit::{new_app, render as render_body};

    #[test]
    fn renders_canvas_scene() {
        let mut app = new_app();
        assert!(render_body(&mut app, WIRES_PLAY_BODY_COMPOSITE).contains("canvas-2d"));
    }

    #[test]
    fn definition_declares_the_canvas_2d_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, WIRES_PLAY_BODY_COMPOSITE);
        assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
    }
}
//#endregion 🧪️Tests
