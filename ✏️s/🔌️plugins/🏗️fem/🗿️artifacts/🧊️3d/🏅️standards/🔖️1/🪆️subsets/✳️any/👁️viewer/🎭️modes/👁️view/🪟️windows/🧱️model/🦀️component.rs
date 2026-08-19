//! 🧱️ FEM 3D viewer — the `view` mode's Model window: a read-only World3d render of the undeformed
//! structure (nodes, bar/frame members, meshed solids) — the exact scene the editor's own Model window
//! renders (same `fem3d_scene_parts(doc, None, deformation_scale, None)` call: no displacement offset,
//! no stress coloring), rebuilt from scratch here rather than imported from the sibling editor module,
//! which `policyViewerPurityBreaches` forbids outright. Camera is a hardcoded default
//! (`crate::artifacts::fem3d::FemCamera::default()`) — a viewer has no persisted per-session camera
//! (`Config = NoConfig`). Mirrors fem3d's own editor style: the manifest declares this window with the
//! scalar `.window_kind(..)` builder call directly (see `crate::viewer::fem3d::create_fem3d_viewer`) —
//! no `WindowKindDefinition` object is built anywhere, so this node exports just its id/body-key
//! constants and `render()`.

use crate::artifacts::fem3d::{Fem3dSnapshot, FemCamera};
use crate::model::Dof;
use semio_framework_plugin::{build_world_3d_scene, world3d_default_selection_json, world3d_scene, UiNode, WorldSunConfig};
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖️Constants
/// 🪟️ The manifest's viewer Model window kind id.
pub const WINDOW_KIND_ID: &str = "fem3d-view-model";
/// 📄️ The viewer Model window's sole render body key.
pub const BODY_KEY: &str = "fem3d.view.model";
/// 👁️ Read-only counterpart of the editor's `FEM3D_APP_ID` controller id — kept distinct so a viewer
/// session's world-3d controller can never be mistaken for an editor session's.
const FEM3D_VIEW_CONTROLLER_ID: &str = "fem3d-view";
/// 🧊️ Half-extent-ish scale of the small box instance drawn at each node.
const NODE_SIZE_3D: f64 = 0.05;
/// 🧊️ Cross-section (x/y) thickness of the oriented box prism drawn for each `Bar`/`Frame` member.
const MEMBER_THICKNESS_3D: f64 = 0.05;
//#endregion 🔖️Constants

//#region 🔖️PureSceneBuild
/// 🧭️ Hamilton quaternion product `a * b`, both `[x,y,z,w]` — applying `b`'s rotation first, then `a`'s.
async fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    let (ax, ay, az, aw) = (a[0], a[1], a[2], a[3]);
    let (bx, by, bz, bw) = (b[0], b[1], b[2], b[3]);
    [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]
}

/// 🧭️ Rotation of `roll` radians about the LOCAL +Z axis — applied before `quat_z_to` reorients +Z to
/// the member direction, so this spins the box prism about its own long axis (matches `Frame3`'s roll).
async fn quat_roll_z(roll: f64) -> [f64; 4] {
    let h = roll / 2.0;
    [0.0, 0.0, h.sin(), h.cos()]
}

/// 🧭️ Shortest-arc rotation taking local `+Z` (the `"box"` mesh's long axis) onto unit direction `dir`.
async fn quat_z_to(dir: [f64; 3]) -> [f64; 4] {
    let dot = dir[2].clamp(-1.0, 1.0);
    if dot > 0.999_999 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    if dot < -0.999_999 {
        return [1.0, 0.0, 0.0, 0.0];
    }
    let axis = [-dir[1], dir[0], 0.0];
    let axis_len = (axis[0] * axis[0] + axis[1] * axis[1]).sqrt();
    let axis_n = [axis[0] / axis_len, axis[1] / axis_len, 0.0];
    let half = dot.acos() / 2.0;
    let s = half.sin();
    [axis_n[0] * s, axis_n[1] * s, axis_n[2] * s, half.cos()]
}

/// 🧊️ Node-position resolver shared by every 3D instance/mesh builder: `displacements` (node id -> 6-DOF
/// values), when present, offsets a node's position by its solved displacement scaled by `deform_scale`.
/// The viewer never passes `Some(displacements)` today (undeformed scene only) — kept general so this
/// stays a byte-for-byte twin of the editor's own helper.
async fn fem3d_deformed_position(pos: [f64; 3], node_id: &str, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64) -> [f64; 3] {
    let mut p = pos;
    if let Some(map) = displacements {
        if let Some(d) = map.get(node_id) {
            p[0] += d[Dof::Tx.index()] * deform_scale;
            p[1] += d[Dof::Ty.index()] * deform_scale;
            p[2] += d[Dof::Tz.index()] * deform_scale;
        }
    }
    p
}

async fn find_node_3d<'a>(nodes: &'a [crate::artifacts::fem3d::FemNode], id: &str) -> Option<&'a crate::artifacts::fem3d::FemNode> {
    nodes.iter().find(|n| n.id == id)
}

async fn fem3d_element_endpoints(element: &crate::artifacts::fem3d::FemElement) -> (&str, &str) {
    match element {
        crate::artifacts::fem3d::FemElement::Bar { start, end, .. } | crate::artifacts::fem3d::FemElement::Frame { start, end, .. } => (start.as_str(), end.as_str()),
    }
}

/// 🧊️ One small box instance per node, plus one ORIENTED box prism per `Bar`/`Frame` member.
async fn fem3d_structural_instances(doc: &Fem3dSnapshot, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64) -> Vec<Value> {
    let node_pos = |node: &crate::artifacts::fem3d::FemNode| fem3d_deformed_position([node.x, node.y, node.z], &node.id, displacements, deform_scale);

    let mut instances: Vec<Value> = Vec::new();
    for node in &doc.nodes {
        let p = node_pos(node);
        instances.push(json!({
            "id": format!("node-{}", node.id),
            "meshId": "box",
            "position": p,
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [NODE_SIZE_3D, NODE_SIZE_3D, NODE_SIZE_3D],
            "label": node.id,
        }));
    }
    for element in &doc.elements {
        let (start, end) = fem3d_element_endpoints(element);
        let (Some(n1), Some(n2)) = (find_node_3d(&doc.nodes, start), find_node_3d(&doc.nodes, end)) else { continue };
        let p1 = node_pos(n1);
        let p2 = node_pos(n2);
        let d = [p2[0] - p1[0], p2[1] - p1[1], p2[2] - p1[2]];
        let length = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(1e-9);
        let dir = [d[0] / length, d[1] / length, d[2] / length];
        let roll = match element {
            crate::artifacts::fem3d::FemElement::Frame { roll, .. } => *roll,
            crate::artifacts::fem3d::FemElement::Bar { .. } => 0.0,
        };
        let rotation = quat_mul(quat_z_to(dir), quat_roll_z(roll));
        let mid = [(p1[0] + p2[0]) / 2.0, (p1[1] + p2[1]) / 2.0, (p1[2] + p2[2]) / 2.0];
        let id = crate::artifacts::fem3d::element_id(element);
        instances.push(json!({
            "id": format!("el-{id}"),
            "meshId": "box",
            "position": mid,
            "rotation": rotation,
            "scale": [MEMBER_THICKNESS_3D, MEMBER_THICKNESS_3D, length],
            "label": id,
        }));
    }
    instances
}

/// 🧱️ Every `FemSolid`'s boundary surface as a custom `meshes_json` entry (flat per-face normals) plus
/// its one identity-transform instance — the viewer never passes `nodal_stress` (undeformed, uncolored
/// scene only), so every vertex gets the same neutral gray. `crate::fem3d_engine::mesh_preview` and
/// `crate::app_surface::{hex_to_rgb01, von_mises_color}` are crate-root shared compute (not app-owned),
/// safe to call directly from a viewer file.
async fn fem3d_solid_mesh_entries(doc: &Fem3dSnapshot, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64, nodal_stress: Option<&HashMap<String, f64>>) -> (Vec<Value>, Vec<Value>) {
    use crate::app_surface::{hex_to_rgb01, von_mises_color};

    let mut meshes = Vec::new();
    let mut instances = Vec::new();
    let Ok(solid_meshes) = crate::fem3d_engine::mesh_preview::fem3d_mesh_preview(doc) else { return (meshes, instances) };
    let (min, max) = match nodal_stress {
        Some(map) if !map.is_empty() => (map.values().cloned().fold(f64::INFINITY, f64::min), map.values().cloned().fold(f64::NEG_INFINITY, f64::max)),
        _ => (0.0, 1.0),
    };

    for solid in &solid_meshes {
        let mut positions: Vec<f64> = Vec::with_capacity(solid.boundary_tris.len() * 9);
        let mut normals: Vec<f64> = Vec::with_capacity(solid.boundary_tris.len() * 9);
        let mut colors: Vec<f64> = Vec::with_capacity(solid.boundary_tris.len() * 9);
        let mut indices: Vec<u32> = Vec::with_capacity(solid.boundary_tris.len() * 3);

        let vertex_pos = |idx: u32| -> [f64; 3] { fem3d_deformed_position(solid.points[idx as usize], &solid.node_ids[idx as usize], displacements, deform_scale) };
        let vertex_color = |idx: u32| -> (f64, f64, f64) {
            let Some(stress_map) = nodal_stress else { return (0.78, 0.78, 0.8) };
            let value = stress_map.get(&solid.node_ids[idx as usize]).copied().unwrap_or(min);
            hex_to_rgb01(von_mises_color(value, min, max))
        };

        for &[a, b, c] in &solid.boundary_tris {
            let (pa, pb, pc) = (vertex_pos(a), vertex_pos(b), vertex_pos(c));
            let e0 = [pb[0] - pa[0], pb[1] - pa[1], pb[2] - pa[2]];
            let e1 = [pc[0] - pa[0], pc[1] - pa[1], pc[2] - pa[2]];
            let raw = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
            let raw_len = (raw[0] * raw[0] + raw[1] * raw[1] + raw[2] * raw[2]).sqrt().max(1e-12);
            let n = [raw[0] / raw_len, raw[1] / raw_len, raw[2] / raw_len];
            let base = (positions.len() / 3) as u32;
            for (idx, p) in [(a, pa), (b, pb), (c, pc)] {
                positions.extend_from_slice(&p);
                normals.extend_from_slice(&n);
                let (r, g, bl) = vertex_color(idx);
                colors.extend_from_slice(&[r, g, bl]);
            }
            indices.extend_from_slice(&[base, base + 1, base + 2]);
        }

        let mesh_id = format!("solid-{}", solid.solid_id);
        meshes.push(json!({ "id": mesh_id, "data": { "positions": positions, "normals": normals, "colors": colors, "indices": indices } }));
        instances.push(json!({
            "id": format!("solid-inst-{}", solid.solid_id),
            "meshId": mesh_id,
            "position": [0.0, 0.0, 0.0],
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [1.0, 1.0, 1.0],
            "label": solid.solid_id,
        }));
    }
    (meshes, instances)
}

/// 🧊️ Builds the FULL `(meshes_json, instances_json)` pair for the undeformed structure — the `"box"`
/// primitive mesh plus every `FemSolid`'s custom surface mesh, and every node/member/solid instance.
async fn fem3d_scene_parts(doc: &Fem3dSnapshot, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64, nodal_stress: Option<&HashMap<String, f64>>) -> (String, String) {
    let mut meshes: Vec<Value> = serde_json::from_str(&semio_framework_plugin::world3d_meshes_json_from_kinds(&["box".to_string()])).unwrap_or_default();
    let mut instances = fem3d_structural_instances(doc, displacements, deform_scale);
    let (solid_meshes, solid_instances) = fem3d_solid_mesh_entries(doc, displacements, deform_scale, nodal_stress);
    meshes.extend(solid_meshes);
    instances.extend(solid_instances);
    (serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into()))
}

/// 🎥️ Resolves a `FemCamera` to its JSON string, falling back to the framework's default 3D camera when
/// the sentinel empty-object placeholder is still set — the viewer always calls this with
/// `FemCamera::default()`, so this always takes the fallback branch today, kept general to stay a
/// byte-for-byte twin of the editor's own helper.
async fn fem3d_camera_json(camera: &FemCamera) -> String {
    if camera.json == "{}" {
        semio_framework_plugin::world3d_default_camera()
    } else {
        camera.json.clone()
    }
}
//#endregion 🔖️PureSceneBuild

//#region 🔖️Render
/// 🧱️ Renders the undeformed structure with a hardcoded default camera — no persisted per-session
/// camera (`Config = NoConfig`), no displacement offset, no stress coloring: the exact same scene the
/// editor's own Model window renders for the same document.
pub async fn render(doc: &Fem3dSnapshot) -> UiNode {
    let camera = FemCamera::default();
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, None, doc.analysis.deformation_scale, None);
    build_world_3d_scene(BODY_KEY, FEM3D_VIEW_CONTROLLER_ID, world3d_scene(fem3d_camera_json(&camera), meshes_json, instances_json, world3d_default_selection_json(), &WorldSunConfig::default()))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn quat_z_to_identity_for_parallel_direction() {
        assert_eq!(quat_z_to([0.0, 0.0, 1.0]), [0.0, 0.0, 0.0, 1.0]);
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_a_scene_node_for_the_bundled_example() {
        let document = crate::artifacts::fem3d::dsl::parse_dsl(crate::artifacts::fem3d::dsl::FEM3D_EXAMPLE_TEXT).expect("example fixture parses");
        let json = serde_json::to_string(&render(&document)).expect("render json");
        assert!(json.contains("world-3d"));
        assert!(json.contains("solid-sol1"), "expected the example fixture's solid mesh: {json}");
        assert!(json.contains("el-e1"), "expected a single oriented box instance per member: {json}");
    }
}
//#endregion 🧪️Tests
