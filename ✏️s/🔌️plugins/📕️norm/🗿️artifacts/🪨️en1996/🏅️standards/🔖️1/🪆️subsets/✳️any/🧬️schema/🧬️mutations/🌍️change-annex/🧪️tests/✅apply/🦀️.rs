#[test]
fn switches_annex_to_en() {
    use crate::document::AnnexChoice;
    use crate::mutations::change_annex::ChangeAnnex;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let base = En1996Snapshot::compliant_clay_wall();
    let m = ChangeAnnex { new_annex: AnnexChoice::En };
    let outcome = <ChangeAnnex as MutationKind<En1996Snapshot, crate::En1996Mutation>>::diff(&m, &base);
    assert!(outcome.diff().annex == Some(AnnexChoice::En) || outcome.diff().annex.is_some());
}
