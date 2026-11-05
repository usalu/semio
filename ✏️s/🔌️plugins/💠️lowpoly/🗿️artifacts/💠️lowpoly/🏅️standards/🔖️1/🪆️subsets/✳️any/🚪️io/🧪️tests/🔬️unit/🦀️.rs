use semio_framework_3d::mesh::{MeshModelingOps};
use semio_s_artifact_lowpoly_lowpoly_model::{snapshot::mesh_child_handle, snapshot::LowpolyObject, snapshot::LowpolyPaintLayer, snapshot::LowpolySnapshot, snapshot::LowpolyTransform, snapshot::LOWPOLY_DOCUMENT_SCHEMA};

/// 🧪️ Two objects with a canonical alternating-color paint canvas exercise snapshot field fidelity.
fn box_mesh_json() -> String {
    HalfedgeMesh::box_prim(1.0, 1.0, 1.0).expect("box prim").to_json().expect("mesh json")
}

fn fixture() -> LowpolySnapshot {
    let mesh_json = box_mesh_json();
    let obj1 = LowpolyObject { mesh_state:None,
        id: "obj-1".into(),
        name: "First Object".into(),
        transform: LowpolyTransform { position: [1.0, 2.0, 3.0], rotation: [0.0, 90.0, 0.0], scale: [1.0, 1.0, 1.0] },
        smooth_shading: false,
        mesh: Some(mesh_child_handle("obj-1", &mesh_json)),
        paint_layers: vec![LowpolyPaintLayer { name: "Base".into(), visible: true, opacity: 0.5, blend_mode: "normal".into(), pixels: [10, 20, 30, 40, 50, 60, 70, 80].repeat(semio_s_artifact_lowpoly_lowpoly_model::snapshot::LOWPOLY_PAINT_TEXTURE_SIZE.pow(2) / 2) }],
        mesh_content: mesh_json,
    };
    let obj2 = LowpolyObject { mesh_state:None, id: "obj-2".into(), name: "Second Object".into(), transform: LowpolyTransform::default(), smooth_shading: true, mesh: None, paint_layers: Vec::new(), mesh_content: String::new() };
    LowpolySnapshot { schema: LOWPOLY_DOCUMENT_SCHEMA.into(), objects: vec![obj1, obj2] }
}

#[semio_framework_async_macros::async_test]
async fn txt_export_import_round_trips_the_snapshot() {
    let snapshot = fixture();
    let bytes = crate::export::serializers::artifacts::txt::v_utf_8::any::serialize_bytes(&snapshot).expect("txt export");
    let recovered = crate::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes).expect("txt import");
    assert_eq!(snapshot, recovered);
}

#[semio_framework_async_macros::async_test]
async fn json_export_import_round_trips_the_snapshot() {
    let snapshot = fixture();
    let bytes = crate::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).expect("json export");
    let recovered = crate::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).expect("json import");
    assert_eq!(snapshot, recovered);
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn geometry_of(snapshot: &LowpolySnapshot) -> (usize, usize) {
    snapshot.objects.iter().filter(|o| !o.mesh_content.is_empty()).map(|o| mesh_of(o)).fold((0, 0), |(v, f), mesh| (v + mesh.vertex_count(), f + mesh.face_count()))
}

#[semio_framework_async_macros::async_test]
async fn mesh_formats_carry_the_geometry_and_nothing_else() {
    let snapshot = fixture();
    let (_, box_faces) = geometry_of(&snapshot);
    let obj = crate::export::serializers::artifacts::obj::v3_0::any::serialize_bytes(&snapshot).expect("obj export");
    assert!(!String::from_utf8(obj.clone()).expect("obj text").contains("semio"), "no private side channel in the file");
    assert_eq!(geometry_of(&crate::import::deserializers::artifacts::obj::v3_0::any::deserialize_bytes(&obj).expect("obj import")), (8, box_faces));
    let ply = crate::export::serializers::artifacts::ply::v1_0::any::serialize_bytes(&snapshot).expect("ply export");
    assert_eq!(geometry_of(&crate::import::deserializers::artifacts::ply::v1_0::any::deserialize_bytes(&ply).expect("ply import")), (8, box_faces));
    let gltf = crate::export::serializers::artifacts::gltf::v2_0::any::serialize_bytes(&snapshot).expect("gltf export");
    assert_eq!(geometry_of(&crate::import::deserializers::artifacts::gltf::v2_0::any::deserialize_bytes(&gltf).expect("gltf import")).0, 8);
    let dwg = crate::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).expect("dwg export");
    assert_eq!(geometry_of(&crate::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(&dwg).expect("dwg import")).0, 8);
    let las = semio_s_artifact_stdio_las::standards::v1_0::subsets::any::io::decode_las(&crate::export::serializers::artifacts::las::v1_0::any::serialize_bytes(&snapshot).expect("las export")).expect("las decodes");
    assert_eq!(las.points.len(), 8);
}

#[semio_framework_async_macros::async_test]
async fn png_is_a_picture_of_the_model() {
    let bytes = crate::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&fixture()).expect("png export");
    let png = semio_framework_pixels::decode_png(&bytes).expect("decodes as png");
    assert_eq!((png.width, png.height), (512, 512));
    assert!(png.pixels.chunks(4).filter(|px| px[0] < 250).count() > 10_000, "the box is painted");
}

/// 🔮️ The third-party `gltf` reader (test-only) loads the GLB export.
#[semio_framework_async_macros::async_test]
async fn gltf_export_is_read_by_the_gltf_crate() {
    let bytes = crate::export::serializers::artifacts::gltf::v2_0::any::serialize_bytes(&fixture()).expect("gltf export");
    let (document, buffers, _) = gltf::import_slice(&bytes).expect("the gltf crate imports the GLB");
    let primitive = document.meshes().next().expect("one mesh").primitives().next().expect("one primitive");
    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
    assert_eq!(reader.read_positions().expect("positions").count(), 8);
    assert_eq!(reader.read_indices().expect("indices").into_u32().count(), 36);
}

#[semio_framework_async_macros::async_test]
async fn geometry_format_garbage_bytes_still_error_loudly() {
    let bytes = b"not a real payload";
    assert!(crate::import::deserializers::artifacts::stl::v_ascii::any::deserialize_bytes(bytes).is_err());
    assert!(crate::import::deserializers::artifacts::gltf::v2_0::any::deserialize_bytes(bytes).is_err());
    assert!(crate::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(bytes).is_err());
}

//#region 🕸️RealGeometryIo
use crate::{import::deserializers::artifacts::obj::v3_0::any as obj_import, import::deserializers::artifacts::ply::v1_0::any as ply_import, import::deserializers::artifacts::stl::v_ascii::any as stl_import};
use crate::{export::serializers::artifacts::obj::v3_0::any as obj_export, export::serializers::artifacts::ply::v1_0::any as ply_export, export::serializers::artifacts::stl::v_ascii::any as stl_export};
use semio_framework_3d::mesh::HalfedgeMesh;

fn mesh_of(object: &LowpolyObject) -> HalfedgeMesh {
    match &object.mesh_state {
        Some(state) => {
            assert_eq!(object.mesh, Some(semio_s_artifact_lowpoly_lowpoly_model::snapshot::managed_mesh_child_handle(&object.id, state)), "handle must hash managed state");
            state.clone().into_mesh().expect("managed mesh")
        }
        None => {
            assert_eq!(object.mesh, Some(mesh_child_handle(&object.id, &object.mesh_content)), "handle must hash authored source");
            HalfedgeMesh::from_json(&object.mesh_content).expect("mesh_content parses")
        }
    }
}

fn total_faces(snapshot: &LowpolySnapshot) -> usize {
    snapshot.objects.iter().filter(|o| !o.mesh_content.is_empty()).map(|o| mesh_of(o).face_count()).sum()
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

    let obj_bytes = obj_export::serialize_bytes(&snapshot).expect("obj export");
    let geometry_only = obj_import::deserialize_bytes(&obj_bytes).expect("geometry obj");
    assert_eq!(geometry_only.objects.len(), 1, "empty-mesh objects contribute no geometry");
    assert_eq!(geometry_only.objects[0].name, "First_Object");
    assert_eq!(total_faces(&geometry_only), box_faces);
    let world = mesh_of(&geometry_only.objects[0]);
    let xs: Vec<f32> = (0..world.vertex_count()).map(|v| world.vertex_position(semio_framework_3d::mesh::VertexId(v as u32)).unwrap().0[0]).collect();
    assert!(xs.iter().all(|x| (*x - 10.0).abs() <= 1.0 + 1e-4), "translated + scaled: {xs:?}");

    let ply_geometry = ply_import::deserialize_bytes(&ply_export::serialize_bytes(&snapshot).expect("ply export")).expect("geometry ply");
    assert_eq!(total_faces(&ply_geometry), box_faces);

    let stl_geometry = stl_import::deserialize_bytes(&stl_export::serialize_bytes(&snapshot).expect("stl export")).expect("geometry stl");
    let triangles: usize = (0..world.face_count()).map(|f| world.face_vertex_ids(semio_framework_3d::mesh::FaceId(f as u32)).unwrap().len() - 2).sum();
    assert_eq!(total_faces(&stl_geometry), triangles);
}
//#endregion 🕸️RealGeometryIo


#[semio_framework_async_macros::async_test]
async fn managed_geometry_exports_current_state_independently_of_authored_source() {
    use semio_s_artifact_lowpoly_lowpoly_model::{managed_mesh::LowpolyMeshState, snapshot::managed_mesh_child_handle};
    let data: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/managed-export.json")).expect("neutral fixture");
    let source_positions: Vec<[f32; 3]> = serde_json::from_value(data["sourcePositions"].clone()).unwrap();
    let positions: Vec<[f32; 3]> = serde_json::from_value(data["positions"].clone()).unwrap();
    let faces: Vec<Vec<u32>> = serde_json::from_value(data["faces"].clone()).unwrap();
    let source = HalfedgeMesh::from_faces(&source_positions, &faces).unwrap().to_json().unwrap();
    let state = LowpolyMeshState::from_mesh(HalfedgeMesh::from_faces(&positions, &faces).unwrap());
    let mut snapshot = fixture();
    snapshot.objects.truncate(1);
    snapshot.objects[0].mesh = Some(managed_mesh_child_handle(&snapshot.objects[0].id, &state));
    snapshot.objects[0].mesh_state = Some(state);
    snapshot.objects[0].transform = LowpolyTransform {
        position: serde_json::from_value(data["transform"]["position"].clone()).unwrap(),
        rotation: serde_json::from_value(data["transform"]["rotation"].clone()).unwrap(),
        scale: serde_json::from_value(data["transform"]["scale"].clone()).unwrap(),
    };
    for (scale, output) in [(&data["transform"]["scale"], &data), (&data["reflected"]["scale"], &data["reflected"])] {
        snapshot.objects[0].transform.scale = serde_json::from_value(scale.clone()).unwrap();
        let expected: Vec<[f32; 3]> = serde_json::from_value(output["expectedPositions"].clone()).unwrap();
        let indices: Vec<u32> = serde_json::from_value(output["expectedIndices"].clone()).unwrap();
        for source in [source.as_str(), "", "independent authored source"] {
            snapshot.objects[0].mesh_content = source.into();
            let before = snapshot.clone();
            let parts = crate::mesh_geometry::world_parts("gltf", &snapshot).expect("managed geometry");
            assert_eq!(parts.len(), 1);
            assert_eq!(parts[0].positions.len(), expected.len());
            for (actual, expected) in parts[0].positions.iter().zip(&expected) {
                assert!(actual.iter().zip(expected).all(|(actual, expected)| (actual - f64::from(*expected)).abs() < 1e-5));
            }
            assert_eq!(parts[0].faces, vec![indices.clone()]);
            let bytes = crate::export::serializers::artifacts::gltf::v2_0::any::serialize_bytes(&snapshot).expect("managed GLB");
            let (document, buffers, _) = gltf::import_slice(&bytes).expect("independent GLB reader");
            let primitive = document.meshes().next().unwrap().primitives().next().unwrap();
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let oracle_positions = reader.read_positions().unwrap().collect::<Vec<_>>();
            assert_eq!(oracle_positions.len(), expected.len());
            for (actual, expected) in oracle_positions.iter().zip(&expected) {
                assert!(actual.iter().zip(expected).all(|(actual, expected)| (actual - expected).abs() < 1e-5));
            }
            assert_eq!(reader.read_indices().unwrap().into_u32().collect::<Vec<_>>(), indices);
            assert_eq!(snapshot, before, "export preserves paint, literal source and managed owners");
        }
    }
    snapshot.objects[0].transform.scale = serde_json::from_value(data["tinyReflectedScale"].clone()).unwrap();
    assert_eq!(crate::mesh_geometry::world_parts("obj", &snapshot).unwrap()[0].faces, vec![vec![2, 1, 0]], "tiny binary32 scales retain reflection sign");
    let empty = LowpolyMeshState::empty();
    snapshot.objects[0].mesh = Some(managed_mesh_child_handle(&snapshot.objects[0].id, &empty));
    snapshot.objects[0].mesh_state = Some(empty);
    snapshot.objects[0].mesh_content = source;
    let parts = crate::mesh_geometry::world_parts("obj", &snapshot).expect("present empty managed mesh");
    assert!(parts.iter().all(|part| part.positions.is_empty() && part.faces.is_empty()));
}


#[semio_framework_async_macros::async_test]
async fn gltf_import_preserves_selected_scene_hierarchy_and_mesh_instances() {
    use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::{schema::snapshot::{GltfNode, GltfScene}, io::export::encode_glb};
    let data: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/gltf-scene.json")).unwrap();
    let positions: Vec<[f32; 3]> = serde_json::from_value(data["positions"].clone()).unwrap();
    let faces: Vec<Vec<u32>> = serde_json::from_value(data["faces"].clone()).unwrap();
    let source = crate::mesh_geometry::snapshot_from_parts("fixture", vec![crate::mesh_geometry::PolygonPart { name: "Triangle".into(), positions, faces }]).unwrap();
    let mut glb = crate::export::serializers::artifacts::gltf::v2_0::any::serialize(&source).unwrap();
    glb.document.nodes = vec![
        GltfNode { children: vec![1, 2, 3], translation: Some(serde_json::from_value(data["rootTranslation"].clone()).unwrap()), ..Default::default() },
        GltfNode { mesh: Some(0), name: Some("TRS".into()), translation: Some(serde_json::from_value(data["childTranslation"].clone()).unwrap()), rotation: Some(serde_json::from_value(data["childRotation"].clone()).unwrap()), scale: Some(serde_json::from_value(data["childScale"].clone()).unwrap()), ..Default::default() },
        GltfNode { mesh: Some(0), name: Some("Matrix".into()), matrix: Some(serde_json::from_value(data["matrix"].clone()).unwrap()), ..Default::default() },
        GltfNode { mesh: Some(0), name: Some("Reflected".into()), scale: Some(serde_json::from_value(data["reflectedScale"].clone()).unwrap()), ..Default::default() },
        GltfNode { mesh: Some(0), translation: Some([1000.0; 3]), ..Default::default() },
    ];
    glb.document.scenes[0].nodes = vec![0];
    let mut other_scene: GltfScene = glb.document.scenes[0].clone();
    other_scene.nodes = vec![4];
    glb.document.scenes.push(other_scene);
    let bytes = encode_glb(&glb).unwrap();
    let (oracle, buffers, _) = gltf::import_slice(&bytes).expect("independent valid GLB");
    let recovered = crate::import::deserializers::artifacts::gltf::v2_0::any::deserialize_bytes(&bytes).unwrap();
    let expected: Vec<Vec<[f32; 3]>> = serde_json::from_value(data["expectedPositions"].clone()).unwrap();
    assert_eq!(recovered.objects.iter().map(|object| object.name.as_str()).collect::<Vec<_>>(), vec!["TRS", "Matrix", "Reflected"]);
    let expected_faces: Vec<Vec<u32>> = serde_json::from_value(data["expectedFaces"].clone()).unwrap();
    let root = oracle.default_scene().unwrap().nodes().next().unwrap();
    for (index, node) in root.children().enumerate() {
        let primitive = node.mesh().unwrap().primitives().next().unwrap();
        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
        let transform = |point: [f32; 3], matrix: [[f32; 4]; 4]| -> [f32; 3] {
            std::array::from_fn(|row| matrix[0][row] * point[0] + matrix[1][row] * point[1] + matrix[2][row] * point[2] + matrix[3][row])
        };
        let oracle_positions: Vec<_> = reader.read_positions().unwrap().map(|point| transform(transform(point, node.transform().matrix()), root.transform().matrix())).collect();
        let mesh = mesh_of(&recovered.objects[index]);
        let mut oracle_face: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();
        if node.transform().decomposed().2.into_iter().chain(root.transform().decomposed().2).product::<f32>() < 0.0 { oracle_face.reverse(); }
        assert_eq!(oracle_face, expected_faces[index]);
        assert_eq!(mesh.face_vertex_ids(semio_framework_3d::mesh::FaceId(0)).unwrap().into_iter().map(|vertex| vertex.0).collect::<Vec<_>>(), expected_faces[index]);
        for (vertex, point) in expected[index].iter().enumerate() {
            let actual = mesh.vertex_position(semio_framework_3d::mesh::VertexId(vertex as u32)).unwrap().0;
            assert!(actual.iter().zip(point).all(|(actual, expected)| (actual - expected).abs() < 1e-5));
            assert!(oracle_positions[vertex].iter().zip(point).all(|(actual, expected)| (actual - expected).abs() < 1e-5));
        }
    }
    for mode in data["unsupportedModes"].as_array().unwrap() {
        glb.document.meshes[0].primitives[0].mode = mode.as_u64();
        let error = crate::import::deserializers::artifacts::gltf::v2_0::any::deserialize(&glb).expect_err("unsupported topology");
        assert!(error.to_string().contains("primitive mode"), "{error}");
    }
    glb.document.meshes[0].primitives[0].mode = Some(4);
    glb.document.nodes[1].children = vec![0];
    assert!(crate::import::deserializers::artifacts::gltf::v2_0::any::deserialize(&glb).is_err(), "cycles are refused");
    glb.document.scene = None;
    glb.document.scenes.clear();
    glb.document.nodes.truncate(2);
    glb.document.nodes[0].children = vec![1];
    let error = crate::import::deserializers::artifacts::gltf::v2_0::any::deserialize(&glb).expect_err("scene-less cycle");
    assert!(error.to_string().contains("no polygon faces") || error.to_string().contains("cycle"), "{error}");
}


#[semio_framework_async_macros::async_test]
async fn gltf_import_admits_complete_node_forests_before_scene_extraction() {
    use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::{schema::snapshot::GltfNode, io::export::encode_glb};
    let data: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/gltf-forest.json")).unwrap();
    let positions: Vec<[f32; 3]> = serde_json::from_value(data["positions"].clone()).unwrap();
    let faces: Vec<Vec<u32>> = serde_json::from_value(data["faces"].clone()).unwrap();
    let geometry = crate::mesh_geometry::snapshot_from_parts("fixture", vec![crate::mesh_geometry::PolygonPart { name: "Triangle Library".into(), positions, faces }]).unwrap();
    let source = crate::export::serializers::artifacts::gltf::v2_0::any::serialize(&geometry).unwrap();
    for case in data["cases"].as_array().unwrap() {
        let mut snapshot = source.clone();
        let mesh_count = case["meshCount"].as_u64().or_else(|| data["meshCount"].as_u64()).unwrap() as usize;
        snapshot.document.meshes = (0..mesh_count).map(|_| source.document.meshes[0].clone()).collect();
        snapshot.document.nodes = case["nodes"].as_array().unwrap().iter().map(|node| GltfNode {
            mesh: node["mesh"].as_u64().map(|index| index as usize),
            translation: node.get("translation").map(|translation| serde_json::from_value(translation.clone()).unwrap()),
            children: node.get("children").map(|children| serde_json::from_value(children.clone()).unwrap()).unwrap_or_default(),
            ..Default::default()
        }).collect();
        snapshot.document.scenes = case["scenes"].as_array().unwrap().iter().map(|roots| {
            let mut scene = source.document.scenes[0].clone();
            scene.nodes = serde_json::from_value(roots.clone()).unwrap();
            scene
        }).collect();
        snapshot.document.scene = case["scene"].as_u64().map(|index| index as usize);
        let imported = crate::import::deserializers::artifacts::gltf::v2_0::any::deserialize(&snapshot);
        if case["valid"].as_bool().unwrap() {
            let imported = imported.unwrap_or_else(|error| panic!("{}: {error}", case["id"]));
            assert_eq!(imported.objects.len(), case["objects"].as_u64().unwrap() as usize, "{}", case["id"]);
            let (oracle, buffers, _) = gltf::import_slice(&encode_glb(&snapshot).unwrap()).expect("independent valid node forest or entity library");
            assert_eq!(oracle.meshes().count(), mesh_count);
            let primitive = oracle.meshes().next().unwrap().primitives().next().unwrap();
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let expected = reader.read_positions().unwrap().collect::<Vec<_>>();
            let object_positions: Option<Vec<Vec<[f32; 3]>>> = case.get("expectedObjectPositions").map(|value| serde_json::from_value(value.clone()).unwrap());
            for (index, object) in imported.objects.iter().enumerate() {
                let mesh = mesh_of(object);
                let expected = object_positions.as_ref().map_or(&expected, |positions| &positions[index]);
                assert_eq!(mesh.vertex_count(), expected.len());
                for (vertex, expected) in expected.iter().enumerate() {
                    assert_eq!(mesh.vertex_position(semio_framework_3d::mesh::VertexId(vertex as u32)).unwrap().0, *expected);
                }
            }
            if let Some(positions) = &object_positions {
                let matrix = oracle.nodes().next().unwrap().transform().matrix();
                let transformed: Vec<[f32; 3]> = expected.iter().map(|point| std::array::from_fn(|row| matrix[0][row]*point[0] + matrix[1][row]*point[1] + matrix[2][row]*point[2] + matrix[3][row])).collect();
                assert_eq!(transformed, positions[0]);
                assert_eq!(expected, positions[1]);
            }
            assert_eq!(oracle.nodes().count(), snapshot.document.nodes.len());
            for (node, authored) in oracle.nodes().zip(&snapshot.document.nodes) {
                assert_eq!(node.children().map(|child| child.index()).collect::<Vec<_>>(), authored.children);
                assert_eq!(node.mesh().map(|mesh| mesh.index()), authored.mesh);
            }
        } else {
            let error = imported.expect_err(case["id"].as_str().unwrap());
            assert!(error.to_string().contains(case["error"].as_str().unwrap()), "{}: {error}", case["id"]);
            if case["externalReject"].as_bool().unwrap_or(false) {
                assert!(gltf::import_slice(&encode_glb(&snapshot).unwrap()).is_err(), "independent invalid index: {}", case["id"]);
            }
        }
    }
}
