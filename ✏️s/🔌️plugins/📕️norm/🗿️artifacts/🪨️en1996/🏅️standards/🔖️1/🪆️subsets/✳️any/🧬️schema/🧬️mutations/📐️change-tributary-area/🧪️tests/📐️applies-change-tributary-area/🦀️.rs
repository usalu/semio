#[test]
fn applies_change_tributary_area() {
    use crate::mutations::change_tributary_area::ChangeTributaryArea;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let _ = En1996Snapshot::compliant_clay_wall();
    assert_eq!(<ChangeTributaryArea as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-tributary-area");
}
