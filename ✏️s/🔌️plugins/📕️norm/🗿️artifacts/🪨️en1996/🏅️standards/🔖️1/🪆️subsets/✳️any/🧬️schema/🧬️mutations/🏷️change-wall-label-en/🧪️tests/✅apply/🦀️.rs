#[test]
fn applies_change_wall_label_en() {
    use crate::mutations::change_wall_label_en::ChangeWallLabelEn;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let _ = En1996Snapshot::compliant_clay_wall();
    assert_eq!(<ChangeWallLabelEn as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-wall-label-en");
}
