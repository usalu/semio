
use semio_framework_3d::mesh::{FaceId, HalfedgeMesh, Vec3 as MeshVec3, VertexId};
use semio_s_artifact_cad_cad::io::geometry_import::{objects_from_fixture_model, parse_geometry};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::{Brep, GeometryHandle};
use std::collections::HashMap;

/// Asserts every directed edge (by vertex id, after welding) has an opposite-winding counterpart, i.e. the
/// mesh has no open boundary loops.
fn assert_watertight(mesh: &HalfedgeMesh) {
    let mut directed: HashMap<(u32, u32), u32> = HashMap::new();
    for fi in 0..mesh.face_count() {
        let verts = mesh.face_vertex_ids(FaceId(fi as u32)).expect("face verts");
        let n = verts.len();
        for i in 0..n {
            *directed.entry((verts[i].0, verts[(i + 1) % n].0)).or_insert(0) += 1;
        }
    }
    let open: Vec<(u32, u32)> = directed.keys().copied().filter(|&(a, b)| !directed.contains_key(&(b, a))).collect();
    assert!(open.is_empty(), "mesh is not watertight: {} open boundary edges, e.g. {:?}", open.len(), &open[..open.len().min(5)]);
}

fn open_boundary_count(mesh: &HalfedgeMesh) -> usize {
    let mut directed: HashMap<(u32, u32), u32> = HashMap::new();
    for fi in 0..mesh.face_count() {
        let verts = mesh.face_vertex_ids(FaceId(fi as u32)).expect("face verts");
        let n = verts.len();
        for i in 0..n {
            *directed.entry((verts[i].0, verts[(i + 1) % n].0)).or_insert(0) += 1;
        }
    }
    directed.keys().filter(|&&(a, b)| !directed.contains_key(&(b, a))).count()
}

/// Spurious `fill_holes` caps on this solid spanned the open gap between vertical supports: large X
/// extent *and* large Z extent on one face. Real CAD faces are either horizontal slabs (small Δz) or
/// vertical support sides (small Δx).
fn assert_no_spanning_face_across_support_gap(mesh: &HalfedgeMesh) {
    for fi in 0..mesh.face_count() {
        let verts = mesh.face_vertex_ids(FaceId(fi as u32)).expect("face verts");
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_z = f32::MAX;
        let mut max_z = f32::MIN;
        for vid in verts {
            let p = mesh.vertex_position(vid).expect("vertex");
            min_x = min_x.min(p.x());
            max_x = max_x.max(p.x());
            min_z = min_z.min(p.z());
            max_z = max_z.max(p.z());
        }
        let dx = max_x - min_x;
        let dz = max_z - min_z;
        assert!(!(dx > 4.0 && dz > 1.0), "face {fi} spans the support gap (dx={dx:.3}, dz={dz:.3}) — likely a filled hole, not a CAD face");
    }
}

#[semio_framework_async_macros::async_test]
async fn export_concrete_forest_left_lowpoly_mesh_json() {
    if std::env::var("EXPORT_LOWPOLY_FOREST_MESH").ok().as_deref() != Some("1") {
        return;
    }
    let source = include_str!("../../../../../../../../../../📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🎮️play/🔣️.json");
    let root: serde_json::Value = serde_json::from_str(source).expect("fixture");
    let geometry = parse_geometry(root.pointer("/models/0/model/geometry"));
    let objects = root.pointer("/models/0/model/objects").and_then(|value| value.as_array()).cloned().unwrap_or_default();
    let mut kernel = Brep::new();
    let imported = objects_from_fixture_model(&mut kernel, &objects, &geometry);
    let handle = GeometryHandle(imported[0].solid_handle.clone().expect("handle"));
    let (positions, face_loops) = kernel.solid_face_loops_sync(&handle).expect("CAD face loops");
    let holed = face_loops.iter().filter(|(_, holes)| !holes.is_empty()).count();
    let mut mesh = HalfedgeMesh::from_face_loops(&positions, &face_loops).expect("halfedge from CAD wires");
    let flips = mesh.orient_faces_consistently().expect("orient faces");
    let before_merge = mesh.face_count();
    let merges = mesh.merge_coplanar_faces().expect("merge coplanar faces");
    assert!(mesh.face_count() <= before_merge, "coplanar merge must not increase face count");
    assert!(merges > 0 || before_merge == mesh.face_count(), "expected coplanar merge to join adjacent CAD faces on the plate/supports");
    assert!((0..mesh.face_count()).any(|fi| mesh.face_vertex_ids(FaceId(fi as u32)).map_or(0, |v| v.len()) > 3), "expected at least one non-triangle CAD face");
    assert_watertight(&mesh);
    assert_no_spanning_face_across_support_gap(&mesh);
    let mut min = MeshVec3::new(f32::MAX, f32::MAX, f32::MAX);
    let mut max = MeshVec3::new(f32::MIN, f32::MIN, f32::MIN);
    for index in 0..mesh.vertex_count() {
        let position = mesh.vertex_position(VertexId(index as u32)).expect("vertex");
        min = MeshVec3([min.x().min(position.x()), min.y().min(position.y()), min.z().min(position.z())]);
        max = MeshVec3([max.x().max(position.x()), max.y().max(position.y()), max.z().max(position.z())]);
    }
    let center = min.add(max).scale(0.5);
    mesh.translate(center.scale(-1.0)).expect("center mesh");
    let _ = mesh.unwrap_uv();
    let json = mesh.to_json().expect("mesh json");
    eprintln!("LOWPOLY_FOREST_MESH_JSON_START");
    eprintln!("{json}");
    eprintln!("LOWPOLY_FOREST_MESH_JSON_END");
}
