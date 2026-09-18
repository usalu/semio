//! 🔬️ Diff laws — an empty diff is the identity, absorb is a real structural merge, and a delta that
//! names a member the base does not carry is refused instead of silently applied.

use super::*;
use crate::schema::snapshot::Grid3dTile;
use protocol::MutationDiff;

fn base() -> Grid3dSnapshot {
    crate::examples::blocks::snapshot()
}

#[test]
fn an_empty_diff_is_the_identity() {
    assert_eq!(Grid3dDiff::default().apply(&base()).expect("empty diff applies"), base());
}

#[test]
fn removing_a_member_the_base_does_not_carry_is_refused() {
    let diff = Grid3dDiff { tiles_removed: vec!["nothing".into()], ..Default::default() };
    assert!(diff.apply(&base()).is_err());
}

#[test]
fn an_insertion_index_past_the_end_is_refused() {
    let tile = Grid3dTile { id: "zzz".into(), label: None, weight: 1.0, media: Default::default() };
    let diff = Grid3dDiff { tiles_upserted: vec![(99, tile)], ..Default::default() };
    assert!(diff.apply(&base()).is_err());
}

#[test]
fn a_later_remove_wins_over_an_earlier_upsert_of_the_same_key() {
    let tile = Grid3dTile { id: "zzz".into(), label: None, weight: 1.0, media: Default::default() };
    let mut first = Grid3dDiff { tiles_upserted: vec![(4, tile)], ..Default::default() };
    first.absorb(Grid3dDiff { tiles_removed: vec!["zzz".into()], ..Default::default() });
    assert!(first.tiles_upserted.is_empty());
    assert_eq!(first.tiles_removed, vec!["zzz".to_string()]);
}

#[test]
fn a_later_upsert_clears_an_earlier_remove_of_the_same_key() {
    let tile = Grid3dTile { id: "wall".into(), label: None, weight: 9.0, media: Default::default() };
    let mut first = Grid3dDiff { tiles_removed: vec!["wall".into()], ..Default::default() };
    first.absorb(Grid3dDiff { tiles_upserted: vec![(3, tile)], ..Default::default() });
    assert!(first.tiles_removed.is_empty());
    assert_eq!(first.tiles_upserted.len(), 1);
}

#[test]
fn every_scalar_lane_overwrites_only_when_the_later_diff_states_it() {
    let mut first = Grid3dDiff { seed: Some(1), width: Some(2), ..Default::default() };
    first.absorb(Grid3dDiff { seed: Some(9), ..Default::default() });
    assert_eq!(first.seed, Some(9));
    assert_eq!(first.width, Some(2));
}
