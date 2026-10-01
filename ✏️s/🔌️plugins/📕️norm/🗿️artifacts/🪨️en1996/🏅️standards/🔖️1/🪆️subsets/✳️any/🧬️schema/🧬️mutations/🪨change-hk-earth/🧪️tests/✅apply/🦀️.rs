#[test]
fn applies_change_hk_earth() {
    use crate::mutations::change_hk_earth::ChangeHKEarth;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let _ = En1996Snapshot::compliant_clay_wall();
    assert_eq!(<ChangeHKEarth as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-hk-earth");
}
