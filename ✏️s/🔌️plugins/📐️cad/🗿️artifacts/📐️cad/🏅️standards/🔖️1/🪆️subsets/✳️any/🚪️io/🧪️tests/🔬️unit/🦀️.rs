
use super::*;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::Brep;

//#region 🔖️SemioMeshBridge
#[semio_framework_async_macros::async_test]
async fn export_solids_as_obj_uses_real_stdio_mesh_codec_not_hand_rolled_bytes() {
    let mut kernel = Brep::new();
    let solid = kernel.box_prim(1.0, 1.0, 1.0).expect("box");
    let export = export_solids_as(&mut kernel, std::slice::from_ref(&solid), CAD_SOLID_EXPORT_DIALECT_OBJ, "box").expect("obj export");
    let DslValue::String(text) = export.data else { panic!("expected text data") };
    let vertex_lines = text.lines().filter(|l| l.starts_with("v ")).count();
    let face_lines = text.lines().filter(|l| l.starts_with("f ")).count();
    assert!(vertex_lines >= 8, "expected real OBJ vertices, got {vertex_lines} in {text:?}");
    assert!(face_lines >= 12, "expected real OBJ faces, got {face_lines}");
    assert_eq!(export.mime_type, cad_solid_export_mime_type(CAD_SOLID_EXPORT_DIALECT_OBJ).unwrap());
    assert!(export.encoding.is_none());
}

#[semio_framework_async_macros::async_test]
async fn export_solids_as_stl_uses_real_stdio_mesh_codec() {
    let mut kernel = Brep::new();
    let solid = kernel.box_prim(1.0, 1.0, 1.0).expect("box");
    let export = export_solids_as(&mut kernel, std::slice::from_ref(&solid), CAD_SOLID_EXPORT_DIALECT_STL, "box").expect("stl export");
    let DslValue::String(encoded) = export.data else { panic!("expected base64 text data") };
    assert_eq!(export.encoding.as_deref(), Some("base64"));
    let bytes = base64_codec::base64_standard_decode(&encoded).expect("valid base64");
    assert!(bytes.len() > 84, "expected a real binary STL body, got {} bytes", bytes.len());
    let triangle_count = u32::from_le_bytes(bytes[80..84].try_into().unwrap());
    assert!(triangle_count >= 12, "expected a real box's 12+ triangles, got {triangle_count}");
    assert_eq!(bytes.len(), 84 + (triangle_count as usize) * 50);
}

#[semio_framework_async_macros::async_test]
async fn export_solids_as_obj_none_for_a_solid_that_fails_to_tessellate() {
    let mut kernel = Brep::new();
    let solid = kernel.box_prim(1.0, 1.0, 1.0).expect("box");
    let _ = kernel.dispose(&solid);
    assert!(export_solids_as(&mut kernel, std::slice::from_ref(&solid), CAD_SOLID_EXPORT_DIALECT_OBJ, "gone").is_none());
}
//#endregion 🔖️SemioMeshBridge

//#region 🔖️SemioBrepBridge
#[semio_framework_async_macros::async_test]
async fn export_solids_as_step_round_trips_through_real_semio_brep_bridge() {
    let mut kernel = Brep::new();
    let solid = kernel.box_prim(2.0, 3.0, 4.0).expect("box");
    let original_volume = kernel.volume(&solid).expect("volume");
    assert!((original_volume - 24.0).abs() < 1e-6, "box volume sanity: {original_volume}");

    let kernel_text = kernel.export_step(std::slice::from_ref(&solid)).expect("kernel step export");
    let original_brep = semio_brep_snapshot_from_step_text(&kernel_text).expect("semio/brep from kernel step");

    let export = export_solids_as(&mut kernel, std::slice::from_ref(&solid), CAD_SOLID_EXPORT_DIALECT_STEP, "box").expect("step export");
    let DslValue::String(step_text) = export.data else { panic!("expected text data") };
    assert!(step_text.starts_with("ISO-10303-21;"), "real Part-21 header expected, got {step_text:?}");
    assert!(step_text.contains("MANIFOLD_SOLID_BREP") || step_text.contains("ADVANCED_BREP_SHAPE_REPRESENTATION"), "expected real AP214 brep entities");

    let reimported_brep = semio_brep_snapshot_from_step_text(&step_text).expect("reimport via semio/brep bridge");
    assert_eq!(reimported_brep.solids.len(), original_brep.solids.len(), "solid count geometry-equivalence");
    assert_eq!(reimported_brep.faces.len(), original_brep.faces.len(), "face count geometry-equivalence");
    assert_eq!(reimported_brep.vertices.len(), original_brep.vertices.len(), "vertex count geometry-equivalence");

    fn vertex_bounds(points: impl Iterator<Item = [f64; 3]>) -> ([f64; 3], [f64; 3]) {
        let mut min = [f64::INFINITY; 3];
        let mut max = [f64::NEG_INFINITY; 3];
        for p in points {
            for axis in 0..3 {
                min[axis] = min[axis].min(p[axis]);
                max[axis] = max[axis].max(p[axis]);
            }
        }
        (min, max)
    }
    let (brep_min, brep_max) = vertex_bounds(reimported_brep.vertices.iter().map(|v| [v.point.x, v.point.y, v.point.z]));
    for axis in 0..3 {
        assert!(brep_max[axis] > brep_min[axis], "reimported brep must carry real spatial extent on axis {axis}, got min {:?} max {:?}", brep_min, brep_max);
    }

    let mesh_snapshot = semio_mesh_snapshot_from_solids(&mut kernel, std::slice::from_ref(&solid), 0.1).expect("tessellate the same solid the reimported brep describes into a real semio/mesh snapshot");
    let mesh_positions: Vec<[f64; 3]> = mesh_snapshot.meshes.iter().flat_map(|m| m.primitives.iter()).flat_map(|p| p.positions.iter()).map(|p| [p.x, p.y, p.z]).collect();
    assert!(!mesh_positions.is_empty(), "expected real tessellated mesh positions, not an empty semio/mesh snapshot");
    let (mesh_min, mesh_max) = vertex_bounds(mesh_positions.iter().copied());
    for axis in 0..3 {
        assert!((mesh_min[axis] - brep_min[axis]).abs() < 1e-6, "semio/mesh vs reimported semio/brep bounding-box MIN mismatch on axis {axis}: mesh {} vs brep {}", mesh_min[axis], brep_min[axis]);
        assert!((mesh_max[axis] - brep_max[axis]).abs() < 1e-6, "semio/mesh vs reimported semio/brep bounding-box MAX mismatch on axis {axis}: mesh {} vs brep {}", mesh_max[axis], brep_max[axis]);
    }

    let gltf = semio_framework_plugin::resolve_ready(SemioMeshToGltf::serialize(&mesh_snapshot)).expect("real semio/mesh -> gltf codec must succeed on a real tessellated box");
    assert_eq!(gltf.document.meshes.len(), 1, "expected exactly one gltf mesh for one solid");
    assert_eq!(gltf.buffers.len(), 1, "expected one packed geometry buffer");
    let position_accessor = gltf.document.accessors.first().expect("POSITION accessor must exist");
    assert_eq!(position_accessor.count, mesh_positions.len(), "gltf POSITION accessor count must match the semio/mesh vertex count");
    let buffer_view = &gltf.document.buffer_views[position_accessor.buffer_view.expect("POSITION accessor must reference a bufferView")];
    let raw = &gltf.buffers[0][buffer_view.byte_offset..buffer_view.byte_offset + buffer_view.byte_length];
    let decoded_positions: Vec<[f64; 3]> =
        raw.chunks_exact(12).map(|triple| [f32::from_le_bytes(triple[0..4].try_into().unwrap()) as f64, f32::from_le_bytes(triple[4..8].try_into().unwrap()) as f64, f32::from_le_bytes(triple[8..12].try_into().unwrap()) as f64]).collect();
    assert_eq!(decoded_positions.len(), mesh_positions.len(), "decoded gltf buffer must carry exactly the semio/mesh vertex count");
    let (gltf_min, gltf_max) = vertex_bounds(decoded_positions.into_iter());
    for axis in 0..3 {
        assert!((gltf_min[axis] - brep_min[axis]).abs() < 1e-4, "final .gltf bytes vs reimported semio/brep bounding-box MIN mismatch on axis {axis}: gltf {} vs brep {}", gltf_min[axis], brep_min[axis]);
        assert!((gltf_max[axis] - brep_max[axis]).abs() < 1e-4, "final .gltf bytes vs reimported semio/brep bounding-box MAX mismatch on axis {axis}: gltf {} vs brep {}", gltf_max[axis], brep_max[axis]);
    }
}

#[semio_framework_async_macros::async_test]
async fn semio_brep_snapshot_from_step_text_carries_real_topology() {
    let mut kernel = Brep::new();
    let solid = kernel.box_prim(1.0, 1.0, 1.0).expect("box");
    let step_text = kernel.export_step(std::slice::from_ref(&solid)).expect("kernel step export");
    let brep = semio_brep_snapshot_from_step_text(&step_text).expect("semio/brep from step");
    assert!(!brep.solids.is_empty(), "expected at least one real BrepSolid");
    assert!(!brep.faces.is_empty(), "expected real BrepFaces, not an empty shell");
    assert!(!brep.vertices.is_empty(), "expected real BrepVertexes");
    let round_tripped = step_text_from_semio_brep_snapshot(&brep).expect("semio/brep to step");
    assert!(round_tripped.starts_with("ISO-10303-21;"));
}

#[semio_framework_async_macros::async_test]
async fn repair_step_trailing_comma_before_close_paren_is_quote_aware() {
    assert_eq!(repair_step_trailing_comma_before_close_paren("(#1,)"), "(#1)");
    assert_eq!(repair_step_trailing_comma_before_close_paren("(#1, #2,)"), "(#1, #2)");
    assert_eq!(repair_step_trailing_comma_before_close_paren("()"), "()");
    assert_eq!(repair_step_trailing_comma_before_close_paren("('weird,)name', #1)"), "('weird,)name', #1)");
}
//#endregion 🔖️SemioBrepBridge
