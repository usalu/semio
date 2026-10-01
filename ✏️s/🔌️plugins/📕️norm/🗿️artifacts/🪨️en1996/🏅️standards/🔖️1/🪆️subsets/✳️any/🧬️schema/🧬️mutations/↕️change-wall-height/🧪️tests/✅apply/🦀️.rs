#[test]
fn shortens_first_wall() {
    use crate::mutations::change_wall_height::ChangeWallHeight;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let base = En1996Snapshot::compliant_clay_wall();
    let m = ChangeWallHeight { index: 0, new_height_m: 2.50 };
    let outcome = <ChangeWallHeight as MutationKind<En1996Snapshot, crate::En1996Mutation>>::diff(&m, &base);
    assert!((outcome.diff().walls.as_ref().unwrap().values[0].height_m - 2.50).abs() < 1e-9);
}
