#[test]
fn applies_change_load_case_situation() {
    use crate::mutations::change_load_case_situation::ChangeLoadCaseSituation;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let _ = En1996Snapshot::compliant_clay_wall();
    assert_eq!(<ChangeLoadCaseSituation as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-load-case-situation");
}
