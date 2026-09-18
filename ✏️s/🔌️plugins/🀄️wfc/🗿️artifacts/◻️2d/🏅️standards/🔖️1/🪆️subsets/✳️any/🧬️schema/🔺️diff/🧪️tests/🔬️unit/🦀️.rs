//! 🧪️ `Wfc2dDiff` — apply and absorb laws.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSlot;

fn base() -> crate::Wfc2dSnapshot {
    crate::examples::two_room_corridor::document()
}

/// 🕳️ The identity delta leaves the document untouched.
#[test]
fn empty_diff_is_the_identity() {
    let document = base();
    let produced = <Wfc2dDiff as protocol::MutationDiff<crate::Wfc2dSnapshot>>::apply(&Wfc2dDiff::default(), &document).expect("identity applies");
    assert_eq!(produced, document);
}

/// 🔀 A later remove beats an earlier upsert of the SAME id.
#[test]
fn absorb_lets_a_later_remove_win() {
    let mut first = Wfc2dDiff { slots_upserted: vec![(3, Wfc2dSlot { id: "room-c".into(), x: 6.0, y: 0.0, width: 2.0, height: 2.0, pinned_tile_id: None })], ..Default::default() };
    protocol::MutationDiff::<crate::Wfc2dSnapshot>::absorb(&mut first, Wfc2dDiff { slots_removed: vec!["room-c".into()], ..Default::default() });
    assert!(first.slots_upserted.is_empty());
    assert_eq!(first.slots_removed, vec!["room-c".to_string()]);
}

/// 🚫️ Removing a row the base never held is refused, not silently ignored.
#[test]
fn apply_refuses_a_missing_removal() {
    let diff = Wfc2dDiff { slots_removed: vec!["ghost".into()], ..Default::default() };
    assert!(<Wfc2dDiff as protocol::MutationDiff<crate::Wfc2dSnapshot>>::apply(&diff, &base()).is_err());
}
