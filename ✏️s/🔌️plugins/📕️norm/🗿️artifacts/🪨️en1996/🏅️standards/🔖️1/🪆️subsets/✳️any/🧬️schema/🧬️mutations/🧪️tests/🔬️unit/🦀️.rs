use super::*;
use crate::document::AnnexChoice;
use protocol::Mutation;

#[test]
fn every_declared_kind_round_trips_diff() {
    let base = En1996Snapshot::compliant_clay_wall();
    let mutation = En1996Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En });
    let _ = Mutation::diff(&mutation, &base);
    assert_eq!(KINDS.len(), <En1996Mutation as protocol::SemanticMutation<En1996Snapshot>>::kinds().len());
}

#[test]
fn from_snapshot_emits_thickness_change() {
    let base = En1996Snapshot::compliant_clay_wall();
    let mut target = base.clone();
    target.walls[0].thickness_m = 0.49;
    let ms = En1996Mutation::from_snapshot(&base, &target);
    assert!(ms.iter().any(|m| matches!(m, En1996Mutation::ChangeWallThickness(_))));
}
