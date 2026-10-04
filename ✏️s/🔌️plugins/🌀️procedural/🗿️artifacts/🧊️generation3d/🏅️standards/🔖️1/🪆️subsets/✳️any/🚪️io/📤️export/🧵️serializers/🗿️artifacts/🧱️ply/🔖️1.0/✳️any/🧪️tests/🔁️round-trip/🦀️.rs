//! 🧱️ ply round trip — the committed unit cube out as ASCII PLY and back.
//!
//! PLY is one of the two lanes with a genuinely SHARED vertex pool, so this case additionally
//! asserts the cube comes back with 8 vertices rather than 36 — the property STL and OBJ
//! legitimately cannot keep, and therefore the property that proves the index buffer really
//! survived rather than being rebuilt from a soup.
//!
//! Import preserves the indexed surface through the existing editable mesh constructor.

use crate::{assert_is_unit_cube, assert_oracle_agrees_on_unit_cube, project, assert_imported_polygon_is_unit_cube, unit_cube_semio_mesh};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::export::serializers::artifacts::ply::v1_0::any as export;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts::ply::v1_0::any as import;

fn exported() -> Vec<u8> {
    export::serialize_mesh_bytes(&unit_cube_semio_mesh()).expect("the unit cube exports as ply")
}

#[test]
fn ply_export_writes_a_real_ply_header_not_the_artifact_dsl() {
    let bytes = exported();
    let text = String::from_utf8_lossy(&bytes).into_owned();
    assert!(text.starts_with("ply"), "a ply file opens with the `ply` magic, got {:?}", &text[..text.len().min(60)]);
    assert!(text.contains("format ascii 1.0"), "this codec writes the ascii form");
    assert!(text.contains("element vertex 8"), "the cube's shared 8-vertex pool is declared");
    assert!(text.contains("element face 12"), "the cube's 12 faces are declared");
    assert!(!text.contains("semio"), "the pre-ticket bug emitted this artifact's own DSL text under a .ply name");
}

#[test]
fn ply_round_trip_preserves_the_unit_cube_including_its_shared_vertex_pool() {
    let bytes = exported();
    let back = import::mesh_from_bytes(&bytes).expect("our own ply bytes re-import");
    let projection = project(&back);
    assert_eq!(projection.vertex_count, 8, "ply keeps a real shared index pool, so no corner is duplicated");
    assert_is_unit_cube("ply", &projection);
    assert_oracle_agrees_on_unit_cube("ply", &back);
}

#[test]
fn ply_import_plants_editable_polygon_geometry_without_losing_the_cube() {
    let document = import::deserialize_bytes(&exported()).expect("ply imports into an editable document");
    assert_imported_polygon_is_unit_cube("ply editable import", document);
}

#[test]
fn ply_import_refuses_bytes_that_are_not_ply_instead_of_returning_an_empty_document() {
    let error = import::deserialize_bytes(b"not a ply file").expect_err("garbage must not import");
    assert!(error.to_string().contains("generation3d\u{2190}ply"), "the error names the direction and format, got {error}");
}
