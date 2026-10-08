use super::*;
use crate::standards::v1::subsets::any::io::export::gltf::testkit::third_party;

fn triangle() -> GltfModel {
    let primitive = GltfPrimitive { material: 0, positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 2.0, 0.0], normals: vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], indices: vec![0, 1, 2] };
    GltfModel {
        name: "scene".into(),
        extras: DslValue::Null,
        materials: vec![GltfMaterial { name: "glass".into(), color: [0.5, 0.25, 0.75, 0.5], metallic: 0.0, roughness: 0.05, blend: true }],
        meshes: vec![GltfMesh { name: "mesh".into(), primitives: vec![primitive] }],
        nodes: vec![
            GltfNode { name: "root".into(), translation: Some([1.0, 2.0, 3.0]), rotation: Some([0.0, 0.0, 0.0, 1.0]), mesh: None, children: vec![1], extras: DslValue::Null },
            GltfNode { name: "leaf".into(), translation: None, rotation: None, mesh: Some(0), children: Vec::new(), extras: DslValue::object([("id".to_string(), DslValue::String("a".into()))]) },
        ],
        roots: vec![0],
    }
}

#[test]
fn the_json_describes_buffer_views_and_accessors_that_tile_the_buffer() {
    let (json, buffer) = triangle().to_json_and_buffer();
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");
    assert_eq!(value["asset"]["version"], "2.0");
    assert_eq!((value["scene"].as_u64(), value["scenes"][0]["nodes"][0].as_u64()), (Some(0), Some(0)));
    assert_eq!(buffer.len(), 36 + 36 + 12);
    assert_eq!(value["buffers"][0]["byteLength"], buffer.len());
    let views = value["bufferViews"].as_array().expect("views");
    assert_eq!(views.len(), 3);
    let mut end = 0;
    for view in views {
        assert_eq!(view["byteOffset"].as_u64(), Some(end));
        assert_eq!(view["byteOffset"].as_u64().unwrap() % 4, 0);
        end += view["byteLength"].as_u64().unwrap();
    }
    assert_eq!(end as usize, buffer.len());
    let position = &value["accessors"][value["meshes"][0]["primitives"][0]["attributes"]["POSITION"].as_u64().unwrap() as usize];
    assert_eq!((position["componentType"].as_u64(), position["count"].as_u64(), position["type"].as_str()), (Some(5126), Some(3), Some("VEC3")));
    assert_eq!((position["min"].clone(), position["max"].clone()), (serde_json::json!([0.0, 0.0, 0.0]), serde_json::json!([1.0, 2.0, 0.0])));
    assert_eq!(value["materials"][0]["alphaMode"], "BLEND");
    assert_eq!(value["nodes"][1]["extras"]["id"], "a");
    assert!(value["nodes"][1].get("translation").is_none() && value["nodes"][0]["children"][0] == 1);
}

#[test]
fn the_binary_buffer_holds_the_little_endian_floats_and_indices() {
    let (_, buffer) = triangle().to_json_and_buffer();
    assert_eq!(f32::from_le_bytes(buffer[12..16].try_into().unwrap()), 1.0);
    assert_eq!(f32::from_le_bytes(buffer[28..32].try_into().unwrap()), 2.0);
    assert_eq!(u32::from_le_bytes(buffer[72..76].try_into().unwrap()), 0);
    assert_eq!(u32::from_le_bytes(buffer[80..84].try_into().unwrap()), 2);
}

#[test]
fn the_gltf_crate_reads_back_the_nodes_the_triangle_and_the_material() {
    let bytes = triangle().to_glb();
    let (document, blob) = third_party(&bytes);
    assert_eq!((document.nodes().len(), document.meshes().len(), document.materials().len()), (2, 1, 1));
    let mesh = document.meshes().next().unwrap();
    let primitive = mesh.primitives().next().unwrap();
    let reader = primitive.reader(|buffer| (buffer.index() == 0).then_some(blob.as_slice()));
    assert_eq!(reader.read_positions().unwrap().collect::<Vec<_>>(), [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 2.0, 0.0]]);
    assert_eq!(reader.read_indices().unwrap().into_u32().collect::<Vec<_>>(), [0, 1, 2]);
    let material = document.materials().next().unwrap();
    assert_eq!(material.pbr_metallic_roughness().base_color_factor(), [0.5, 0.25, 0.75, 0.5]);
    assert_eq!(material.alpha_mode(), gltf::material::AlphaMode::Blend);
    assert_eq!(document.nodes().next().unwrap().transform().decomposed().0, [1.0, 2.0, 3.0]);
}

#[test]
fn a_model_without_meshes_writes_no_buffer() {
    let empty = GltfModel { name: "n".into(), extras: DslValue::Null, materials: Vec::new(), meshes: Vec::new(), nodes: Vec::new(), roots: Vec::new() };
    let (json, buffer) = empty.to_json_and_buffer();
    assert!(buffer.is_empty() && !json.contains("buffers"));
    let (document, _) = third_party(&empty.to_glb());
    assert_eq!(document.nodes().len(), 0);
}

#[test]
fn primitive_bounds_are_the_componentwise_extremes() {
    let primitive = GltfPrimitive { positions: vec![1.0, -2.0, 3.0, -1.0, 5.0, 0.0], ..GltfPrimitive::default() };
    assert_eq!(primitive.bounds(), Some(([-1.0, -2.0, 0.0], [1.0, 5.0, 3.0])));
    assert_eq!(GltfPrimitive::default().bounds(), None);
}
