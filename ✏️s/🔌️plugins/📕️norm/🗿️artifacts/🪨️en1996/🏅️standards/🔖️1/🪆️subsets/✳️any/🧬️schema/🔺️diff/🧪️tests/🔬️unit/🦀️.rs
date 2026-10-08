use crate::document::AnnexChoice;
use crate::mutations::change_annex::ChangeAnnex;
use crate::{En1996Diff, En1996Mutation, En1996Snapshot};
use protocol::{apply_diff, DiffAlgebra, Mutation};

#[test]
fn change_annex_diff_updates_only_annex() {
    let base = En1996Snapshot::compliant_clay_wall();
    let mutation = En1996Mutation::ChangeAnnex(ChangeAnnex { new_annex: AnnexChoice::En });
    let outcome = Mutation::diff(&mutation, &base);
    let mut expected = base.clone();
    expected.annex = AnnexChoice::En;
    assert_eq!(apply_diff(outcome.diff(), &base).expect("valid mutation diff"), expected);
}

#[test]
fn between_carries_a_document_to_another_and_inverts() {
    let base = En1996Snapshot::compliant_clay_wall();
    let mut target = base.clone();
    target.storeys += 2;
    target.walls[0].thickness_m = 0.49;
    target.walls[0].openings.push(crate::WallOpening { id: "win".into(), width_m: 1.0, height_m: 1.2, sill_height_m: 0.9 });
    target.walls[0].load_cases[0].concentrated.push(crate::ConcentratedLoad { id: "c1".into(), force_n: 10.0, bearing_area_m2: 0.1, bearing_length_m: 0.2 });
    let mut second = target.walls[0].clone();
    second.id = "wall-extra".into();
    target.walls.insert(0, second);
    let delta = En1996Diff::between(&base, &target);
    assert_eq!(apply_diff(&delta, &base).expect("between applies"), target);
    assert_eq!(apply_diff(&delta.inverse(&base), &target).expect("inverse applies"), base);
    assert!(En1996Diff::between(&base, &base).is_empty());
}

#[test]
fn absorb_of_two_steps_equals_their_sequence() {
    let base = En1996Snapshot::compliant_clay_wall();
    let first = Mutation::diff(&En1996Mutation::ChangeStoreys(crate::mutations::change_storeys::ChangeStoreys { new_storeys: 7 }), &base).diff().clone();
    let mid = apply_diff(&first, &base).expect("first applies");
    let second = Mutation::diff(&En1996Mutation::ChangeWallLength(crate::mutations::change_wall_length::ChangeWallLength { index: 0, new_length_m: 9.5 }), &mid).diff().clone();
    let after = apply_diff(&second, &mid).expect("second applies");
    let mut sum = first;
    protocol::MutationDiff::absorb(&mut sum, second);
    assert_eq!(apply_diff(&sum, &base).expect("sum applies"), after);
}
