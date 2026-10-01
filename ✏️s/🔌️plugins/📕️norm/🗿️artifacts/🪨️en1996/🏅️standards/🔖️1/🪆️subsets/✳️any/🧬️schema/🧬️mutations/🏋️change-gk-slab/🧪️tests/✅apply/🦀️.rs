#[test]
fn applies_change_gk_slab() {
    use crate::mutations::change_gk_slab::ChangeGKSlab;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let _ = En1996Snapshot::compliant_clay_wall();
    assert_eq!(<ChangeGKSlab as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-gk-slab");
}
