use super::*;
use crate::standards::v1::subsets::any::io::binary::snapshot as pack;
use crate::standards::v1::subsets::any::io::export::gltf::testkit::{house, read, third_party, HOUSE_DIR};
use crate::standards::v1::subsets::any::io::io;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::compute_element_solids;
use semio_framework_os_kernel::io::io_mechanism::IoEntryDirection;

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
    assert_eq!(produced.value, IoPayload::Binary(export_glb(&model).expect("exports").0));
    assert!(produced.diagnostics.is_empty());
}

#[test]
fn the_gltf_crate_reads_the_house_node_tree_meshes_and_accessors() {
    let model = house();
    let (bytes, notes) = export_glb(&model).expect("exports");
    assert!(notes.is_empty());
    let (document, blob) = third_party(&bytes);
    let solids = compute_element_solids(&model);
    let (gltf, _) = model_to_gltf(&model).expect("the model infers");
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
    let (bytes, _) = export_glb(&model).expect("exports");
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
    let (bytes, _) = export_glb(&house()).expect("exports");
    if std::env::var("BIM_BLESS").is_ok() {
        std::fs::create_dir_all(HOUSE_DIR).expect("the fixture directory");
        std::fs::write(format!("{HOUSE_DIR}/🏠️house.glb"), &bytes).expect("the file is written");
    }
    assert_eq!(read("🏠️house.glb"), bytes, "the committed export drifted: rewrite it with BIM_BLESS=1");
}

#[test]
fn the_exported_house_is_a_valid_binary_container_with_a_json_chunk_the_spec_accepts() {
    let (bytes, _) = export_glb(&house()).expect("exports");
    let decoded = codec::decode(&bytes).expect("the stdio codec decodes the container");
    assert_eq!(decoded.document.asset.version, "2.0");
    assert_eq!(decoded.document.buffers[0].byte_length, decoded.buffers[0].len());
    assert!(decoded.document.nodes.iter().all(|node| node.name.as_deref().is_some_and(|name| !name.is_empty())));
    assert_eq!(decoded.document, model_to_gltf(&house()).expect("the house infers").0.to_snapshot().document, "the container carries exactly the document that was built");
}

#[test]
fn the_committed_components_file_is_the_current_export() {
    use crate::standards::v1::subsets::any::io::export::gltf::testkit::{components, read_components, COMPONENTS_DIR};
    let (bytes, notes) = export_glb(&components()).expect("exports");
    assert!(notes.is_empty(), "{notes:?}");
    if std::env::var("BIM_BLESS").is_ok() {
        std::fs::create_dir_all(COMPONENTS_DIR).expect("the fixture directory");
        std::fs::write(format!("{COMPONENTS_DIR}/🪑️components.glb"), &bytes).expect("the file is written");
    }
    assert_eq!(read_components("🪑️components.glb"), bytes, "the committed export drifted: rewrite it with BIM_BLESS=1");
}

#[test]
fn the_components_and_runs_are_element_nodes_with_the_colour_of_their_service() {
    use crate::standards::v1::subsets::any::io::export::gltf::testkit::components;
    let (gltf, _) = model_to_gltf(&components()).expect("infers");
    let report = projection::project(&gltf);
    assert_eq!((report.kinds["component"], report.kinds["mep"]), (12, 7));
    let names: Vec<&str> = gltf.materials.iter().map(|material| material.name.as_str()).collect();
    for system in ["supply", "return", "domestic-water", "waste", "gas", "power", "lighting"] {
        assert!(names.contains(&format!("MEP {system}").as_str()), "{names:?}");
    }
    let supply = gltf.materials.iter().find(|material| material.name == "MEP supply").expect("a supply material");
    assert_eq!(supply.color.map(|channel| (channel * 255.0).round() as u8), [0x1f, 0x77, 0xd4, 255]);
    let volume = report.volumes["mep-supply"];
    assert!((volume - 0.1 * 5.3).abs() < 1e-5, "the mitred prisms of the duct add up to section times length: {volume}");
}

#[test]
fn the_components_subject_report_equals_the_table_the_three_oracle_measured_from_the_committed_file() {
    use crate::standards::v1::subsets::any::io::export::gltf::testkit::{components, read_components};
    let oracle: serde_json::Value = serde_json::from_slice(&read_components("🔬️measure/🔣️.json")).expect("the oracle table");
    let ours: serde_json::Value = serde_json::from_str(&projection::project(&model_to_gltf(&components()).expect("infers").0).to_json()).expect("the report parses");
    for key in ["nodes", "meshes", "primitives", "triangles", "materials", "kinds", "storeys"] {
        assert_eq!(oracle[key], ours[key], "{key}");
    }
    for (id, volume) in oracle["volumes"].as_object().expect("volumes") {
        let found = ours["volumes"][id].as_f64().unwrap_or_else(|| panic!("{id}: no volume"));
        assert!((volume.as_f64().unwrap() - found).abs() <= 1e-9 * found.abs().max(1.0), "{id}: three {volume}, written {found}");
    }
}
