//! 🎬️ Fem3d — the `World3d` scene the editor's windows and the viewer draw: the `"box"`/`"cone"` primitives plus
//! every solid's surface mesh, and one pickable instance per document entity — nodes and members
//! as boxes, solids as their meshed surface, supports as cones under their node, loads as arrows —
//! every instance keyed by the RAW entity id it stands for and tagged with its `fem3d` granularity,
//! so a `World3dHost` raycast hit IS the framework `interactionSelect` the artifact tree and the
//! inspector already speak. Every builder takes the same optional displacement field the results
//! views deform with, so a load arrow rides its node into the deformed shape. Lives beside the
//! editor and the viewer (not inside either) because both surfaces draw it and a viewer may not
//! import through its sibling editor.

use crate::fem3d_engine::mesh_preview;
use crate::model::Dof;
use crate::{element_id, load_id, Fem3dSnapshot, FemAxis, FemDof, FemElement, FemLoad, FemSolid};
use dsl::json::Value;
use std::collections::HashMap;

//#region 🔖️Granularities
/// 🕹️ The `fem3d` interaction domain's granularity ids — one per document entity kind; declared
/// here because every scene instance names the one it stands for.
pub const FEM3D_INTERACTION_DOMAIN: &str = "fem3d";
pub const FEM3D_GRANULARITY_NODE: &str = "node";
pub const FEM3D_GRANULARITY_ELEMENT: &str = "element";
pub const FEM3D_GRANULARITY_SOLID: &str = "solid";
pub const FEM3D_GRANULARITY_SUPPORT: &str = "support";
pub const FEM3D_GRANULARITY_LOAD: &str = "load";
pub const FEM3D_GRANULARITY_MATERIAL: &str = "material";
pub const FEM3D_GRANULARITY_SECTION: &str = "section";
pub const FEM3D_GRANULARITY_LOAD_CASE: &str = "loadCase";
pub const FEM3D_GRANULARITY_COMBINATION: &str = "combination";
//#endregion 🔖️Granularities

//#region 🔖️Geometry
/// 📍️ A node's world position.
pub fn fem3d_node_point(doc: &Fem3dSnapshot, id: &str) -> Option<[f64; 3]> {
    doc.nodes.iter().find(|node| node.id == id).map(|node| [node.x, node.y, node.z])
}

/// 🔩️ A member's two node ids.
pub fn fem3d_element_endpoints(element: &FemElement) -> (&str, &str) {
    match element {
        FemElement::Bar { start, end, .. } | FemElement::Frame { start, end, .. } => (start.as_str(), end.as_str()),
    }
}

/// 🧊️ A solid's world centroid — the mean of its outline vertices lifted to mid-extrusion, which is
/// where its gumball pivot sits and where `focusEntity` looks.
pub fn fem3d_solid_centroid(solid: &FemSolid) -> Option<[f64; 3]> {
    if solid.outline.is_empty() {
        return None;
    }
    let sum = solid.outline.iter().fold([0.0, 0.0], |sum, point| [sum[0] + point[0], sum[1] + point[1]]);
    let count = solid.outline.len() as f64;
    Some(solid.axis.to_world(sum[0] / count, sum[1] / count, solid.base_z + solid.height * 0.5))
}

/// 🧊️ The world point on a solid's upward-facing surface a surface load glyph anchors to: the
/// footprint centroid lifted to the solid's top along the world `+Z` — the extrusion end for a
/// `Z` solid, the upper extent of the footprint plane otherwise.
pub fn fem3d_solid_top_point(solid: &FemSolid) -> Option<[f64; 3]> {
    let centroid = fem3d_solid_centroid(solid)?;
    let top = match solid.axis {
        FemAxis::Z => solid.base_z + solid.height,
        _ => solid.outline.iter().map(|point| point[1]).fold(f64::NEG_INFINITY, f64::max),
    };
    Some([centroid[0], centroid[1], top])
}
//#endregion 🔖️Geometry

//#region 🔖️Quaternions
/// 🧭️ Hamilton quaternion product `a * b`, both `[x,y,z,w]` — applying `b`'s rotation first, then `a`'s.
pub fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    let (ax, ay, az, aw) = (a[0], a[1], a[2], a[3]);
    let (bx, by, bz, bw) = (b[0], b[1], b[2], b[3]);
    [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]
}

/// 🧭️ Rotation of `roll` radians about the LOCAL +Z axis — applied before `quat_z_to` reorients +Z to
/// the member direction, so this spins the box prism about its own long axis (matches `Frame3`'s roll).
pub fn quat_roll_z(roll: f64) -> [f64; 4] {
    let h = roll / 2.0;
    [0.0, 0.0, h.sin(), h.cos()]
}

/// 🧭️ Shortest-arc rotation taking local `+Z` (the `"box"` mesh's long axis) onto unit direction `dir`
/// — the standard "rotate A onto B" quaternion (`axis = cross(from,to)`, `angle = acos(dot(from,to))`),
/// specialized for `from = (0,0,1)` so `cross` reduces to `(-dir.y, dir.x, 0)`. Handles the antiparallel
/// case (`dir ≈ (0,0,-1)`) with a fixed 180° flip about the X axis, since `cross` degenerates to zero there.
pub fn quat_z_to(dir: [f64; 3]) -> [f64; 4] {
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

/// 🧭️ Shortest-arc rotation taking local `+Y` (the `"cone"` mesh's axis, base at the origin and apex
/// at `+1`) onto unit direction `dir` — `quat_z_to`'s twin for the one built-in kind that is Y-up.
pub fn quat_y_to(dir: [f64; 3]) -> [f64; 4] {
    let dot = dir[1].clamp(-1.0, 1.0);
    if dot > 0.999_999 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    if dot < -0.999_999 {
        return [0.0, 0.0, 1.0, 0.0];
    }
    let axis = [dir[2], 0.0, -dir[0]];
    let axis_len = (axis[0] * axis[0] + axis[2] * axis[2]).sqrt();
    let axis_n = [axis[0] / axis_len, 0.0, axis[2] / axis_len];
    let half = dot.acos() / 2.0;
    let s = half.sin();
    [axis_n[0] * s, axis_n[1] * s, axis_n[2] * s, half.cos()]
}
//#endregion 🔖️Quaternions

//#region 🔖️Sizes
/// 🧊️ Half-extent-ish scale of the small box instance drawn at each node.
pub const NODE_SIZE_3D: f64 = 0.05;
/// 🧊️ Cross-section (x/y) thickness of the oriented box prism drawn for each `Bar`/`Frame` member —
/// a fixed visual thickness, not the member's actual section dimensions.
pub const MEMBER_THICKNESS_3D: f64 = 0.05;
/// 🛡️ A support cone's height and base diameter.
pub const SUPPORT_SIZE_3D: f64 = 0.24;
/// 🏋️ A load arrow's overall length, head length and shaft thickness.
pub const LOAD_ARROW_LENGTH_3D: f64 = 0.6;
pub const LOAD_ARROW_HEAD_3D: f64 = 0.18;
pub const LOAD_ARROW_SHAFT_3D: f64 = 0.03;
/// 🏋️ How many arrows a member load is drawn with along its member.
pub const MEMBER_LOAD_ARROWS_3D: usize = 3;
//#endregion 🔖️Sizes

//#region 🔖️Instances
/// 🧊️ Node-position resolver shared by every 3D instance/mesh builder: `displacements` (node id -> 6-DOF
/// values), when present, offsets a node's position by its solved displacement scaled by `deform_scale`.
pub fn fem3d_deformed_position(pos: [f64; 3], node_id: &str, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64) -> [f64; 3] {
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

fn instance(id: &str, granularity: &str, mesh_id: &str, position: [f64; 3], rotation: [f64; 4], scale: [f64; 3], label: &str) -> Value {
    dsl::json!({
        "id": id,
        "interactionGranularityId": granularity,
        "meshId": mesh_id,
        "position": position,
        "rotation": rotation,
        "scale": scale,
        "label": label,
    })
}

/// 🏹️ One arrow glyph pointing along unit `direction` and ENDING at `tip`: a cone head whose apex is
/// the tip and a box shaft behind it. Both instances redirect their hit onto `interaction_id`.
fn arrow_instances(id_prefix: &str, interaction_id: &str, tip: [f64; 3], direction: [f64; 3], length: f64, label: &str, out: &mut Vec<Value>) {
    let head_base = [tip[0] - direction[0] * LOAD_ARROW_HEAD_3D, tip[1] - direction[1] * LOAD_ARROW_HEAD_3D, tip[2] - direction[2] * LOAD_ARROW_HEAD_3D];
    let shaft_length = (length - LOAD_ARROW_HEAD_3D).max(LOAD_ARROW_HEAD_3D);
    let shaft_centre = [head_base[0] - direction[0] * shaft_length * 0.5, head_base[1] - direction[1] * shaft_length * 0.5, head_base[2] - direction[2] * shaft_length * 0.5];
    let mut head = instance(&format!("{id_prefix}:head"), FEM3D_GRANULARITY_LOAD, "cone", head_base, quat_y_to(direction), [LOAD_ARROW_HEAD_3D * 0.6, LOAD_ARROW_HEAD_3D, LOAD_ARROW_HEAD_3D * 0.6], label);
    let mut shaft = instance(&format!("{id_prefix}:shaft"), FEM3D_GRANULARITY_LOAD, "box", shaft_centre, quat_z_to(direction), [LOAD_ARROW_SHAFT_3D, LOAD_ARROW_SHAFT_3D, shaft_length], label);
    for glyph in [&mut head, &mut shaft] {
        if let Some(object) = glyph.as_object_mut() {
            object.insert("interactionId", dsl::json!(interaction_id));
        }
    }
    out.push(head);
    out.push(shaft);
}

/// 🧭️ The unit world direction a signed load along `dof` points in, or `None` for a moment.
fn dof_direction(dof: FemDof, value: f64) -> Option<[f64; 3]> {
    let sign = if value < 0.0 { -1.0 } else { 1.0 };
    match dof {
        FemDof::Tx => Some([sign, 0.0, 0.0]),
        FemDof::Ty => Some([0.0, sign, 0.0]),
        FemDof::Tz => Some([0.0, 0.0, sign]),
        FemDof::Rx | FemDof::Ry | FemDof::Rz => None,
    }
}

/// 🧊️ One small box instance per node, plus one ORIENTED box prism per `Bar`/`Frame` member — position
/// at the (possibly deformed) midpoint, `scale=[t,t,length]` so the mesh's own long (local Z) axis
/// stretches along the member, `rotation` a quaternion aligning that axis to the member's direction
/// (composed with a `Frame`'s own `roll` about its own axis; `Bar`s have no roll).
pub fn fem3d_structural_instances(doc: &Fem3dSnapshot, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64) -> Vec<Value> {
    let node_pos = |node: &crate::FemNode| fem3d_deformed_position([node.x, node.y, node.z], &node.id, displacements, deform_scale);
    let mut instances: Vec<Value> = Vec::new();
    for node in &doc.nodes {
        instances.push(instance(&node.id, FEM3D_GRANULARITY_NODE, "box", node_pos(node), [0.0, 0.0, 0.0, 1.0], [NODE_SIZE_3D, NODE_SIZE_3D, NODE_SIZE_3D], &node.id));
    }
    for element in &doc.elements {
        let (start, end) = fem3d_element_endpoints(element);
        let (Some(n1), Some(n2)) = (doc.nodes.iter().find(|n| n.id == start), doc.nodes.iter().find(|n| n.id == end)) else { continue };
        let p1 = node_pos(n1);
        let p2 = node_pos(n2);
        let d = [p2[0] - p1[0], p2[1] - p1[1], p2[2] - p1[2]];
        let length = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(1e-9);
        let dir = [d[0] / length, d[1] / length, d[2] / length];
        let roll = match element {
            crate::FemElement::Frame { roll, .. } => *roll,
            crate::FemElement::Bar { .. } => 0.0,
        };
        let rotation = quat_mul(quat_z_to(dir), quat_roll_z(roll));
        let mid = [(p1[0] + p2[0]) / 2.0, (p1[1] + p2[1]) / 2.0, (p1[2] + p2[2]) / 2.0];
        let id = element_id(element);
        instances.push(instance(id, FEM3D_GRANULARITY_ELEMENT, "box", mid, rotation, [MEMBER_THICKNESS_3D, MEMBER_THICKNESS_3D, length], id));
    }
    instances
}

/// 🛡️ One cone per support, apex at the supported node and base hanging below it.
pub fn fem3d_support_instances(doc: &Fem3dSnapshot, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64) -> Vec<Value> {
    let mut instances = Vec::new();
    for support in &doc.supports {
        let Some(node) = doc.nodes.iter().find(|node| node.id == support.node_id) else { continue };
        let apex = fem3d_deformed_position([node.x, node.y, node.z], &node.id, displacements, deform_scale);
        let base = [apex[0], apex[1], apex[2] - SUPPORT_SIZE_3D];
        instances.push(instance(&support.id, FEM3D_GRANULARITY_SUPPORT, "cone", base, quat_y_to([0.0, 0.0, 1.0]), [SUPPORT_SIZE_3D, SUPPORT_SIZE_3D, SUPPORT_SIZE_3D], &support.id));
    }
    instances
}

/// 🏋️ Arrow glyphs for every load of every case: a nodal force as one arrow ending at its node
/// (a moment as a short arrow about its axis), a member load as [`MEMBER_LOAD_ARROWS_3D`] arrows
/// along the member, a surface pressure as one arrow ending on the solid's top.
pub fn fem3d_load_instances(doc: &Fem3dSnapshot, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64) -> Vec<Value> {
    let mut instances = Vec::new();
    let position = |id: &str| fem3d_node_point(doc, id).map(|point| fem3d_deformed_position(point, id, displacements, deform_scale));
    for case in &doc.load_cases {
        for load in &case.loads {
            let id = load_id(load);
            match load {
                FemLoad::Nodal { node_id, dof, value, .. } => {
                    let Some(tip) = position(node_id) else { continue };
                    let direction = dof_direction(*dof, *value).unwrap_or([0.0, 0.0, 1.0]);
                    arrow_instances(id, id, tip, direction, LOAD_ARROW_LENGTH_3D, id, &mut instances);
                }
                FemLoad::MemberUdl { element_id: target, wx, wy, wz, .. } => {
                    let Some(element) = doc.elements.iter().find(|element| element_id(element) == target) else { continue };
                    let (start, end) = fem3d_element_endpoints(element);
                    let (Some(a), Some(b)) = (position(start), position(end)) else { continue };
                    let magnitude = (wx * wx + wy * wy + wz * wz).sqrt();
                    let direction = if magnitude > 1e-12 { [wx / magnitude, wy / magnitude, wz / magnitude] } else { [0.0, 0.0, -1.0] };
                    for index in 0..MEMBER_LOAD_ARROWS_3D {
                        let t = (index as f64 + 0.5) / MEMBER_LOAD_ARROWS_3D as f64;
                        let tip = [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t];
                        arrow_instances(&format!("{id}#{index}"), id, tip, direction, LOAD_ARROW_LENGTH_3D * 0.7, id, &mut instances);
                    }
                }
                FemLoad::Area { solid_id, pressure, .. } => {
                    let Some(tip) = doc.solids.iter().find(|solid| &solid.id == solid_id).and_then(fem3d_solid_top_point) else { continue };
                    let direction = if *pressure < 0.0 { [0.0, 0.0, 1.0] } else { [0.0, 0.0, -1.0] };
                    arrow_instances(id, id, tip, direction, LOAD_ARROW_LENGTH_3D, id, &mut instances);
                }
            }
        }
    }
    instances
}

/// 🧱️ Every `FemSolid`'s boundary surface as a custom `meshes_json` entry (flat per-face normals, one
/// duplicated vertex triple per triangle) plus its one identity-transform instance keyed by the
/// solid's own id — `nodal_stress`, when present, colors each vertex by
/// `crate::app_surface::von_mises_color` (min/max taken across ALL solids' averaged values), driving
/// the react renderer's vertex-color contour (see `PaintTexturedMesh`). `displacements` deforms vertex
/// positions the same way `fem3d_structural_instances` deforms node/member instances.
pub fn fem3d_solid_mesh_entries(doc: &Fem3dSnapshot, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64, nodal_stress: Option<&HashMap<String, f64>>) -> (Vec<Value>, Vec<Value>) {
    use crate::app_surface::{hex_to_rgb01, von_mises_color};

    let mut meshes = Vec::new();
    let mut instances = Vec::new();
    let Ok(solid_meshes) = mesh_preview::fem3d_mesh_preview(doc) else { return (meshes, instances) };
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
        meshes.push(dsl::json!({ "id": mesh_id, "data": { "positions": positions, "normals": normals, "colors": colors, "indices": indices } }));
        instances.push(instance(&solid.solid_id, FEM3D_GRANULARITY_SOLID, &mesh_id, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0], &solid.solid_id));
    }
    (meshes, instances)
}

/// 🧊️ Builds the FULL `(meshes_json, instances_json)` pair for a 3D scene: the `"box"` and `"cone"`
/// primitive meshes plus every `FemSolid`'s custom surface mesh, and every node/member/solid/support/
/// load instance — shared by the model window and every results view (static/modal/buckling).
pub fn fem3d_scene_parts(doc: &Fem3dSnapshot, displacements: Option<&HashMap<String, [f64; 6]>>, deform_scale: f64, nodal_stress: Option<&HashMap<String, f64>>) -> (String, String) {
    let mut meshes = dsl::json::parse(&semio_framework_plugin::world3d_meshes_json_from_kinds(&["box".to_string(), "cone".to_string()])).ok().and_then(|value| value.as_array().cloned()).unwrap_or_default();
    let mut instances = fem3d_structural_instances(doc, displacements, deform_scale);
    let (solid_meshes, solid_instances) = fem3d_solid_mesh_entries(doc, displacements, deform_scale, nodal_stress);
    meshes.extend(solid_meshes);
    instances.extend(solid_instances);
    instances.extend(fem3d_support_instances(doc, displacements, deform_scale));
    instances.extend(fem3d_load_instances(doc, displacements, deform_scale));
    (dsl::json::to_string(&Value::Array(meshes)), dsl::json::to_string(&Value::Array(instances)))
}
//#endregion 🔖️Instances

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
