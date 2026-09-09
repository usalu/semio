//! 🕸️ Wires play app — the canvas window: the editable WIRES mindmap board.

#[path = "🎚️config/🦀️.rs"]
pub mod config;

use crate::schema::{dsl_to_json, fixture_edges, fixture_nodes, wires_relationships};
use dsl::os_pack::json::Value;
use dsl::DslValue;
use semio_framework_plugin::{BuiltNode, Canvas2dScene, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const WIRES_PLAY_WINDOW_CANVAS: &str = "reasoning-wires-composite";
pub const WIRES_PLAY_BODY_COMPOSITE: &str = "reasoning.wires.composite";
const WIRES_PLAY_SURFACE_ID: &str = "reasoning.wires.composite";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::wires::create_wires_app`.
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
        // 🕹️ Populated post-hoc by `create_wires_app`'s `.window_kind_interactions(..)` call — the
        // "graph" domain (node/edge selection + hover, ticket
        // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
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
            layers.push(dsl::os_pack::json::object([
                ("id".into(), edge_id.into()),
                ("kind".into(), "edge".into()),
                ("edgeKind".into(), relationship.get("kind").map_or_else(|| Value::from("relationship"), dsl_to_json)),
                ("source".into(), relationship.get("sourceIdentityId").map(|value| value.as_f64().map(|n| n.to_string()).unwrap_or_default()).unwrap_or_default().into()),
                ("target".into(), relationship.get("targetIdentityId").map(|value| value.as_f64().map(|n| n.to_string()).unwrap_or_default()).unwrap_or_default().into()),
            ]));
        }
    }
    layers
}

pub fn render(board: &DslValue, wires: &DslValue, window: &config::WiresCanvasWindowConfig) -> UiAssemblyResult<BuiltNode> {
    let camera_x = window.camera.x;
    let camera_y = window.camera.y;
    let zoom = window.camera.zoom;
    let mut layers: Vec<Value> = fixture_nodes(board).iter().map(dsl_to_json).collect();
    layers.extend(fixture_edges(board).iter().map(dsl_to_json));
    layers.extend(relationship_edge_layers(wires, board));
    semio_framework_plugin::scene_surface(WIRES_PLAY_SURFACE_ID, semio_framework_ui_contract::SurfaceKind::Canvas2d, &Canvas2dScene { camera_x, camera_y, zoom, layers_json: dsl::os_pack::json::to_string(&Value::Array(layers)), snapshot: None })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
