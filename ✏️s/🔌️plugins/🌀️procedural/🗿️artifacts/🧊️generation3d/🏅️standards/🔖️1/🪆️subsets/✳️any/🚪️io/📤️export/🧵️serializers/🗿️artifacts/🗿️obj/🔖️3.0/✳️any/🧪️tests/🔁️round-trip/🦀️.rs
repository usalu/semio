//! 🗿️ obj round trip — the committed unit cube out as Wavefront OBJ and back.
//!
//! Import keeps editable polygon data in the existing mesh constructor.

use crate::{assert_is_unit_cube, assert_oracle_agrees_on_unit_cube, project, assert_imported_polygon_is_unit_cube, unit_cube_semio_mesh};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::export::serializers::artifacts::obj::v3_0::any as export;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts::obj::v3_0::any as import;

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
fn obj_import_plants_editable_polygon_geometry_without_losing_the_cube() {
    let document = import::deserialize_bytes(&exported()).expect("obj imports into an editable document");
    assert_imported_polygon_is_unit_cube("obj editable import", document);
}

#[test]
fn obj_import_refuses_bytes_that_are_not_utf8_instead_of_returning_an_empty_document() {
    let error = import::deserialize_bytes(&[0xff, 0xfe, 0x00, 0x01]).expect_err("non-utf8 must not import");
    assert!(error.to_string().contains("generation3d\u{2190}obj"), "the error names the direction and format, got {error}");
}
