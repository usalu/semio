#[test]
fn applies_change_c_pe() {
    use crate::mutations::change_c_pe::ChangeCPe;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let _ = En1996Snapshot::compliant_clay_wall();
    assert_eq!(<ChangeCPe as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-c-pe");
}
