use crate::{mesh_child_handle, LowpolyObject, LowpolyPaintLayer, LowpolySnapshot, LowpolyTransform, LOWPOLY_DOCUMENT_SCHEMA};

/// 🧪️ A small (deliberately NOT the real `LOWPOLY_PAINT_TEXTURE_SIZE`-scoped canvas) but
/// structurally representative fixture: two objects, one carrying a mesh handle and a paint
/// layer, the other bare -- exercising every field kind `LowpolySnapshot` actually has.
fn box_mesh_json() -> String {
    semio_framework_3d::mesh::HalfedgeMesh::box_prim(1.0, 1.0, 1.0).expect("box prim").to_json().expect("mesh json")
}

fn fixture() -> LowpolySnapshot {
    let mesh_json = box_mesh_json();
    let obj1 = LowpolyObject {
        id: "obj-1".into(),
        name: "First Object".into(),
        transform: LowpolyTransform { position: [1.0, 2.0, 3.0], rotation: [0.0, 90.0, 0.0], scale: [1.0, 1.0, 1.0] },
        smooth_shading: false,
        mesh: Some(mesh_child_handle("obj-1", &mesh_json)),
        paint_layers: vec![LowpolyPaintLayer { name: "Base".into(), visible: true, opacity: 0.5, blend_mode: "normal".into(), pixels: vec![10, 20, 30, 40, 50, 60, 70, 80] }],
        mesh_content: mesh_json,
    };
    let obj2 = LowpolyObject { id: "obj-2".into(), name: "Second Object".into(), transform: LowpolyTransform::default(), smooth_shading: true, mesh: None, paint_layers: Vec::new(), mesh_content: String::new() };
    LowpolySnapshot { schema: LOWPOLY_DOCUMENT_SCHEMA.into(), objects: vec![obj1, obj2] }
}

#[semio_framework_async_macros::async_test]
async fn txt_export_import_round_trips_the_snapshot() {
    let snapshot = fixture();
    let bytes = crate::io::export::serializers::artifacts::txt::v_utf_8::any::serialize_bytes(&snapshot).expect("txt export");
    let recovered = crate::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes).expect("txt import");
    assert_eq!(snapshot, recovered);
}

#[semio_framework_async_macros::async_test]
async fn json_export_import_round_trips_the_snapshot() {
    let snapshot = fixture();
    let bytes = crate::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).expect("json export");
    let recovered = crate::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).expect("json import");
    assert_eq!(snapshot, recovered);
}

#[semio_framework_async_macros::async_test]
async fn obj_export_import_round_trips_the_snapshot() {
    let snapshot = fixture();
    let bytes = crate::io::export::serializers::artifacts::obj::v3_0::any::serialize_bytes(&snapshot).expect("obj export");
    let recovered = crate::io::import::deserializers::artifacts::obj::v3_0::any::deserialize_bytes(&bytes).expect("obj import");
    assert_eq!(snapshot, recovered);
}

#[semio_framework_async_macros::async_test]
async fn png_export_import_round_trips_the_snapshot() {
    let snapshot = fixture();
    let bytes = crate::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).expect("png export");
    let recovered = crate::io::import::deserializers::artifacts::png::v1_2::any::deserialize_bytes(&bytes).expect("png import");
    assert_eq!(snapshot, recovered);
}

#[semio_framework_async_macros::async_test]
async fn ply_export_import_round_trips_the_snapshot() {
    let snapshot = fixture();
    let bytes = crate::io::export::serializers::artifacts::ply::v1_0::any::serialize_bytes(&snapshot).expect("ply export");
    let recovered = crate::io::import::deserializers::artifacts::ply::v1_0::any::deserialize_bytes(&bytes).expect("ply import");
    assert_eq!(snapshot, recovered);
}

/// 🚫️ gltf/dwg/las are honest stubs (see those leaves' own doc comments) -- proves they fail
/// LOUDLY, never silently miscompile as a pack-envelope-mismatch lie.
#[semio_framework_async_macros::async_test]
async fn unimplemented_geometry_formats_error_honestly_instead_of_lying() {
    let snapshot = fixture();
    assert!(crate::io::export::serializers::artifacts::gltf::v2_0::any::serialize_bytes(&snapshot).is_err());
    assert!(crate::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).is_err());
    assert!(crate::io::export::serializers::artifacts::las::v1_0::any::serialize_bytes(&snapshot).is_err());
}

/// 🚫️ Import-direction counterpart: gltf/dwg/las deserializers are honest stubs too (stl now
/// parses real geometry, and still rejects garbage loudly)
/// (creating a resolvable mesh child artifact from parsed geometry needs a store/session
/// handle, not available to a synchronous `&[u8] -> LowpolySnapshot` function) -- arbitrary
/// input bytes must error, never silently fabricate a geometry-less `LowpolySnapshot`.
#[semio_framework_async_macros::async_test]
async fn unimplemented_geometry_import_formats_error_honestly_instead_of_lying() {
    let bytes = b"not a real payload, contents are irrelevant -- these stubs ignore input entirely";
    assert!(crate::io::import::deserializers::artifacts::stl::v_ascii::any::deserialize_bytes(bytes).is_err());
    assert!(crate::io::import::deserializers::artifacts::gltf::v2_0::any::deserialize_bytes(bytes).is_err());
    assert!(crate::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(bytes).is_err());
    assert!(crate::io::import::deserializers::artifacts::las::v1_0::any::deserialize_bytes(bytes).is_err());
}

//#region 🕸️RealGeometryIo
use crate::io::import::deserializers::artifacts::{obj::v3_0::any as obj_import, ply::v1_0::any as ply_import, stl::v_ascii::any as stl_import};
use crate::io::export::serializers::artifacts::{obj::v3_0::any as obj_export, ply::v1_0::any as ply_export, stl::v_ascii::any as stl_export};
use semio_framework_3d::mesh::HalfedgeMesh;

fn mesh_of(object: &LowpolyObject) -> HalfedgeMesh {
    assert_eq!(object.mesh, Some(mesh_child_handle(&object.id, &object.mesh_content)), "handle must hash mesh_content");
    HalfedgeMesh::from_json(&object.mesh_content).expect("mesh_content parses")
}

fn total_faces(snapshot: &LowpolySnapshot) -> usize {
    snapshot.objects.iter().filter(|o| !o.mesh_content.is_empty()).map(|o| mesh_of(o).face_count()).sum()
}

fn strip_lines_starting_with(text: &[u8], prefix: &str) -> Vec<u8> {
    String::from_utf8(text.to_vec()).unwrap().lines().filter(|l| !l.trim_start().starts_with(prefix)).collect::<Vec<_>>().join("\n").into_bytes()
}

const MULTI_OBJECT_OBJ: &str = "# hand-written, Blender-style\n\
mtllib scene.mtl\n\
o Quad\n\
v 0 0 0\n\
v 1 0 0\n\
v 1 1 0\n\
v 0 1 0\n\
usemtl Grey\n\
s off\n\
f 1 2 3 4\n\
o Pentagon\n\
v 0 0 2\n\
v 1 0 2\n\
v 1.5 1 2\n\
v 0.5 1.8 2\n\
v -0.5 1 2\n\
vn 0 0 1\n\
f 5//1 6//1 7//1 8//1 9//1\n\
f -5//1 -3//1 -1//1\n";

#[semio_framework_async_macros::async_test]
async fn obj_import_reads_real_multi_object_geometry_with_ngons() {
    let snapshot = obj_import::deserialize_bytes(MULTI_OBJECT_OBJ.as_bytes()).expect("real obj import");
    assert_eq!(snapshot.objects.len(), 2);
    assert_eq!(snapshot.objects[0].id, "obj-1");
    assert_eq!(snapshot.objects[1].id, "obj-2");
    assert_eq!(snapshot.objects[0].name, "Quad");
    assert_eq!(snapshot.objects[1].name, "Pentagon");
    assert_eq!(snapshot.objects[0].transform, LowpolyTransform::default());
    let quad = mesh_of(&snapshot.objects[0]);
    assert_eq!((quad.vertex_count(), quad.face_count()), (4, 1));
    assert_eq!(quad.face_vertex_ids(semio_framework_3d::mesh::FaceId(0)).unwrap().len(), 4, "quad stays a quad");
    let pentagon = mesh_of(&snapshot.objects[1]);
    assert_eq!((pentagon.vertex_count(), pentagon.face_count()), (5, 2), "indices remapped per object");
    assert_eq!(pentagon.face_vertex_ids(semio_framework_3d::mesh::FaceId(0)).unwrap().len(), 5, "pentagon stays an n-gon");
}

#[semio_framework_async_macros::async_test]
async fn obj_import_falls_back_to_groups_then_single_object_and_rejects_empty() {
    let grouped = "v 0 0 0\nv 1 0 0\nv 0 1 0\nv 0 0 1\ng A\nf 1 2 3\ng B\nf 1 2 4\n";
    let snapshot = obj_import::deserialize_bytes(grouped.as_bytes()).expect("grouped obj");
    assert_eq!(snapshot.objects.iter().map(|o| o.name.as_str()).collect::<Vec<_>>(), vec!["A", "B"]);
    let plain = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";
    assert_eq!(obj_import::deserialize_bytes(plain.as_bytes()).expect("plain obj").objects.len(), 1);
    let err = obj_import::deserialize_bytes(b"v 0 0 0\nv 1 0 0\n").expect_err("no faces");
    assert!(err.to_string().contains("no polygon faces"), "{err}");
}

#[semio_framework_async_macros::async_test]
async fn obj_import_merges_more_than_64_parts_into_one_object() {
    let mut text = String::new();
    for i in 0..70 {
        let base = i * 3;
        text.push_str(&format!("o part{i}\nv {i} 0 0\nv {i} 1 0\nv {i} 0 1\nf {} {} {}\n", base + 1, base + 2, base + 3));
    }
    let snapshot = obj_import::deserialize_bytes(text.as_bytes()).expect("many parts");
    assert_eq!(snapshot.objects.len(), 1);
    assert_eq!(mesh_of(&snapshot.objects[0]).face_count(), 70);
}

fn ascii_cube_stl() -> String {
    let p = |x: u8, y: u8, z: u8| [x as f64, y as f64, z as f64];
    let quads = [
        [p(0, 0, 0), p(0, 1, 0), p(1, 1, 0), p(1, 0, 0)],
        [p(0, 0, 1), p(1, 0, 1), p(1, 1, 1), p(0, 1, 1)],
        [p(0, 0, 0), p(1, 0, 0), p(1, 0, 1), p(0, 0, 1)],
        [p(0, 1, 0), p(0, 1, 1), p(1, 1, 1), p(1, 1, 0)],
        [p(0, 0, 0), p(0, 0, 1), p(0, 1, 1), p(0, 1, 0)],
        [p(1, 0, 0), p(1, 1, 0), p(1, 1, 1), p(1, 0, 1)],
    ];
    let mut out = String::from("solid cube\n");
    for q in quads {
        for tri in [[q[0], q[1], q[2]], [q[0], q[2], q[3]]] {
            out.push_str("  facet normal 0 0 0\n    outer loop\n");
            for v in tri {
                out.push_str(&format!("      vertex {} {} {}\n", v[0], v[1], v[2]));
            }
            out.push_str("    endloop\n  endfacet\n");
        }
    }
    out.push_str("endsolid cube\n");
    out
}

#[semio_framework_async_macros::async_test]
async fn stl_ascii_cube_import_welds_and_round_trips() {
    let snapshot = stl_import::deserialize_bytes(ascii_cube_stl().as_bytes()).expect("ascii stl import");
    assert_eq!(snapshot.objects.len(), 1);
    assert_eq!(snapshot.objects[0].name, "cube");
    let mesh = mesh_of(&snapshot.objects[0]);
    assert_eq!((mesh.vertex_count(), mesh.face_count()), (8, 12));

    let exported = stl_export::serialize(&snapshot).expect("stl export");
    assert_eq!(exported.triangles.len(), 12);
    assert!(exported.triangles.iter().all(|t| ((t.normal[0].powi(2) + t.normal[1].powi(2) + t.normal[2].powi(2)).sqrt() - 1.0).abs() < 1e-9), "unit face normals");
    let recovered = stl_import::deserialize_bytes(&stl_export::serialize_bytes(&snapshot).expect("stl bytes")).expect("stl re-import");
    let mesh = mesh_of(&recovered.objects[0]);
    assert_eq!((mesh.vertex_count(), mesh.face_count()), (8, 12));

    let binary = semio_s_artifact_stdio_stl::engine::encode_stl_binary(&exported);
    let from_binary = stl_import::deserialize_bytes(&binary).expect("binary stl import");
    assert_eq!(mesh_of(&from_binary.objects[0]).face_count(), 12);
    assert!(stl_import::deserialize_bytes(b"solid empty\nendsolid empty\n").is_err());
}

#[semio_framework_async_macros::async_test]
async fn ply_ascii_import_reads_vertex_and_face_elements() {
    let text = "ply\nformat ascii 1.0\ncomment made by hand\nelement vertex 5\nproperty float x\nproperty float y\nproperty float z\nproperty uchar red\nelement face 2\nproperty list uchar int vertex_indices\nend_header\n0 0 0 255\n1 0 0 255\n1 1 0 255\n0 1 0 255\n0.5 0.5 1 255\n4 0 1 2 3\n3 0 1 4\n";
    let snapshot = ply_import::deserialize_bytes(text.as_bytes()).expect("ply import");
    assert_eq!(snapshot.objects.len(), 1);
    let mesh = mesh_of(&snapshot.objects[0]);
    assert_eq!((mesh.vertex_count(), mesh.face_count()), (5, 2));
    assert_eq!(mesh.face_vertex_ids(semio_framework_3d::mesh::FaceId(0)).unwrap().len(), 4);
}

#[semio_framework_async_macros::async_test]
async fn geometry_export_then_import_preserves_face_counts_and_applies_transform() {
    let mut snapshot = fixture();
    snapshot.objects[0].transform = LowpolyTransform { position: [10.0, 0.0, 0.0], rotation: [0.0, 0.0, 90.0], scale: [2.0, 2.0, 2.0] };
    let box_faces = HalfedgeMesh::from_json(&snapshot.objects[0].mesh_content).unwrap().face_count();
    assert_eq!(total_faces(&snapshot), box_faces);

    // OBJ: lossless via the DSL comment; geometry-only once the comment is stripped.
    let obj_bytes = obj_export::serialize_bytes(&snapshot).expect("obj export");
    assert_eq!(obj_import::deserialize_bytes(&obj_bytes).expect("lossless obj"), snapshot);
    let geometry_only = obj_import::deserialize_bytes(&strip_lines_starting_with(&obj_bytes, "# semio-lowpoly-dsl")).expect("geometry obj");
    assert_eq!(geometry_only.objects.len(), 1, "empty-mesh objects contribute no geometry");
    assert_eq!(geometry_only.objects[0].name, "First_Object");
    assert_eq!(total_faces(&geometry_only), box_faces);
    let world = mesh_of(&geometry_only.objects[0]);
    let xs: Vec<f32> = (0..world.vertex_count()).map(|v| world.vertex_position(semio_framework_3d::mesh::VertexId(v as u32)).unwrap().0[0]).collect();
    assert!(xs.iter().all(|x| (*x - 10.0).abs() <= 1.0 + 1e-4), "translated + scaled: {xs:?}");

    // PLY: lossless via the comment; geometry-only once the comment is stripped.
    let ply_bytes = ply_export::serialize_bytes(&snapshot).expect("ply export");
    assert_eq!(ply_import::deserialize_bytes(&ply_bytes).expect("lossless ply"), snapshot);
    let ply_geometry = ply_import::deserialize_bytes(&strip_lines_starting_with(&ply_bytes, "comment semio-lowpoly-dsl")).expect("geometry ply");
    assert_eq!(total_faces(&ply_geometry), box_faces);

    // STL: triangulated, so face count = fan triangles.
    let triangles: usize = (0..world.face_count()).map(|f| world.face_vertex_ids(semio_framework_3d::mesh::FaceId(f as u32)).unwrap().len() - 2).sum();
    let stl = stl_import::deserialize_bytes(&stl_export::serialize_bytes(&snapshot).expect("stl export")).expect("stl import");
    assert_eq!(total_faces(&stl), triangles);
}
//#endregion 🕸️RealGeometryIo
