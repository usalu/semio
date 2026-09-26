#[test]
fn removes_first_wall() {
    use crate::mutations::remove_wall::RemoveWall;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let base = En1996Snapshot::compliant_clay_wall();
    let m = RemoveWall { index: 0 };
    let outcome = <RemoveWall as MutationKind<En1996Snapshot, crate::En1996Mutation>>::diff(&m, &base);
    assert!(outcome.diff().walls.as_ref().unwrap().values.is_empty());
}
