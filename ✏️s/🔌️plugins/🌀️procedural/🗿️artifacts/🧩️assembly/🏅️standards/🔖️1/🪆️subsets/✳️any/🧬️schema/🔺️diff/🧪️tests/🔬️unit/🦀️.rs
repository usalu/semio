use super::*;
use protocol::MutationDiff;

fn base() -> AssemblySnapshot {
    let mut snapshot = AssemblySnapshot::default();
    snapshot.slots.push(AssemblySlot { id: "s1".into(), x: 0.0, y: 0.0, z: 0.0, pinned_module_id: None });
    snapshot
}

#[test]
fn upsert_by_id_replaces_in_place_never_duplicates() {
    let diff = AssemblyDiff { slots_upserted: vec![(0, AssemblySlot { id: "s1".into(), x: 9.0, y: 9.0, z: 0.0, pinned_module_id: None })], ..Default::default() };
    let after = diff.apply(&base()).expect("valid mutation diff");
    assert_eq!(after.slots.len(), 1);
    assert_eq!(after.slots[0].x, 9.0);
}

#[test]
fn insert_at_index_for_a_new_id() {
    let diff = AssemblyDiff { slots_upserted: vec![(1, AssemblySlot { id: "s2".into(), x: 1.0, y: 1.0, z: 0.0, pinned_module_id: None })], ..Default::default() };
    let after = diff.apply(&base()).expect("valid mutation diff");
    assert_eq!(after.slots.len(), 2);
    assert_eq!(after.slots[1].id, "s2");
}

#[test]
fn malformed_indexed_diff_rejects_without_changing_the_base() {
    let base = base();
    let diff = AssemblyDiff { slots_upserted: vec![(99, AssemblySlot { id: "s2".into(), ..Default::default() })], ..Default::default() };
    let error = diff.apply(&base).expect_err("out-of-range insertion must reject");
    assert_eq!(error.code, "mutation.apply.invalid-index");
    assert_eq!(error.target, ["slots", "upserted", "0"]);
    assert_eq!(base.slots.len(), 1);
    assert_eq!(base.slots[0].id, "s1");
}

#[test]
fn remove_drops_the_matching_id_only() {
    let diff = AssemblyDiff { slots_removed: vec!["s1".into()], ..Default::default() };
    let after = diff.apply(&base()).expect("valid mutation diff");
    assert!(after.slots.is_empty());
}

#[test]
fn absorb_a_later_remove_wins_over_an_earlier_upsert_of_the_same_id() {
    let mut d1 = AssemblyDiff { slots_upserted: vec![(0, AssemblySlot { id: "s2".into(), ..Default::default() })], ..Default::default() };
    let d2 = AssemblyDiff { slots_removed: vec!["s2".into()], ..Default::default() };
    d1.absorb(d2);
    assert!(d1.slots_upserted.is_empty());
    assert_eq!(d1.slots_removed, vec!["s2".to_string()]);
}

#[test]
fn absorb_a_later_upsert_clears_an_earlier_remove_of_the_same_id() {
    let mut d1 = AssemblyDiff { slots_removed: vec!["s1".into()], ..Default::default() };
    let d2 = AssemblyDiff { slots_upserted: vec![(0, AssemblySlot { id: "s1".into(), x: 5.0, ..Default::default() })], ..Default::default() };
    d1.absorb(d2);
    assert!(d1.slots_removed.is_empty());
    assert_eq!(d1.slots_upserted.len(), 1);
    assert_eq!(d1.slots_upserted[0].1.x, 5.0);
}

#[test]
fn absorb_composes_to_the_same_result_as_applying_sequentially() {
    let start = base();
    let d1 = AssemblyDiff { seed: Some(7), ..Default::default() };
    let mid = d1.apply(&start).expect("valid mutation diff");
    let d2 = AssemblyDiff { slots_removed: vec!["s1".into()], ..Default::default() };
    let after_sequential = d2.apply(&mid).expect("valid mutation diff");
    let mut composed = d1.clone();
    composed.absorb(d2);
    let after_composed = composed.apply(&start).expect("valid mutation diff");
    assert_eq!(after_sequential, after_composed);
}
