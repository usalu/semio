//! ⚖️ CAD reference owners retire under quoted grants: the keyed reference index passes the typed-command input lane and retires its keys and rows exactly.

#[global_allocator]
static HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[path = "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧪️test/⚖️typed-input-law/🦀️.rs"]
mod typed_input_law;

use semio_s_artifact_cad_cad_model::{snapshot::CadReference, CadReferenceIndex};
use typed_input_law::typed_input_retirement_law;

fn reference(id: &str) -> CadReference {
    CadReference {
        id: id.into(),
        source_url: format!("https://example.invalid/{id}.png"),
        media_kind: "image".into(),
        origin: [1.0, 2.0, 3.0],
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: Some(2.0),
        width_world: 10.0,
        hidden: false,
        locked: true,
        opacity: Some(0.5),
    }
}

/// 🗂️ The keyed reference index retires exactly: keys, lists and rows in paid turns, empty or populated.
#[test]
fn the_cad_reference_index_retires_through_the_typed_command_lane() {
    let empty = typed_input_retirement_law("cad reference index, empty", CadReferenceIndex::new());
    let index = CadReferenceIndex::from_entries(vec![("model-a".into(), vec![reference("a1"), reference("a2")]), ("model-b".into(), Vec::new()), ("model-c".into(), vec![reference("c1")])]).expect("unique keys");
    let populated = typed_input_retirement_law("cad reference index, populated", index);
    assert!(populated.declared_births > empty.declared_births && populated.turns > empty.turns, "rows and keys cost paid turns");
}
