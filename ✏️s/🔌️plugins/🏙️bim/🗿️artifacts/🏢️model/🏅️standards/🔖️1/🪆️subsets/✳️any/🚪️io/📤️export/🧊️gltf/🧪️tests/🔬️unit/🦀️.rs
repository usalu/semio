use super::*;
use crate::standards::v1::subsets::any::io::binary::snapshot as pack;
use crate::standards::v1::subsets::any::io::export::gltf::testkit::{house, read, third_party, HOUSE_DIR};
use crate::standards::v1::subsets::any::io::io;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::compute_element_solids;
use semio_framework::io::io_mechanism::IoEntryDirection;

#[test]
fn the_declaration_lists_the_gltf_export_between_the_bim_and_the_gltf_dialect() {
    let declaration = io();
    let entry = declaration.entries.iter().find(|entry| entry.into == GLTF_DIALECT).expect("the glTF entry");
    assert_eq!((entry.from, entry.direction), (crate::BIM_MODEL_DIALECT, IoEntryDirection::Export));
    assert_eq!(GLTF_DIALECT.artifact_kind, "s.stdio.gltf");
}

#[test]
fn the_export_entry_turns_a_packed_model_into_the_glb_bytes() {
    let model = house();
    let declaration = io();
    let entry = declaration.entries.iter().find(|entry| entry.into == GLTF_DIALECT).expect("the glTF entry");
    let produced = (entry.run)(&IoPayload::Binary(pack::encode(&model))).expect("the entry runs");
    assert_eq!(produced.value, IoPayload::Binary(export_glb(&model).0));
    assert!(produced.diagnostics.is_empty());
}

#[test]
fn the_gltf_crate_reads_the_house_node_tree_meshes_and_accessors() {
    let model = house();
    let (bytes, notes) = export_glb(&model);
    assert!(notes.is_empty());
    let (document, blob) = third_party(&bytes);
    let solids = compute_element_solids(&model);
    let (gltf, _) = model_to_gltf(&model);
    assert_eq!(document.nodes().len(), gltf.nodes.len());
    assert_eq!(document.nodes().filter(|node| node.mesh().is_some()).count(), solids.len());
    assert_eq!(document.scenes().len(), 1);
    assert_eq!(document.default_scene().expect("a default scene").nodes().count(), model.sites.len());
    let mut triangles = 0;
    for mesh in document.meshes() {
        for primitive in mesh.primitives() {
            assert_eq!(primitive.mode(), gltf::mesh::Mode::Triangles);
            let reader = primitive.reader(|buffer| (buffer.index() == 0).then_some(blob.as_slice()));
            let positions: Vec<[f32; 3]> = reader.read_positions().expect("positions").collect();
            let indices: Vec<u32> = reader.read_indices().expect("indices").into_u32().collect();
            assert_eq!(reader.read_normals().expect("normals").count(), positions.len());
            assert!(indices.iter().all(|index| (*index as usize) < positions.len()));
            let bounds = primitive.bounding_box();
            assert!(positions.iter().all(|p| (0..3).all(|axis| p[axis] >= bounds.min[axis] && p[axis] <= bounds.max[axis])));
            triangles += indices.len() / 3;
        }
    }
    assert_eq!(triangles, solids.values().map(|solid| solid.triangle_count()).sum::<usize>());
    assert_eq!(document.materials().len(), gltf.materials.len());
}

#[test]
fn the_gltf_crate_composes_the_same_world_position_for_a_known_vertex() {
    let model = house();
    let (bytes, _) = export_glb(&model);
    let (document, blob) = third_party(&bytes);
    let solids = compute_element_solids(&model);
    let node = document.nodes().find(|node| node.extras().as_ref().is_some_and(|extras| serde_json::from_str::<serde_json::Value>(extras.get()).is_ok_and(|value| value["id"] == "w-south"))).expect("the south wall");
    let storey = document.nodes().find(|candidate| candidate.children().any(|child| child.index() == node.index())).expect("its storey");
    let building = document.nodes().find(|candidate| candidate.children().any(|child| child.index() == storey.index())).expect("its building");
    let site = document.nodes().find(|candidate| candidate.children().any(|child| child.index() == building.index())).expect("its site");
    let primitive = node.mesh().unwrap().primitives().next().unwrap();
    let reader = primitive.reader(|buffer| (buffer.index() == 0).then_some(blob.as_slice()));
    let first = reader.read_positions().unwrap().next().unwrap();
    let mut world = [f64::from(first[0]), f64::from(first[1]), f64::from(first[2])];
    for chain in [&node, &storey, &building, &site] {
        let matrix = chain.transform().matrix();
        let at = [world[0], world[1], world[2]];
        world = std::array::from_fn(|row| (0..3).map(|column| f64::from(matrix[column][row]) * at[column]).sum::<f64>() + f64::from(matrix[3][row]));
    }
    let solid = &solids["w-south"];
    let (sin, cos) = solid.placement.rotation.sin_cos();
    let layer_zero = solid.indices.iter().map(|corner| &solid.positions[*corner as usize * 3..*corner as usize * 3 + 3]).map(|p| scene::y_up(solid.placement.x + cos * p[0] - sin * p[1], solid.placement.y + sin * p[0] + cos * p[1], solid.placement.z + p[2])).collect::<Vec<_>>();
    assert!(layer_zero.iter().any(|p| (0..3).all(|axis| (p[axis] - world[axis]).abs() < 1e-3)), "{world:?} is not a vertex of the solid placed by the model");
}

#[test]
fn the_committed_house_file_is_the_current_export() {
    let (bytes, _) = export_glb(&house());
    if std::env::var("BIM_BLESS").is_ok() {
        std::fs::create_dir_all(HOUSE_DIR).expect("the fixture directory");
        std::fs::write(format!("{HOUSE_DIR}/🏠️house.glb"), &bytes).expect("the file is written");
    }
    assert_eq!(read("🏠️house.glb"), bytes, "the committed export drifted: rewrite it with BIM_BLESS=1");
}

#[test]
fn the_exported_house_is_a_valid_binary_container_with_a_json_chunk_the_spec_accepts() {
    let (bytes, _) = export_glb(&house());
    let (json, buffer) = container::split_glb(&bytes).expect("the container splits");
    let value: serde_json::Value = serde_json::from_str(json.trim_end()).expect("valid JSON");
    assert_eq!(value["asset"]["version"], "2.0");
    assert_eq!(value["buffers"][0]["byteLength"].as_u64(), Some(buffer.len() as u64));
    assert!(value["nodes"].as_array().unwrap().iter().all(|node| node["name"].as_str().is_some_and(|name| !name.is_empty())));
}
