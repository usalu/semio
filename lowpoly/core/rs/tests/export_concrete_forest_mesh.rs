#[path = "../../../../cad/plugin/rs/geometry_import.rs"]
mod geometry_import;

use geometry_import::{objects_from_fixture_model, parse_geometry, tessellate_geometry_handle};
use kernel_3d_brepkit::BrepkitKernel;
use kernel_3d_mesh::{FaceId, HalfedgeMesh, Vec3 as MeshVec3, VertexId};
use serde_json::Value;
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

#[test]
fn export_concrete_forest_left_lowpoly_mesh_json() {
    if std::env::var("EXPORT_LOWPOLY_FOREST_MESH").ok().as_deref() != Some("1") {
        return;
    }
    let source = include_str!("../../../../cad/asset/play/hexagonal-cut-concrete-forest-left.model.json");
    let root: Value = serde_json::from_str(source).expect("fixture");
    let geometry = parse_geometry(root.pointer("/models/0/model/geometry"));
    let objects = root
        .pointer("/models/0/model/objects")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();
    let mut kernel = BrepkitKernel::new();
    let imported = objects_from_fixture_model(&mut kernel, &objects, &geometry);
    let mesh_data = tessellate_geometry_handle(
        &mut kernel,
        imported[0].solid_handle.as_ref().expect("handle"),
        "solid",
    )
    .expect("tessellated mesh");
    let mut mesh = HalfedgeMesh::from_indexed_triangles(&mesh_data.positions, &mesh_data.indices)
        .expect("halfedge mesh");
    // The BREP tessellator emits independently-tessellated, non-shared vertices along seams between
    // adjacent source faces; weld those before touching topology so twins/boundaries are accurate.
    mesh.weld_coincident_vertices(1e-4).expect("weld coincident vertices");
    // Any remaining true boundary loops (e.g. an unclosed cut cross-section) get capped so the solid is
    // watertight before merging — capping first lets the new cap faces also join the coplanar merge below.
    mesh.fill_holes().expect("fill holes");
    let triangle_face_count = mesh.face_count();
    mesh.merge_coplanar_faces().expect("merge coplanar faces");
    assert!(
        mesh.face_count() < triangle_face_count,
        "expected coplanar merge to reduce face count below {triangle_face_count}, got {}",
        mesh.face_count()
    );
    assert!(
        (0..mesh.face_count()).any(|fi| mesh.face_vertex_ids(kernel_3d_mesh::FaceId(fi as u32)).map(|v| v.len()).unwrap_or(0) > 4),
        "expected at least one merged face with more than 4 corners"
    );
    assert_watertight(&mesh);
    let mut min = MeshVec3::new(f32::MAX, f32::MAX, f32::MAX);
    let mut max = MeshVec3::new(f32::MIN, f32::MIN, f32::MIN);
    for index in 0..mesh.vertex_count() {
        let position = mesh.vertex_position(VertexId(index as u32)).expect("vertex");
        min = MeshVec3([
            min.x().min(position.x()),
            min.y().min(position.y()),
            min.z().min(position.z()),
        ]);
        max = MeshVec3([
            max.x().max(position.x()),
            max.y().max(position.y()),
            max.z().max(position.z()),
        ]);
    }
    let center = min.add(max).scale(0.5);
    mesh.translate(center.scale(-1.0)).expect("center mesh");
    let _ = mesh.unwrap_uv();
    let json = mesh.to_json().expect("mesh json");
    eprintln!("LOWPOLY_FOREST_MESH_JSON_START");
    eprintln!("{json}");
    eprintln!("LOWPOLY_FOREST_MESH_JSON_END");
}
