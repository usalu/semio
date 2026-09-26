#[test]
fn applies_change_phi_infinity() {
    use crate::mutations::change_phi_infinity::ChangePhiInfinity;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let _ = En1996Snapshot::compliant_clay_wall();
    assert_eq!(<ChangePhiInfinity as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-phi-infinity");
}
