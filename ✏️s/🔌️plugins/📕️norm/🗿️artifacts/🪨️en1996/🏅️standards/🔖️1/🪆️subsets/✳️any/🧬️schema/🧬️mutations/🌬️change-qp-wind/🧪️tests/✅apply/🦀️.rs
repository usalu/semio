#[test]
fn applies_change_qp_wind() {
    use crate::mutations::change_qp_wind::ChangeQPWind;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let _ = En1996Snapshot::compliant_clay_wall();
    assert_eq!(<ChangeQPWind as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-qp-wind");
}
