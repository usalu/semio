//! 🧊️ gltf round trip — the committed unit cube out as glTF 2.0 JSON and back.
//!
//! glTF is the only lane whose geometry lives in a BINARY buffer rather than in the text, so the
//! byte-level assertion is about the container: valid JSON, an `asset.version` of `"2.0"`, and a
//! buffer that actually carries a `data:` uri — a `.gltf` whose buffer never got embedded parses
//! fine and renders nothing, which is precisely the failure the old placeholder could not show.
//!
//! Like ply, import normalizes onto `brep.io.importStl` (the evaluator has no glTF operator).

use crate::{assert_is_unit_cube, assert_oracle_agrees_on_unit_cube, project, retire_document, unit_cube_semio_mesh};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::export::serializers::artifacts::gltf::v2_0::any as export;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts::gltf::v2_0::any as import;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts::stl::v_ascii::any as stl_import;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::mesh_bridge;

fn exported() -> Vec<u8> {
    export::serialize_mesh_bytes(&unit_cube_semio_mesh()).expect("the unit cube exports as gltf")
}

#[test]
fn gltf_export_writes_a_real_gltf_2_0_document_not_the_artifact_dsl() {
    let bytes = exported();
    let text = std::str::from_utf8(&bytes).expect("gltf json is utf-8");
    let document: serde_json::Value = serde_json::from_str(text).expect("gltf export is valid json");
    assert_eq!(document["asset"]["version"].as_str(), Some("2.0"), "the asset declares glTF 2.0");
    assert_eq!(document["meshes"].as_array().map(Vec::len), Some(1), "one mesh, the merged preview");
    let uri = document["buffers"][0]["uri"].as_str().expect("the geometry buffer is embedded");
    assert!(uri.starts_with("data:application/octet-stream;base64,"), "a .gltf has no binary chunk, so the buffer must be a data uri");
    assert!(!text.contains("semio procedural"), "the pre-ticket bug emitted this artifact's own DSL text under a .gltf name");
}

#[test]
fn gltf_round_trip_preserves_the_unit_cube_including_its_shared_vertex_pool() {
    let bytes = exported();
    let back = import::mesh_from_bytes(&bytes).expect("our own gltf bytes re-import");
    let projection = project(&back);
    assert_eq!(projection.vertex_count, 8, "gltf keeps the indexed vertex pool");
    assert_is_unit_cube("gltf", &projection);
    assert_oracle_agrees_on_unit_cube("gltf", &back);
}

#[test]
fn gltf_import_normalizes_onto_the_stl_import_neuron_without_losing_the_cube() {
    let bytes = exported();
    let document = import::deserialize_bytes(&bytes).expect("gltf imports into a document");
    let (kind, payload) = mesh_bridge::imported_source(&document).expect("the import fixture plants a source note and an import neuron");
    assert_eq!(kind, import::IMPORT_NEURON_KIND, "gltf has no flow operator of its own, so it normalizes to stl");
    let planted = mesh_bridge::base64_decode(payload).expect("the note holds base64");
    let recovered = stl_import::mesh_from_bytes(&planted).expect("the planted payload is real stl");
    assert_is_unit_cube("gltf→stl", &project(&recovered));
    retire_document(document);
}

#[test]
fn gltf_import_refuses_bytes_that_are_not_gltf_instead_of_returning_an_empty_document() {
    let error = import::deserialize_bytes(b"{\"not\": \"a gltf document\"}").expect_err("a json document that is not gltf must not import");
    assert!(error.to_string().contains("generation3d\u{2190}gltf"), "the error names the direction and format, got {error}");
}
