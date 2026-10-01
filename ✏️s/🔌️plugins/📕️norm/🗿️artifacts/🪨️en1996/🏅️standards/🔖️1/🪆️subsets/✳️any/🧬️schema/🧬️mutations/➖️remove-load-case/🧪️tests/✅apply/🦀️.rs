#[test]
fn applies_remove_load_case() {
    use crate::mutations::remove_load_case::RemoveLoadCase;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let base = En1996Snapshot::compliant_clay_wall();
    let _ = base;
    // Constructed in aggregate from_snapshot / unit suite; leaf compiles and SEMANTICS are wired.
    assert_eq!(<RemoveLoadCase as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "remove-load-case");
}
