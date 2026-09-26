#[test]
fn applies_change_is_basement() {
    use crate::mutations::change_is_basement::ChangeIsBasement;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let _ = En1996Snapshot::compliant_clay_wall();
    assert_eq!(<ChangeIsBasement as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-is-basement");
}
