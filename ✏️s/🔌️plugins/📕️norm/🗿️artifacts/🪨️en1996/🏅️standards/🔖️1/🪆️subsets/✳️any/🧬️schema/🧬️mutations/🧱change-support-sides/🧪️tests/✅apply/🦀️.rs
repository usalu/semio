#[test]
fn sets_four_sided_support() {
    use crate::mutations::change_support_sides::ChangeSupportSides;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let base = En1996Snapshot::noncompliant_multi_fail();
    let m = ChangeSupportSides { index: 0, new_support_sides: 4 };
    let outcome = <ChangeSupportSides as MutationKind<En1996Snapshot, crate::En1996Mutation>>::diff(&m, &base);
    assert_eq!(outcome.diff().walls.as_ref().unwrap().values[0].support_sides, 4);
}
