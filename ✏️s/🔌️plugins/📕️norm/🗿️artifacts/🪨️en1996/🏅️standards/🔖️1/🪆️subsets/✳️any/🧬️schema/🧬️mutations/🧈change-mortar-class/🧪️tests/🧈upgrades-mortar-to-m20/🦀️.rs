#[test]
fn upgrades_mortar_to_m20() {
    use crate::mutations::change_mortar_class::ChangeMortarClass;
    use crate::{En1996Snapshot, MortarClass};
    use protocol::MutationKind;
    let base = En1996Snapshot::compliant_clay_wall();
    let m = ChangeMortarClass { index: 0, new_mortar_class: MortarClass::M20 };
    let outcome = <ChangeMortarClass as MutationKind<En1996Snapshot, crate::En1996Mutation>>::diff(&m, &base);
    assert_eq!(outcome.diff().walls.as_ref().unwrap().values[0].mortar_class, MortarClass::M20);
}
