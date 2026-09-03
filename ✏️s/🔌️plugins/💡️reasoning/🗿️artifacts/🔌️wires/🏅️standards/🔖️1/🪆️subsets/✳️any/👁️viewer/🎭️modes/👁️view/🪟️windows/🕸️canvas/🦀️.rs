//! 🕸️ Wires viewer — the canvas window: a read-only render of the WIRES mindmap board. Genuinely
//! independent of the sibling editor module (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
//! contract §2.2) — never calls into it; duplicates the small, pure 2D-scene projection instead,
//! built only from framework-level `build_canvas_2d_scene`/`Canvas2dScene` and artifact-level pure
//! helpers (`wires_working_board`, `fixture_camera`, `fixture_nodes`, `fixture_edges`,
//! `wires_relationships`, `dsl_to_json`) — the wires board is a general node/edge graph (cycles legal,
//! see the artifact's own `topology.cycle_free` inference), so no SDK window kit (Text/Table/Tree/
//! Image/Mesh/Document/Media) matches it directly; `TreeWindowKit` was checked and rejected for this
//! reason before writing this render function by hand.

use crate::artifacts::wires::schema::{dsl_to_json, fixture_camera, fixture_edges, fixture_nodes, wires_relationships};
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;
use dsl::os_pack::json::Value;
use semio_framework_plugin::{build_canvas_2d_scene, Canvas2dScene, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const WIRES_VIEW_WINDOW_CANVAS: &str = "reasoning-wires-view-composite";
pub const WIRES_VIEW_BODY_CANVAS: &str = "reasoning.wires.view.composite";
const WIRES_VIEW_CANVAS_CONTROLLER_ID: &str = "reasoning-wires-view";
const WIRES_VIEW_CANVAS_SURFACE_ID: &str = "reasoning.wires.view.composite";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::wires::create_wires_viewer`.
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WIRES_VIEW_WINDOW_CANVAS.into(),
        label: LocalizedLabel::native("Canvas", "Leinwand"),
        body_key: WIRES_VIEW_BODY_CANVAS.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "git-branch".into(),
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
/// 🔗️ Turns `wires_fixture.relationships` into board-edge-shaped scene layers, synthesizing a
/// degenerate edge for any relationship whose board edge is missing — own copy of the editor
/// window's identically named helper (duplication is the deliberate cost of a genuinely independent
/// viewer, contract §2.2).
async fn relationship_edge_layers(wires: &DslValue, board: &DslValue) -> Vec<Value> {
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

/// 👁️ Read-only render straight off a `WiresSnapshot` — no config/runtime/utility state, matching the
/// viewer's `ViewEmit`-only contract.
pub async fn render(document: &WiresSnapshot) -> UiNode {
    let board = crate::artifacts::wires::wires_working_board(document);
    let wires = &document.wires_fixture;
    let (camera_x, camera_y, zoom) = fixture_camera(&board);
    let mut layers: Vec<Value> = fixture_nodes(&board).iter().map(dsl_to_json).collect();
    layers.extend(fixture_edges(&board).iter().map(dsl_to_json));
    layers.extend(relationship_edge_layers(wires, &board));
    build_canvas_2d_scene(WIRES_VIEW_CANVAS_SURFACE_ID, WIRES_VIEW_CANVAS_CONTROLLER_ID, Canvas2dScene { camera_x, camera_y, zoom, layers_json: dsl::os_pack::json::to_string(&Value::Array(layers)), snapshot: None })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_canvas_2d_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, WIRES_VIEW_BODY_CANVAS);
        assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_canvas_scene_for_the_empty_document() {
        let document = crate::artifacts::wires::empty_wires_snapshot();
        let json = serde_json::to_string(&render(&document)).expect("render json");
        assert!(json.contains("canvas-2d"));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_canvas_scene_for_the_metabolism_example() {
        let document = crate::artifacts::wires::schema::metabolism_wires_example_snapshot().expect("valid metabolism fixture mutations");
        let json = serde_json::to_string(&render(&document)).expect("render json");
        assert!(json.contains("canvas-2d"));
        assert!(json.contains("Demo") || json.contains("Metabolism") || json.contains("Topic"));
    }
}
//#endregion 🧪️Tests
