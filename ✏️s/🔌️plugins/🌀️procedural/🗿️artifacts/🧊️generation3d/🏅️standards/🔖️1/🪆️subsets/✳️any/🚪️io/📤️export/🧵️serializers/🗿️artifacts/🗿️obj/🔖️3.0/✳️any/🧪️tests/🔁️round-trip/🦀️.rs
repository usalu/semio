//! 🗿️ obj round trip — the committed unit cube out as Wavefront OBJ and back.
//!
//! OBJ is the one mesh lane whose flow operator takes PLAIN TEXT, so the import fixture's note is
//! asserted to be readable OBJ source rather than base64 — that is a real user-visible property of
//! the imported graph, not an implementation detail.

use crate::{assert_is_unit_cube, assert_oracle_agrees_on_unit_cube, project, retire_document, unit_cube_semio_mesh};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::export::serializers::artifacts::obj::v3_0::any as export;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts::obj::v3_0::any as import;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::mesh_bridge;

fn exported() -> Vec<u8> {
    export::serialize_mesh_bytes(&unit_cube_semio_mesh()).expect("the unit cube exports as obj")
}

#[test]
fn obj_export_writes_real_wavefront_obj_not_the_artifact_dsl() {
    let bytes = exported();
    let text = std::str::from_utf8(&bytes).expect("obj is utf-8");
    assert_eq!(text.lines().filter(|line| line.starts_with("v ")).count(), 36, "one `v` per triangle corner (this codec does not deduplicate)");
    assert_eq!(text.lines().filter(|line| line.starts_with("f ")).count(), 12, "the cube's 12 faces are all written");
    assert!(text.lines().any(|line| line.starts_with("o ")), "one `o` block per mesh, so a re-import recovers the mesh boundary");
    assert!(!text.contains("semio"), "the pre-ticket bug emitted this artifact's own DSL text under an .obj name");
}

#[test]
fn obj_round_trip_preserves_the_unit_cube() {
    let bytes = exported();
    let back = import::mesh_from_bytes(&bytes).expect("our own obj bytes re-import");
    assert_is_unit_cube("obj", &project(&back));
    assert_oracle_agrees_on_unit_cube("obj", &back);
}

#[test]
fn obj_import_plants_a_previewable_import_neuron_carrying_readable_source_text() {
    let bytes = exported();
    let document = import::deserialize_bytes(&bytes).expect("obj imports into a document");
    let (kind, payload) = mesh_bridge::imported_source(&document).expect("the import fixture plants a source note and an import neuron");
    assert_eq!(kind, import::IMPORT_NEURON_KIND, "obj re-enters the graph through its own brep operator");
    assert!(payload.lines().filter(|line| line.starts_with("f ")).count() == 12, "the note holds plain obj text, not base64");
    retire_document(document);
}

#[test]
fn obj_import_refuses_bytes_that_are_not_utf8_instead_of_returning_an_empty_document() {
    let error = import::deserialize_bytes(&[0xff, 0xfe, 0x00, 0x01]).expect_err("non-utf8 must not import");
    assert!(error.to_string().contains("generation3d\u{2190}obj"), "the error names the direction and format, got {error}");
}
