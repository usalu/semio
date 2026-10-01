#[test]
fn applies_change_qk_snow() {
    use crate::mutations::change_qk_snow::ChangeQKSnow;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let _ = En1996Snapshot::compliant_clay_wall();
    assert_eq!(<ChangeQKSnow as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-qk-snow");
}
