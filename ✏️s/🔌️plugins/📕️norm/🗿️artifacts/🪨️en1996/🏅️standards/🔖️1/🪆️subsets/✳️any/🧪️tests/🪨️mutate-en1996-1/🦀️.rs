//! Language-agnostic mutation round-trips for the masonry-building subject.

use crate::document::AnnexChoice;
use crate::mutations::{change_annex, change_wall_thickness, insert_wall, remove_wall};
use crate::{En1996Mutation, En1996Snapshot};
use protocol::{Mutation, MutationDiff};

#[test]
fn change_annex_applies() {
    let base = En1996Snapshot::compliant_clay_wall();
    let mutation = En1996Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En });
    let next = Mutation::diff(&mutation, &base).diff().apply(&base).expect("apply");
    assert_eq!(next.annex, AnnexChoice::En);
}

#[test]
fn change_wall_thickness_applies() {
    let base = En1996Snapshot::compliant_clay_wall();
    let mutation = En1996Mutation::ChangeWallThickness(change_wall_thickness::ChangeWallThickness { index: 0, new_thickness_m: 0.49 });
    let next = Mutation::diff(&mutation, &base).diff().apply(&base).expect("apply");
    assert!((next.walls[0].thickness_m - 0.49).abs() < 1e-12);
}

#[test]
fn insert_and_remove_wall_apply() {
    let base = En1996Snapshot::compliant_clay_wall();
    let mut wall = base.walls[0].clone();
    wall.id = "wall-2".into();
    let inserted = Mutation::diff(&En1996Mutation::InsertWall(insert_wall::InsertWall { index: 1, wall }), &base).diff().apply(&base).expect("insert");
    assert_eq!(inserted.walls.len(), 2);
    let removed = Mutation::diff(&En1996Mutation::RemoveWall(remove_wall::RemoveWall { index: 0 }), &base).diff().apply(&base).expect("remove");
    assert!(removed.walls.is_empty());
}

#[test]
fn kinds_catalog_has_full_vocabulary() {
    assert!(crate::mutations::KINDS.len() >= 40);
}
