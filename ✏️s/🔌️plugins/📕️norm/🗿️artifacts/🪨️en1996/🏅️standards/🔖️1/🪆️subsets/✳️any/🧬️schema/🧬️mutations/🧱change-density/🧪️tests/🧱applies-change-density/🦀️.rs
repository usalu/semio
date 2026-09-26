#[test]
fn applies_change_density() {
    use crate::mutations::change_density::ChangeDensity;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let _ = En1996Snapshot::compliant_clay_wall();
    assert_eq!(<ChangeDensity as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-density");
}
