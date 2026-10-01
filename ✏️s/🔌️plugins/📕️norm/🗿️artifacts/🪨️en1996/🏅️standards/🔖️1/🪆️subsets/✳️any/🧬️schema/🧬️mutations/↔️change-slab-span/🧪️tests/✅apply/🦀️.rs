#[test]
fn applies_change_slab_span() {
    use crate::mutations::change_slab_span::ChangeSlabSpan;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let _ = En1996Snapshot::compliant_clay_wall();
    assert_eq!(<ChangeSlabSpan as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-slab-span");
}
