use crate::io::export::serializers::artifacts::{gltf::v2_0::any as gltf_out, las::v1_0::any as las_out, obj::v3_0::any as obj_out, ply::v1_0::any as ply_out, stl::v_ascii::any as stl_out, txt::v_utf_8::any as txt_out};
use crate::io::import::deserializers::artifacts::txt::v_utf_8::any as txt_in;
use crate::GisTerrainSnapshot;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn terrain() -> GisTerrainSnapshot {
    GisTerrainSnapshot { exaggeration: 1.5, ..GisTerrainSnapshot::default() }
}

/// 🔮️ The third-party `gltf` reader (test-only) loads the export and reads its surface back.
#[test]
fn gltf_export_is_read_by_the_gltf_crate() {
    let bytes = gltf_out::serialize_bytes(&terrain()).expect("gltf export");
    let (document, buffers, _) = gltf::import_slice(&bytes).expect("the gltf crate imports the export");
    let primitive = document.meshes().next().expect("one mesh").primitives().next().expect("one primitive");
    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
    let positions: Vec<[f32; 3]> = reader.read_positions().expect("positions").collect();
    let indices: Vec<u32> = reader.read_indices().expect("indices").into_u32().collect();
    assert_eq!((positions.len(), indices.len()), (4, 6));
    assert!(positions.contains(&[1.0, 1.0, 0.0]));
}

#[test]
fn mesh_formats_carry_the_two_surface_triangles() {
    let stl = String::from_utf8(stl_out::serialize_bytes(&terrain()).expect("stl")).expect("ascii stl");
    assert_eq!(stl.matches("facet normal").count(), 2);
    let obj = String::from_utf8(obj_out::serialize_bytes(&terrain()).expect("obj")).expect("obj text");
    assert_eq!(obj.lines().filter(|line| line.starts_with("f ")).count(), 2);
    let ply = ply_out::serialize_bytes(&terrain()).expect("ply");
    assert!(ply.starts_with(b"ply\n"));
    let las = semio_s_artifact_stdio_las::io::decode_las(&las_out::serialize_bytes(&terrain()).expect("las")).expect("decodes as las");
    assert!(!las.points.is_empty());
}

#[test]
fn txt_is_the_exact_dsl_carrier() {
    let bytes = txt_out::serialize_bytes(&terrain()).expect("txt export");
    assert_eq!(txt_in::deserialize_bytes(&bytes).expect("txt import"), terrain());
}
