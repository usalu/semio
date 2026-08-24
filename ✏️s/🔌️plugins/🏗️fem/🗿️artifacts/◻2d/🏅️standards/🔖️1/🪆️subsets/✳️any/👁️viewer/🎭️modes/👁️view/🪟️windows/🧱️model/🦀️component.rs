//! 🧱️ Fem2d viewer — the model window: a read-only render of the undeformed 2D structural canvas
//! (nodes/members/supports, mesh-edge preview overlay) — the read-only counterpart of the sibling
//! editor's model window. This file itself imports nothing from the sibling editor module
//! (`policyViewerPurityBreaches` forbids it outright): the pure geometry draw helpers it needs are
//! duplicated here rather than imported, per contract §2.2/§2.9. No selection, no camera persistence
//! (a viewer has no `Config`, so the camera is always `FemCamera::default()`), no results overlay —
//! that lives in the editor's separate results window, which this viewer does not (yet) mirror.

use crate::artifacts::fem2d::{element_id, Fem2dSnapshot, FemCamera, FemElement};
use semio_framework_plugin::{BuiltNode, Canvas2dScene};
use serde_json::json;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "fem2d-view-model";
pub const BODY_KEY: &str = "fem2d.view.model";
/// 👁️ Read-only counterpart of the editor's `FEM2D_APP_ID` controller id — kept distinct so a viewer
/// session's canvas-2d controller can never be mistaken for an editor session's.
const FEM2D_VIEW_CONTROLLER_ID: &str = "fem2d-view";

/// 📐️ Model-meters -> screen-pixels scale for the 2D canvas — duplicated from the sibling editor's
/// model window (same literal value, not imported through it; see this file's own doc comment).
const SCALE_2D: f64 = 20.0;
/// 📐️ Screen-space origin offset so a structure anchored at (0,0) isn't drawn at the canvas corner.
const ORIGIN_2D: f64 = 40.0;

/// 🎨️ Muted color for the mesh-edge preview overlay drawn under this window's members.
const MESH_EDGE_COLOR: &str = "#475569";
//#endregion 🔖️Constants

//#region 🔖️DrawHelpers
/// 👁️ Pure, self-contained duplicate of the sibling editor's `screen_2d` — a viewer window must not
/// import through the sibling editor module.
fn screen_2d(x: f64, y: f64) -> (f64, f64) {
    (x * SCALE_2D + ORIGIN_2D, -y * SCALE_2D + ORIGIN_2D)
}

fn find_node_2d<'a>(nodes: &'a [crate::artifacts::fem2d::FemNode], id: &str) -> Option<&'a crate::artifacts::fem2d::FemNode> {
    nodes.iter().find(|n| n.id == id)
}

fn fem2d_element_endpoints(element: &FemElement) -> (&str, &str) {
    match element {
        FemElement::Bar { start, end, .. } | FemElement::Beam { start, end, .. } => (start.as_str(), end.as_str()),
    }
}

/// 🖼️ Nodes/members/supports as Canvas2d layers — read-only duplicate of the sibling editor's
/// `fem2d_structure_layers`.
fn fem2d_structure_layers(doc: &Fem2dSnapshot, node_color: &str, line_color: &str, support_color: &str) -> Vec<serde_json::Value> {
    let mut layers = Vec::new();
    for node in &doc.nodes {
        let (sx, sy) = screen_2d(node.x, node.y);
        layers.push(json!({ "kind": "circle", "id": format!("node-{}", node.id), "x": sx - 4.0, "y": sy - 4.0, "width": 8.0, "height": 8.0, "color": node_color }));
    }
    for element in &doc.elements {
        let (start, end) = fem2d_element_endpoints(element);
        if let (Some(n1), Some(n2)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) {
            let (x0, y0) = screen_2d(n1.x, n1.y);
            let (x1, y1) = screen_2d(n2.x, n2.y);
            layers.push(json!({ "kind": "line", "id": format!("el-{}", element_id(element)), "x0": x0, "y0": y0, "x1": x1, "y1": y1, "color": line_color }));
        }
    }
    for support in &doc.supports {
        if let Some(node) = find_node_2d(&doc.nodes, &support.node_id) {
            let (sx, sy) = screen_2d(node.x, node.y);
            layers.push(json!({ "kind": "circle", "id": format!("support-{}", support.id), "x": sx - 5.0, "y": sy - 5.0, "width": 10.0, "height": 10.0, "color": support_color }));
        }
    }
    layers
}

/// 🗺️ Every meshed region's triangles as `(element_id, [screen_p0, screen_p1, screen_p2])` — read-only
/// duplicate of the sibling editor's `fem2d_region_triangles`. Calls
/// `crate::fem2d_engine::mesh_preview::fem2d_mesh_preview` directly (crate-root, shared compute — safe
/// to call from either surface, not a duplication concern for THAT call).
fn fem2d_region_triangles(doc: &Fem2dSnapshot) -> Vec<(String, [(f64, f64); 3])> {
    let mut out = Vec::new();
    let Ok(meshes) = crate::fem2d_engine::mesh_preview::fem2d_mesh_preview(doc) else { return out };
    for mesh in &meshes {
        for (tri_index, tri) in mesh.tris.iter().enumerate() {
            let id = format!("{}_t{}", mesh.region_id, tri_index);
            let p0 = mesh.points[tri[0] as usize];
            let p1 = mesh.points[tri[1] as usize];
            let p2 = mesh.points[tri[2] as usize];
            out.push((id, [screen_2d(p0[0], p0[1]), screen_2d(p1[0], p1[1]), screen_2d(p2[0], p2[1])]));
        }
    }
    out
}
//#endregion 🔖️DrawHelpers

//#region 🔖️Render
/// 👁️ Pure `Fem2dSnapshot -> UiNode` read: the undeformed structure plus the mesh-edge preview
/// overlay, hardcoded `FemCamera::default()` (a viewer has no persisted per-session camera —
/// `Config = NoConfig`). No results overlay, no selection, no gumball — a viewer has no utilities
/// that edit and emits no mutations by construction (`ViewEmit`).
pub fn render(doc: &Fem2dSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let camera = FemCamera::default();
    let mut layers = fem2d_structure_layers(doc, "#38bdf8", "#94a3b8", "#f97316");
    for (tri_index, (_, tri)) in fem2d_region_triangles(doc).iter().enumerate() {
        let [(x0, y0), (x1, y1), (x2, y2)] = *tri;
        layers.push(json!({
            "kind": "polyline",
            "id": format!("mesh-edge-{tri_index}"),
            "points": [[x0, y0], [x1, y1], [x1, y1], [x2, y2], [x2, y2], [x0, y0]],
            "color": MESH_EDGE_COLOR,
        }));
    }
    let layers_json = serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into());
    crate::app_surface::canvas_2d_surface(BODY_KEY, Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_a_canvas_2d_scene_for_the_default_document() {
        let document = crate::artifacts::fem2d::schema::empty_fem2d_snapshot();
        let json = serde_json::to_string(&render(&document)).expect("render json");
        assert!(json.contains("canvas-2d"), "expected a valid canvas-2d scene, got: {json}");
    }

    #[test]
    fn renders_mesh_edge_preview_for_the_default_example() {
        use store::ArtifactDsl;
        let document = Fem2dSnapshot::parse_dsl(crate::artifacts::fem2d::dsl::FEM2D_EXAMPLE_TEXT).expect("parse default example");
        let node = render(&document);
        let semio_framework_ui_contract::Component::Surface(props) = &node.component else { panic!("expected canvas surface") };
        let scene: Canvas2dScene = semio_framework_ui_scene::decode(props).expect("decode canvas scene");
        assert!(scene.layers_json.contains("mesh-edge-"), "expected mesh-edge preview layers in the view scene: {}", scene.layers_json);
    }
}
//#endregion 🧪️Tests
