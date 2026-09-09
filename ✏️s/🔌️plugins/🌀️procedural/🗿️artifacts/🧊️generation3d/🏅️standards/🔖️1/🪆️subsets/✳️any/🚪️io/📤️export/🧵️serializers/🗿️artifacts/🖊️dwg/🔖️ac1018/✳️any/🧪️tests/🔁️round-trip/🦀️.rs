//! 🖊️ dwg round trip — the committed unit cube out as a real DWG polyface mesh and back.
//!
//! This is the lane the plugin root's registered mesh-import host-media handler
//! (`register_mesh_dwg_import_handler`, `✏️s/🔌️plugins/🌀️procedural/🦀️.rs`) actually rides on, so a
//! silent failure here is a blank canvas after a drag-and-drop rather than a bad file on disk.
//!
//! DWG is a container format with a version-stamped binary header, so the byte-level assertion is
//! the `AC1` magic; everything semantic is asserted through the decoded drawing.

use crate::{assert_is_unit_cube, assert_oracle_agrees_on_unit_cube, project, retire_document, unit_cube_semio_mesh};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::export::serializers::artifacts::dwg::v_ac1018::any as export;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts::dwg::v_ac1018::any as import;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::mesh_bridge;

fn exported() -> Vec<u8> {
    export::serialize_mesh_bytes(&unit_cube_semio_mesh()).expect("the unit cube exports as dwg")
}

#[test]
fn dwg_export_writes_a_real_dwg_container_not_the_artifact_dsl() {
    let bytes = exported();
    assert!(bytes.starts_with(b"AC1"), "a dwg file opens with an `AC1<nnn>` version stamp, got {:?}", &bytes[..bytes.len().min(8)]);
    assert!(!String::from_utf8_lossy(&bytes).contains("semio procedural"), "the pre-ticket bug emitted this artifact's own DSL text under a .dwg name");
}

#[test]
fn dwg_export_writes_a_mesh_entity_so_the_round_trip_preserves_the_unit_cube() {
    let bytes = exported();
    let back = import::mesh_from_bytes(&bytes).expect("our own dwg bytes re-import");
    assert_is_unit_cube("dwg", &project(&back));
    assert_oracle_agrees_on_unit_cube("dwg", &back);
}

#[test]
fn dwg_import_plants_a_previewable_import_neuron_carrying_the_source_bytes() {
    let bytes = exported();
    let document = import::deserialize_bytes(&bytes).expect("dwg imports into a document");
    let (kind, payload) = mesh_bridge::imported_source(&document).expect("the import fixture plants a source note and an import neuron");
    assert_eq!(kind, import::IMPORT_NEURON_KIND, "dwg re-enters the graph through its own brep operator");
    assert_eq!(mesh_bridge::base64_decode(payload).expect("the note holds base64"), bytes, "the note carries the native dwg bytes verbatim");
    retire_document(document);
}

#[test]
fn dwg_import_refuses_bytes_that_are_not_dwg_instead_of_returning_an_empty_document() {
    let error = import::deserialize_bytes(b"not a dwg file at all, not even close").expect_err("garbage must not import");
    assert!(error.to_string().contains("generation3d\u{2190}dwg"), "the error names the direction and format, got {error}");
}
