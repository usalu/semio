#[test]
fn thickens_first_wall() {
    use crate::mutations::change_wall_thickness::ChangeWallThickness;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let base = En1996Snapshot::compliant_clay_wall();
    let m = ChangeWallThickness { index: 0, new_thickness_m: 0.490 };
    let outcome = <ChangeWallThickness as MutationKind<En1996Snapshot, crate::En1996Mutation>>::diff(&m, &base);
    assert!((outcome.diff().walls.as_ref().unwrap().values[0].thickness_m - 0.490).abs() < 1e-9);
}
