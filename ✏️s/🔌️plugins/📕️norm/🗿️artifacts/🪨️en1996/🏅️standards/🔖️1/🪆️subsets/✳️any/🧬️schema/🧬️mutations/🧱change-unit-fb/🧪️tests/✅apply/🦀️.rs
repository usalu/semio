#[test]
fn raises_unit_strength() {
    use crate::mutations::change_unit_fb::ChangeUnitFb;
    use crate::En1996Snapshot;
    use protocol::MutationKind;
    let base = En1996Snapshot::compliant_clay_wall();
    let m = ChangeUnitFb { index: 0, new_f_b_pa: 28e6 };
    let outcome = <ChangeUnitFb as MutationKind<En1996Snapshot, crate::En1996Mutation>>::diff(&m, &base);
    assert!((outcome.diff().walls.as_ref().unwrap().values[0].f_b_pa - 28e6).abs() < 1.0);
}
