#[test]
fn inserts_a_wall() {
    use crate::mutations::insert_wall::InsertWall;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let base = En1996Snapshot::compliant_clay_wall();
    let mut wall = base.walls[0].clone();
    wall.id = "wall-2".into();
    let m = InsertWall { index: 1, wall };
    let outcome = <InsertWall as MutationKind<En1996Snapshot, crate::En1996Mutation>>::diff(&m, &base);
    assert_eq!(outcome.diff().walls.as_ref().unwrap().values.len(), 2);
}
