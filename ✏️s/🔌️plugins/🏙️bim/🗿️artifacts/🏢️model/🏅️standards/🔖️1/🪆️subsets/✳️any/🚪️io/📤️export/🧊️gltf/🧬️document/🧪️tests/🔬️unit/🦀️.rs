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
fn the_snapshot_describes_buffer_views_and_accessors_that_tile_the_buffer() {
    let snapshot = triangle().to_snapshot();
    let (document, buffer) = (&snapshot.document, &snapshot.buffers[0]);
    assert_eq!(document.asset.version, "2.0");
    assert_eq!((document.scene, document.scenes[0].nodes.clone()), (Some(0), vec![0]));
    assert_eq!(buffer.len(), 36 + 36 + 12);
    assert_eq!(document.buffers[0].byte_length, buffer.len());
    assert_eq!(document.buffer_views.len(), 3);
    let mut end = 0;
    for view in &document.buffer_views {
        assert_eq!((view.byte_offset, view.byte_offset % 4), (end, 0));
        end += view.byte_length;
    }
    assert_eq!(end, buffer.len());
    let primitive = &document.meshes[0].primitives[0];
    let position = &document.accessors[primitive.attributes.iter().find(|(name, _)| name == "POSITION").expect("positions").1];
    assert_eq!((position.component_type, position.count, position.kind), (codec::ComponentType::Float, 3, codec::AccessorType::Vec3));
    assert_eq!((position.min.clone(), position.max.clone()), (Some(vec![0.0, 0.0, 0.0]), Some(vec![1.0, 2.0, 0.0])));
    assert_eq!(document.materials[0].alpha_mode, codec::AlphaMode::Blend);
    assert_eq!(document.nodes[1].extras, Some(codec::Json::Object(vec![("id".into(), codec::Json::String("a".into()))])));
    assert!(document.nodes[1].translation.is_none() && document.nodes[0].children == [1]);
}

#[test]
fn the_binary_buffer_holds_the_little_endian_floats_and_indices() {
    let buffer = triangle().to_snapshot().buffers.remove(0);
    assert_eq!(f32::from_le_bytes(buffer[12..16].try_into().unwrap()), 1.0);
    assert_eq!(f32::from_le_bytes(buffer[28..32].try_into().unwrap()), 2.0);
    assert_eq!(u32::from_le_bytes(buffer[72..76].try_into().unwrap()), 0);
    assert_eq!(u32::from_le_bytes(buffer[80..84].try_into().unwrap()), 2);
}

#[test]
fn the_gltf_crate_reads_back_the_nodes_the_triangle_and_the_material() {
    let bytes = triangle().to_glb().expect("encodes");
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
    let snapshot = empty.to_snapshot();
    assert!(snapshot.buffers.is_empty() && snapshot.document.buffers.is_empty());
    let decoded = codec::decode(&empty.to_glb().expect("encodes")).expect("the stdio codec reads its own container");
    assert_eq!((decoded.document.nodes.len(), decoded.document.scenes.len(), decoded.buffers.len()), (0, 1, 0));
}

#[test]
fn primitive_bounds_are_the_componentwise_extremes() {
    let primitive = GltfPrimitive { positions: vec![1.0, -2.0, 3.0, -1.0, 5.0, 0.0], ..GltfPrimitive::default() };
    assert_eq!(primitive.bounds(), Some(([-1.0, -2.0, 0.0], [1.0, 5.0, 3.0])));
    assert_eq!(GltfPrimitive::default().bounds(), None);
}
