//! 🧪️ The sparse diff's own laws: it applies, it absorbs structurally, and it refuses an index that
//! does not address what it claims to.

use super::*;
use crate::schema::snapshot::Slot3d;
use protocol::MutationDiff;

fn base() -> Wfc3dSnapshot {
    crate::examples::two_room_corridor::snapshot()
}

fn slot(id: &str) -> Slot3d {
    Slot3d { id: id.into(), x: 0.0, y: 0.0, z: 0.0, width: 1.0, height: 1.0, depth: 1.0, pinned_tile_id: None }
}

#[test]
fn an_empty_diff_is_the_identity() {
    let base = base();
    assert_eq!(Wfc3dDiff::default().apply(&base).expect("the empty diff applies"), base);
}

#[test]
fn a_scalar_lane_replaces_only_its_own_field() {
    let base = base();
    let applied = Wfc3dDiff { seed: Some(99), ..Default::default() }.apply(&base).expect("seed applies");
    assert_eq!(applied.seed, 99);
    assert_eq!(applied.slots, base.slots);
}

#[test]
fn an_insertion_index_beyond_the_collection_is_refused() {
    let base = base();
    let error = Wfc3dDiff { slots_upserted: vec![(99, slot("room-z"))], ..Default::default() }.apply(&base).expect_err("an out-of-range index is refused");
    assert_eq!(error.code, "mutation.apply.invalid-index");
}

/// 🧬️ Replacing an EXISTING member must carry that member's own index, or the delta is addressing
/// something other than what it names.
#[test]
fn a_replacement_at_the_wrong_index_is_refused() {
    let base = base();
    let error = Wfc3dDiff { slots_upserted: vec![(0, slot("room-b"))], ..Default::default() }.apply(&base).expect_err("a mismatched replacement index is refused");
    assert_eq!(error.code, "mutation.apply.invalid-index");
}

#[test]
fn removing_something_that_does_not_exist_is_refused() {
    let base = base();
    let error = Wfc3dDiff { tiles_removed: vec!["ghost".into()], ..Default::default() }.apply(&base).expect_err("a missing removal target is refused");
    assert_eq!(error.code, "mutation.apply.missing-target");
}

/// 🔀 `absorb` is a structural map-merge: a later REMOVE beats an earlier upsert of the same id, and
/// a later UPSERT clears an earlier remove.
#[test]
fn absorb_lets_the_later_statement_win_per_id() {
    let mut first = Wfc3dDiff { slots_upserted: vec![(3, slot("room-c"))], ..Default::default() };
    first.absorb(Wfc3dDiff { slots_removed: vec!["room-c".into()], ..Default::default() });
    assert!(first.slots_upserted.is_empty());
    assert_eq!(first.slots_removed, vec!["room-c".to_string()]);

    let mut second = Wfc3dDiff { slots_removed: vec!["room-c".into()], ..Default::default() };
    second.absorb(Wfc3dDiff { slots_upserted: vec![(3, slot("room-c"))], ..Default::default() });
    assert!(second.slots_removed.is_empty());
    assert_eq!(second.slots_upserted.len(), 1);
}
