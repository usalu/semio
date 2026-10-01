#[test]
fn applies_change_qk_imposed() {
    use crate::mutations::change_qk_imposed::ChangeQKImposed;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let _ = En1996Snapshot::compliant_clay_wall();
    assert_eq!(<ChangeQKImposed as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-qk-imposed");
}
