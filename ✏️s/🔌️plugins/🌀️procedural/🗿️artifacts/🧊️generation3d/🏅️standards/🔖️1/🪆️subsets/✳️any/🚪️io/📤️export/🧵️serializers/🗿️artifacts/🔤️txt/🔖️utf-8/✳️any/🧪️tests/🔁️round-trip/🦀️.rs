//! 🔤️ txt round trip — the only FULL-FIDELITY lane in this artifact's IO surface.
//!
//! Every mesh format keeps just the evaluated geometry; txt keeps the document. So this case
//! asserts what none of the others can: a real document — flow graph, synapses, layout AND
//! generation history — goes out as text and comes back equal in VALUE, not merely recognizable.
//! Both halves were `Err("not yet implemented")` before this ticket.
//!
//! The document under test is deliberately an IMPORTED one rather than the default fixture: it has
//! a note carrying a long base64 payload, a neuron with a real `neuron_kind`, two synapses and three
//! layout entries, so a grammar that dropped any of those would show up here.

use crate::retire_document;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::export::serializers::artifacts::txt::v_utf_8::any as export;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any as import;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::mesh_bridge;

fn document() -> semio_s_artifact_procedural_generation3d::Generation3dSnapshot {
    mesh_bridge::import_document("brep.io.importStl", mesh_bridge::base64_encode(&crate::stl_bytes_for_txt_case()))
}

#[test]
fn txt_export_writes_this_artifacts_own_text_grammar() {
    let source = document();
    let bytes = export::serialize_bytes(&source).expect("a document exports as txt");
    retire_document(source);
    let text = String::from_utf8(bytes).expect("the txt export is utf-8");
    assert!(!text.is_empty(), "the export is not empty");
    assert!(text.contains("brep.io.importStl"), "the graph's neuron kind is in the text");
    assert!(text.contains("layout"), "the layout the import fixture pinned is in the text");
}

#[test]
fn txt_round_trip_returns_the_whole_document_not_just_its_geometry() {
    let before = document();
    let bytes = export::serialize_bytes(&before).expect("a document exports as txt");
    let after = import::deserialize_bytes(&bytes).expect("our own txt bytes re-import");
    assert_eq!(after, before, "txt is full fidelity: the flow graph and the generation history both come back");
    retire_document(after);
    retire_document(before);
}

#[test]
fn txt_import_refuses_text_that_is_not_this_grammar_instead_of_returning_an_empty_document() {
    let error = import::deserialize_bytes(b"hello, this is just prose").expect_err("arbitrary prose must not import");
    assert!(!error.to_string().is_empty(), "the parse failure carries a message");
}
