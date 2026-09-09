//! 🔺️ stl round trip — the committed unit cube out as ASCII STL and back.
//!
//! STL is the reference lane: it is the format the other mesh imports normalize THROUGH, and the
//! one whose export was byte-for-byte the artifact's own DSL text before this ticket. The bytes are
//! asserted to be STL by grammar (`solid` header, 12 `facet normal` records), not only by whether
//! our own reader accepts them — a reader and a writer that agree on the same wrong grammar would
//! otherwise pass.

use crate::{assert_is_unit_cube, assert_oracle_agrees_on_unit_cube, project, retire_document, unit_cube_semio_mesh};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::export::serializers::artifacts::stl::v_ascii::any as export;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts::stl::v_ascii::any as import;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::mesh_bridge;

fn exported() -> Vec<u8> {
    export::serialize_mesh_bytes(&unit_cube_semio_mesh()).expect("the unit cube exports as stl")
}

#[test]
fn stl_export_writes_real_ascii_stl_not_the_artifact_dsl() {
    let bytes = exported();
    let text = std::str::from_utf8(&bytes).expect("ascii stl is utf-8");
    assert!(text.starts_with("solid"), "ascii stl opens with `solid`, got {:?}", &text[..text.len().min(60)]);
    assert!(text.contains("endsolid"), "ascii stl closes with `endsolid`");
    assert_eq!(text.matches("facet normal").count(), 12, "the cube's 12 facets are all written");
    assert_eq!(text.matches("vertex ").count(), 36, "every facet writes its own 3 corners (stl shares no vertex pool)");
    assert!(!text.contains("semio"), "the pre-ticket bug emitted this artifact's own DSL text under an .stl name");
}

#[test]
fn stl_round_trip_preserves_the_unit_cube() {
    let bytes = exported();
    let back = import::mesh_from_bytes(&bytes).expect("our own stl bytes re-import");
    assert_is_unit_cube("stl", &project(&back));
    assert_oracle_agrees_on_unit_cube("stl", &back);
}

#[test]
fn stl_import_plants_a_previewable_import_neuron_carrying_the_source_bytes() {
    let bytes = exported();
    let document = import::deserialize_bytes(&bytes).expect("stl imports into a document");
    let (kind, payload) = mesh_bridge::imported_source(&document).expect("the import fixture plants a source note and an import neuron");
    assert_eq!(kind, import::IMPORT_NEURON_KIND, "stl re-enters the graph through its own brep operator");
    assert_eq!(mesh_bridge::base64_decode(payload).expect("the note holds base64"), bytes, "the note carries the source bytes verbatim");
    let previews = document.fixture.widgets.iter().any(|widget| matches!(widget, semio_framework_artifact_flow_flow::Widget::Neuron { preview: true, .. }));
    assert!(previews, "the import neuron previews, so the imported mesh reaches the 3d window");
    assert_eq!(document.fixture.synapses.len(), 2, "note -> import -> preview");
    retire_document(document);
}

#[test]
fn stl_import_refuses_bytes_that_are_not_stl_instead_of_returning_an_empty_document() {
    let error = import::deserialize_bytes(b"this is not an stl file at all").expect_err("garbage must not import");
    assert!(error.to_string().contains("generation3d\u{2190}stl"), "the error names the direction and format, got {error}");
}
