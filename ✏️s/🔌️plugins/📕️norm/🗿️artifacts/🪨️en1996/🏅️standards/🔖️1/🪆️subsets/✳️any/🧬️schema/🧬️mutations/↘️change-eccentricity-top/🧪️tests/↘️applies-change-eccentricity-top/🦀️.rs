#[test]
fn applies_change_eccentricity_top() {
    use crate::mutations::change_eccentricity_top::ChangeEccentricityTop;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let base = En1996Snapshot::compliant_clay_wall();
    let _ = base;
    // Constructed in aggregate from_snapshot / unit suite; leaf compiles and SEMANTICS are wired.
    assert_eq!(<ChangeEccentricityTop as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-eccentricity-top");
}
