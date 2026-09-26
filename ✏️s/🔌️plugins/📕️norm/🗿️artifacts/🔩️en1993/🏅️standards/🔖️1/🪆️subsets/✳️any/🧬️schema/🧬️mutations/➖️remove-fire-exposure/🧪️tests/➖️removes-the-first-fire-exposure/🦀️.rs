use crate::mutations::{remove_fire_exposure::RemoveFireExposure, insert_fire_exposure::InsertFireExposure};
use crate::{FireExposure, En1993Snapshot};
#[test]
fn removes_first() {
    let base0 = En1993Snapshot::compliant_heb240_frame();
    let item = FireExposure { id: "fire-new".into(), member_id: "member-b1".into(), rating: "r30".into(), protection_thickness: 0.01, section_factor: 120.0, mu0: 0.4,
                design_temperature: 500.0,
                protection_conductivity: 0.20,
                protection_density: 800.0,
                protection_specific_heat: 1700.0,
            };
    let inserted = protocol::MutationDiff::apply(
        protocol::MutationKind::diff(&InsertFireExposure { index: 0, fire_exposure: item }, &base0).diff(),
        &base0,
    ).unwrap();
    let before_len = inserted.fire_exposures.len();
    let out = protocol::MutationKind::diff(&RemoveFireExposure { index: 0 }, &inserted);
    let next = protocol::MutationDiff::apply(out.diff(), &inserted).unwrap();
    assert_eq!(next.fire_exposures.len(), before_len - 1);
}
