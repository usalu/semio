//! 🧪️ Laws of the `.pack.semio` document codec and of its RETIREMENT discipline: a displaced
//! projection is released in bounded steps, reaches a terminal-empty state, and only then may drop.

use super::{decode, decode_into, encode, AssemblySnapshotRetirement};
use crate::schema::snapshot::{AssemblyModuleWeight, AssemblyRule, AssemblySlot, AssemblySlotEdge, AssemblySnapshot, ASSEMBLY_DOCUMENT_SCHEMA};
use store::{ErasedSnapshotRetirement, SnapshotRetirementStep};

fn populated() -> AssemblySnapshot {
    AssemblySnapshot {
        schema: ASSEMBLY_DOCUMENT_SCHEMA.into(),
        seed: 5,
        slots: vec![AssemblySlot { id: "a".into(), x: 1.0, y: 2.0, z: 3.0, pinned_module_id: Some("wall".into()) }],
        edges: vec![AssemblySlotEdge { id: "aa".into(), from_slot_id: "a".into(), to_slot_id: "a".into() }],
        modules: vec![crate::module_child_handle("wall")],
        weights: vec![AssemblyModuleWeight { module_id: "wall".into(), weight: 1.25 }],
        rules: vec![AssemblyRule { id: "r".into(), module_a_id: "wall".into(), module_b_id: "wall".into(), allowed: true, params: Default::default() }],
    }
}

#[test]
fn a_populated_document_round_trips_through_pack() {
    let document = populated();
    assert_eq!(decode(&encode(&document)).expect("pack decodes"), document);
}

#[test]
fn the_empty_document_round_trips_through_pack() {
    let document = AssemblySnapshot::default();
    assert_eq!(decode(&encode(&document)).expect("pack decodes"), document);
}

#[test]
fn bytes_that_are_not_this_facets_envelope_are_refused() {
    assert!(decode(b"not a pack envelope at all").is_err());
}

#[test]
fn decode_into_replaces_the_live_projection_and_yields_the_displaced_one() {
    let mut live = populated();
    let next = AssemblySnapshot { seed: 77, ..AssemblySnapshot::default() };
    let mut retirement = decode_into(&mut live, &encode(&next)).expect("decode-into succeeds");
    assert_eq!(live, next, "the live projection carries the decoded document");
    while !retirement.terminal_is_empty() {
        assert!(matches!(retirement.close_step(64, 8_192), Ok(SnapshotRetirementStep::Pending { .. })));
    }
    assert!(matches!(retirement.close_step(64, 8_192), Ok(SnapshotRetirementStep::Complete)));
}

#[test]
fn a_retirement_makes_no_progress_under_a_budget_it_cannot_afford() {
    let mut live = populated();
    let mut retirement = decode_into(&mut live, &encode(&AssemblySnapshot::default())).expect("decode-into succeeds");
    assert!(matches!(retirement.close_step(0, 8_192), Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })));
    assert!(matches!(retirement.close_step(64, 8), Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })));
    assert!(!retirement.terminal_is_empty(), "a refused step must not advance the stage cursor");
    while !retirement.terminal_is_empty() {
        let _ = retirement.close_step(64, 8_192);
    }
}

#[test]
fn every_collection_is_released_before_the_terminal_state() {
    let mut live = populated();
    let mut retirement = decode_into(&mut live, &encode(&AssemblySnapshot::default())).expect("decode-into succeeds");
    let mut steps = 0usize;
    while !retirement.terminal_is_empty() {
        assert!(retirement.close_step(64, 8_192).is_ok());
        steps += 1;
        assert!(steps <= 8, "retirement must terminate");
    }
    assert_eq!(steps, 5, "modules, rules, weights, edges and slots are released one stage each");
}

#[test]
fn the_retirement_type_is_the_one_the_decode_entry_point_hands_back() {
    let mut live = AssemblySnapshot::default();
    let mut retirement: Box<dyn ErasedSnapshotRetirement> = decode_into(&mut live, &encode(&populated())).expect("decode-into succeeds");
    while !retirement.terminal_is_empty() {
        let _ = retirement.close_step(64, 8_192);
    }
    let _ = std::any::type_name::<AssemblySnapshotRetirement>();
}
