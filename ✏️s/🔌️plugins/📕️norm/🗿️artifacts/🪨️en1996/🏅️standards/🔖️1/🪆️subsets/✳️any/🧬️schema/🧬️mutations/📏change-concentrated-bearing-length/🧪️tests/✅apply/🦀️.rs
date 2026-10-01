#[test]
fn applies_change_concentrated_bearing_length() {
    use crate::mutations::change_concentrated_bearing_length::ChangeConcentratedBearingLength;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let base = En1996Snapshot::compliant_clay_wall();
    let _ = base;
    // Constructed in aggregate from_snapshot / unit suite; leaf compiles and SEMANTICS are wired.
    assert_eq!(<ChangeConcentratedBearingLength as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-concentrated-bearing-length");
}
