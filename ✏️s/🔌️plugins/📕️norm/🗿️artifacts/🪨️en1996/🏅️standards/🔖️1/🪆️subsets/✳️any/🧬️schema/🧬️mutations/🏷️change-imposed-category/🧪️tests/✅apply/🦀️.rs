#[test]
fn applies_change_imposed_category() {
    use crate::mutations::change_imposed_category::ChangeImposedCategory;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let _ = En1996Snapshot::compliant_clay_wall();
    assert_eq!(<ChangeImposedCategory as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-imposed-category");
}
