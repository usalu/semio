//! 🧵️ Lowpoly artifact engine — mesh transfer payload construction and media export/import compute.
//! Split out of `🦀️component.rs` (the topic sibling file the plan's godfile-split convention allows for
//! a large engine).

use crate::artifacts::lowpoly::LowpolyProjection;
use semio_framework_plugin::MeshData;
use serde_json::Value;

//#region 🔖️MeshTransfer
/// @emoji 🧵️ Builds a `MeshData` transfer payload from a raw tessellation-transfer JSON value (as
/// produced by `LowpolyDocument::tessellate_transfer_json`), attaching a composited paint texture when
/// one is supplied. Shared by the app's live 3D scene builder and media export.
pub fn mesh_data_from_transfer(transfer: &Value, paint_texture: Option<String>) -> MeshData {
    let read_f32 = |key: &str| -> Vec<f32> { transfer.get(key).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default() };
    let read_u32 = |key: &str| -> Vec<u32> { transfer.get(key).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default() };
    let read_u8 = |key: &str| -> Vec<u8> { transfer.get(key).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default() };
    MeshData {
        positions: read_f32("positions"),
        normals: read_f32("normals"),
        indices: read_u32("indices"),
        uvs: read_f32("uvs"),
        face_ids: read_u32("faceIds"),
        vertex_ids: read_u32("vertexIds"),
        edge_positions: read_f32("edgePositions"),
        edge_ids: read_u32("edgeIds"),
        edge_uvs: read_f32("edgeUvs"),
        edge_is_seam: read_u8("edgeIsSeam"),
        paint_texture_base64: paint_texture,
        ..MeshData::default()
    }
}
//#endregion 🔖️MeshTransfer

//#region 🔖️MediaExportImport
/// 🔺️ Tessellates a lowpoly document's active object into a `MeshData` for media export.
pub fn lowpoly_mesh_from_document(doc: &Value) -> Result<MeshData, String> {
    let projection: LowpolyProjection = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
    let loaded = super::LowpolyDocument::new(projection).map_err(|e| e.to_string())?;
    Ok(loaded.active_mesh().ok().and_then(|mesh| super::LowpolyDocument::tessellate_transfer_json(mesh).ok()).map(|transfer| mesh_data_from_transfer(&transfer, None)).unwrap_or_default())
}

/// 🔺️ Rebuilds a fresh single-object lowpoly projection from a DWG-imported mesh.
pub fn lowpoly_document_from_mesh(mesh: &MeshData) -> Result<Value, String> {
    let halfedge = semio_s_3d::mesh::HalfedgeMesh::from_indexed_triangles(&mesh.positions, &mesh.indices).map_err(|err| format!("{err:?}"))?;
    let mesh_json = halfedge.to_json().map_err(|err| format!("{err:?}"))?;
    let projection = crate::artifacts::lowpoly::projection_from_mesh_json(&mesh_json, "obj-1", "Imported Mesh");
    serde_json::to_value(projection).map_err(|err| err.to_string())
}

/// 🧊️ Minimal document wrapper for `3d.mesh` resources — no dedicated schema exists yet.
pub fn mesh_document_from_mesh(mesh: &MeshData) -> Result<Value, String> {
    let mesh_value = serde_json::to_value(mesh).map_err(|err| err.to_string())?;
    Ok(serde_json::json!({ "schema": "mesh.document", "mesh": mesh_value }))
}

pub fn mesh_from_mesh_document(doc: &Value) -> Result<MeshData, String> {
    doc.get("mesh").and_then(|value| serde_json::from_value(value.clone()).ok()).filter(|mesh: &MeshData| !mesh.positions.is_empty() && !mesh.indices.is_empty()).map_or_else(|| Ok(semio_framework_plugin::mesh_from_kind("box")), Ok)
}
//#endregion 🔖️MediaExportImport

//#region 🔖️ExportConcreteForestMeshTests
#[cfg(test)]
mod export_concrete_forest_mesh_tests {
    use cad_plugin::artifacts::cad::engine::geometry_import::{objects_from_fixture_model, parse_geometry};
    use semio_s_3d::brep::kernel::Brep;
    use semio_s_3d::brep::engine::GeometryHandle;
    use semio_s_3d::mesh::{FaceId, HalfedgeMesh, Vec3 as MeshVec3, VertexId};
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

    #[test]
    fn export_concrete_forest_left_lowpoly_mesh_json() {
        if std::env::var("EXPORT_LOWPOLY_FOREST_MESH").ok().as_deref() != Some("1") {
            return;
        }
        let source = include_str!("../../../../../📐️cad/🖼️assets/🎮️play/🔣️hexagonal-cut-concrete-forest-left.model.json");
        let root: Value = serde_json::from_str(source).expect("fixture");
        let geometry = parse_geometry(root.pointer("/models/0/model/geometry"));
        let objects = root.pointer("/models/0/model/objects").and_then(|value| value.as_array()).cloned().unwrap_or_default();
        let mut kernel = Brep::new();
        let imported = objects_from_fixture_model(&mut kernel, &objects, &geometry);
        let handle = GeometryHandle(imported[0].solid_handle.clone().expect("handle"));
        let (positions, face_loops) = kernel.solid_face_loops_sync(&handle).expect("CAD face loops");
        let holed = face_loops.iter().filter(|(_, holes)| !holes.is_empty()).count();
        eprintln!("[DEBUG] CAD face loops: verts={} faces={} holed={}", positions.len(), face_loops.len(), holed);
        let mut mesh = HalfedgeMesh::from_face_loops(&positions, &face_loops).expect("halfedge from CAD wires");
        let flips = mesh.orient_faces_consistently().expect("orient faces");
        eprintln!("[DEBUG] after wire build+orient: verts={} faces={} flips={} open={}", mesh.vertex_count(), mesh.face_count(), flips, open_boundary_count(&mesh));
        let before_merge = mesh.face_count();
        let merges = mesh.merge_coplanar_faces().expect("merge coplanar faces");
        eprintln!("[DEBUG] after coplanar merge: verts={} faces={} merges={} (was {}) open={}", mesh.vertex_count(), mesh.face_count(), merges, before_merge, open_boundary_count(&mesh));
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
}
//#endregion 🔖️ExportConcreteForestMeshTests
