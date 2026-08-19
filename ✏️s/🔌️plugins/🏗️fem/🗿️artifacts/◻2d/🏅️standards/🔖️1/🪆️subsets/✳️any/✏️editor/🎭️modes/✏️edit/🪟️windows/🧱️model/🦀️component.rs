//! 🧱️ Fem2d play app — the model window: the editable 2D structural canvas (nodes/members/supports,
//! mesh-edge preview overlay). Also hosts the screen-space draw helpers shared with the results window
//! (`crate::editor::fem2d::modes::edit::windows::results`) — kept here rather than in the artifact's
//! `⚙️engine` because they take/return app-facing `semio_framework_plugin` scene types and their only
//! two consumers are these two sibling windows, both at app level.

use crate::artifacts::fem2d::{element_id, Fem2dSnapshot, FemCamera, FemElement};
use crate::model::Dof;
use semio_framework_plugin::{build_canvas_2d_scene, Canvas2dScene, UiNode};
use serde_json::json;
use std::collections::HashMap;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "fem2d-model";
pub const BODY_KEY: &str = "fem2d.play.model";

/// 📐️ Model-meters -> screen-pixels scale for the 2D canvas (a 6m span shouldn't render as 6px wide).
pub(crate) const SCALE_2D: f64 = 20.0;
/// 📐️ Screen-space origin offset so a structure anchored at (0,0) isn't drawn at the canvas corner.
pub(crate) const ORIGIN_2D: f64 = 40.0;
/// 📐️ Exaggeration factor for offsetting the moment-diagram polyline perpendicular to a member — single
/// consumer: the results window's static-results moment diagram
/// (`crate::editor::fem2d::modes::edit::windows::results::render`).
pub(crate) const MOMENT_SCALE_2D: f64 = 0.001;

/// 🎨️ Muted color for the mesh-edge preview overlay drawn under this window's members.
const MESH_EDGE_COLOR: &str = "#475569";
//#endregion 🔖️Constants

//#region 🔖️SharedDrawHelpers
pub(crate) async fn screen_2d(x: f64, y: f64) -> (f64, f64) {
    (x * SCALE_2D + ORIGIN_2D, -y * SCALE_2D + ORIGIN_2D)
}

pub(crate) async fn find_node_2d<'a>(nodes: &'a [crate::artifacts::fem2d::FemNode], id: &str) -> Option<&'a crate::artifacts::fem2d::FemNode> {
    nodes.iter().find(|n| n.id == id)
}

pub(crate) async fn fem2d_element_endpoints(element: &FemElement) -> (&str, &str) {
    match element {
        FemElement::Bar { start, end, .. } | FemElement::Beam { start, end, .. } => (start.as_str(), end.as_str()),
    }
}

/// 📐️ Bounding-box diagonal (in model meters) over every node plus every region outline vertex — the
/// reference length `MODE_SHAPE_AMPLITUDE_RATIO` scales a normalized mode shape against. Falls back to
/// `1.0` for a degenerate (empty or point-like) model so mode-shape rendering never divides by zero.
pub(crate) async fn fem2d_model_extent(doc: &Fem2dSnapshot) -> f64 {
    let mut min = [f64::INFINITY; 2];
    let mut max = [f64::NEG_INFINITY; 2];
    let mut expand = |x: f64, y: f64| {
        min[0] = min[0].min(x);
        min[1] = min[1].min(y);
        max[0] = max[0].max(x);
        max[1] = max[1].max(y);
    };
    for node in &doc.nodes {
        expand(node.x, node.y);
    }
    for region in &doc.regions {
        for p in &region.outline {
            expand(p[0], p[1]);
        }
    }
    if min[0] > max[0] {
        return 1.0;
    }
    let d = [max[0] - min[0], max[1] - min[1]];
    (d[0] * d[0] + d[1] * d[1]).sqrt().max(1.0)
}

/// 🖼️ Nodes/members/supports as Canvas2d layers — shared by this window (bright colors) and the results
/// window's faint undeformed backdrop (a single muted color for every layer kind).
pub(crate) async fn fem2d_structure_layers(doc: &Fem2dSnapshot, node_color: &str, line_color: &str, support_color: &str) -> Vec<serde_json::Value> {
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

/// 🗺️ Every meshed region's triangles as `(element_id, [screen_p0, screen_p1, screen_p2])` — the
/// element id matches `fem2d_solve`/`fem2d_solve_all`'s `Tri3Cst` ids (`"{region_id}_t{tri_index}"`),
/// so callers can correlate a solved `ElementResult::Plane` back to on-screen triangle geometry. A
/// mesh failure for one region silently yields fewer triangles rather than failing the whole render.
pub(crate) async fn fem2d_region_triangles(doc: &Fem2dSnapshot) -> Vec<(String, [(f64, f64); 3])> {
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

/// 🗺️ Every meshed region's triangles as `(element_id, screen points, node ids)` — like
/// `fem2d_region_triangles` but also carrying each vertex's mesh node id, needed to look values up in
/// `fem2d_nodal_von_mises`'s node-keyed map for banded contour rendering.
pub(crate) async fn fem2d_region_mesh_triangles(doc: &Fem2dSnapshot) -> Vec<(String, [(f64, f64); 3], [String; 3])> {
    let mut out = Vec::new();
    let Ok(meshes) = crate::fem2d_engine::mesh_preview::fem2d_mesh_preview(doc) else { return out };
    for mesh in &meshes {
        for (tri_index, tri) in mesh.tris.iter().enumerate() {
            let id = format!("{}_t{}", mesh.region_id, tri_index);
            let p0 = mesh.points[tri[0] as usize];
            let p1 = mesh.points[tri[1] as usize];
            let p2 = mesh.points[tri[2] as usize];
            let node_ids = [mesh.node_ids[tri[0] as usize].clone(), mesh.node_ids[tri[1] as usize].clone(), mesh.node_ids[tri[2] as usize].clone()];
            out.push((id, [screen_2d(p0[0], p0[1]), screen_2d(p1[0], p1[1]), screen_2d(p2[0], p2[1])], node_ids));
        }
    }
    out
}

/// 🖼️ Every element's deformed-shape polyline (pink), given a node-id-keyed displacement map and a
/// display scale — shared by the static, modal, and buckling results renders.
pub(crate) async fn fem2d_deformed_shape_layers(doc: &Fem2dSnapshot, disp_map: &HashMap<String, [f64; 6]>, deform_scale: f64) -> Vec<serde_json::Value> {
    let mut layers = Vec::new();
    for element in &doc.elements {
        let (start, end) = fem2d_element_endpoints(element);
        let (Some(n1), Some(n2)) = (find_node_2d(&doc.nodes, start), find_node_2d(&doc.nodes, end)) else { continue };
        let (x0, y0) = screen_2d(n1.x, n1.y);
        let (x1, y1) = screen_2d(n2.x, n2.y);
        let d1 = disp_map.get(&n1.id).copied().unwrap_or([0.0; 6]);
        let d2 = disp_map.get(&n2.id).copied().unwrap_or([0.0; 6]);
        let dx0 = d1[Dof::Tx.index()] * deform_scale * SCALE_2D;
        let dy0 = -d1[Dof::Ty.index()] * deform_scale * SCALE_2D;
        let dx1 = d2[Dof::Tx.index()] * deform_scale * SCALE_2D;
        let dy1 = -d2[Dof::Ty.index()] * deform_scale * SCALE_2D;
        layers.push(json!({
            "kind": "polyline",
            "id": format!("deformed-{}", element_id(element)),
            "points": [[x0 + dx0, y0 + dy0], [x1 + dx1, y1 + dy1]],
            "color": "#f472b6",
        }));
    }
    layers
}
//#endregion 🔖️SharedDrawHelpers

//#region 🔖️Render
pub async fn render(doc: &Fem2dSnapshot, camera: &FemCamera) -> UiNode {
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
    build_canvas_2d_scene(BODY_KEY, crate::editor::fem2d::FEM2D_APP_ID, Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::fem2d::testkit::{fem2d_app, render as render_body};

    #[test]
    async fn renders_fem2d_model_scene() {
        let mut app = fem2d_app();
        assert!(render_body(&mut app, BODY_KEY).contains("canvas-2d"));
    }

    #[test]
    async fn mesh_preview_renders_region_edges() {
        let mut app = fem2d_app();
        crate::editor::fem2d::testkit::dispatch(&mut app, crate::editor::fem2d::Fem2dCommand::SetActiveExample(crate::editor::fem2d::commands::set_active_example::SetActiveExample { example_id: "default".into() }));
        let json = render_body(&mut app, BODY_KEY);
        assert!(json.contains("mesh-edge-"), "expected mesh-edge preview layers in the model scene");
    }

    #[test]
    async fn fem2d_model_extent_degenerate_model_returns_one() {
        assert_eq!(fem2d_model_extent(&crate::artifacts::fem2d::schema::empty_fem2d_snapshot()), 1.0);
    }
}
//#endregion 🧪️Tests
