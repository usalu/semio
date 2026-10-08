use super::*;
use crate::mutations::En1991Mutation;
use protocol::{apply_diff, DiffAlgebra, Mutation as _, MutationDiff};

/// 🔺️ A floor change patches only that floor's field; the document settings and the other lists stay untouched.
#[test]
fn change_floor_diff_patches_only_its_field() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeFloorAssumedQk(crate::mutations::change_floor_assumed_qk::ChangeFloorAssumedQk { index: 0, new_assumed_qk: base.floors[0].assumed_qk + 100.0 });
    let outcome = mutation.diff(&base);
    let diff = outcome.diff();
    assert!(diff.annex.is_none() && diff.roofs.is_empty() && diff.wind_faces.is_empty());
    assert_eq!(diff.floors.modified.len(), 1);
    assert_eq!(diff.floors.modified[0].patch.assumed_qk, Some(base.floors[0].assumed_qk + 100.0));
    let mut expected = base.clone();
    expected.floors[0].assumed_qk += 100.0;
    assert_eq!(apply_diff(diff, &base).expect("valid mutation diff"), expected);
}

/// 🧲 `absorb` coalesces a create followed by its delete into nothing and keeps sequential semantics for patches.
#[test]
fn absorb_cancels_a_create_followed_by_its_delete() {
    let base = En1991Snapshot::default();
    let mut floor = base.floors[0].clone();
    floor.id = "floor-extra".into();
    let mut sum = En1991Mutation::InsertFloors(crate::mutations::insert_floors::InsertFloors { index: 1, item: floor }).diff(&base).diff().clone();
    let mid = apply_diff(&sum, &base).expect("the insertion applies");
    sum.absorb(En1991Diff { floors: En1991FloorDelta::removal(&mid.floors, 1), ..Default::default() });
    assert!(sum.is_empty());
}

