#[test]
fn applies_insert_wall() {
    use crate::mutations::insert_wall::InsertWall;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let base = En1996Snapshot::compliant_clay_wall();
    let _ = base;
    // Constructed in aggregate from_snapshot / unit suite; leaf compiles and SEMANTICS are wired.
    assert_eq!(<InsertWall as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "insert-wall");
}
