use crate::artifact_schema::mutations::change_annex::ChangeAnnex;
use crate::document::AnnexChoice;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_annex() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::ChangeAnnex(ChangeAnnex { new_annex: AnnexChoice::En });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
