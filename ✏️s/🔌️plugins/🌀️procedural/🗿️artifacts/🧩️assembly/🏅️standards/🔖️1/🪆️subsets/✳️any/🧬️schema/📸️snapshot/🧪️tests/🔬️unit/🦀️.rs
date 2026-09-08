
use super::*;

#[test]
fn default_snapshot_is_empty_and_zero_seeded() {
    let snapshot = AssemblySnapshot::default();
    assert_eq!(snapshot.schema, ASSEMBLY_DOCUMENT_SCHEMA);
    assert_eq!(snapshot.seed, 0);
    assert!(snapshot.slots.is_empty() && snapshot.edges.is_empty() && snapshot.modules.is_empty() && snapshot.weights.is_empty() && snapshot.rules.is_empty());
}

#[test]
fn json_round_trips() {
    let mut snapshot = AssemblySnapshot::default();
    snapshot.slots.push(AssemblySlot { id: "s1".into(), x: 1.0, y: 2.0, z: 0.0, pinned_module_id: None });
    snapshot.edges.push(AssemblySlotEdge { id: "e1".into(), from_slot_id: "s1".into(), to_slot_id: "s1".into() });
    snapshot.weights.push(AssemblyModuleWeight { module_id: "m1".into(), weight: 2.5 });
    snapshot.rules.push(AssemblyRule { id: "r1".into(), module_a_id: "m1".into(), module_b_id: "m2".into(), allowed: true, params: SemioValue::default() });
    let bytes = dsl::json::to_json_string(&snapshot).into_bytes();
    let back: AssemblySnapshot = dsl::json::from_json_str(std::str::from_utf8(&bytes).expect("UTF-8 snapshot")).expect("decode");
    assert_eq!(snapshot, back);
}

#[test]
fn addressing_finds_existing_and_misses_unknown() {
    let mut snapshot = AssemblySnapshot::default();
    snapshot.slots.push(AssemblySlot { id: "s1".into(), ..Default::default() });
    assert_eq!(slot_index(&snapshot, "s1"), Some(0));
    assert_eq!(slot_index(&snapshot, "missing"), None);
}
