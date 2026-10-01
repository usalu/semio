#[test]
fn applies_change_wall_label_de() {
    use crate::mutations::change_wall_label_de::ChangeWallLabelDe;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let _ = En1996Snapshot::compliant_clay_wall();
    assert_eq!(<ChangeWallLabelDe as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-wall-label-de");
}
