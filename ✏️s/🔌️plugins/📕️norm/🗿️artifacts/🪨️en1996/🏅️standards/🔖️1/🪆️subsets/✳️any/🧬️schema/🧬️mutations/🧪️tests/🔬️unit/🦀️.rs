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

