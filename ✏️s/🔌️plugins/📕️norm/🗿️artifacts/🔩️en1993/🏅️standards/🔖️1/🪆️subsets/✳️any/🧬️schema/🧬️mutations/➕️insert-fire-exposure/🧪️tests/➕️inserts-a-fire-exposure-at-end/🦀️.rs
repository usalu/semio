use crate::mutations::insert_fire_exposure::InsertFireExposure;
use crate::{FireExposure, En1993Snapshot};
#[test]
fn inserts_at_end() {
    let base = En1993Snapshot::compliant_heb240_frame();
    let item = FireExposure { id: "fire-new".into(), member_id: "member-b1".into(), rating: "r30".into(), protection_thickness: 0.01, section_factor: 120.0, mu0: 0.4,
                design_temperature: 500.0,
                protection_conductivity: 0.20,
                protection_density: 800.0,
                protection_specific_heat: 1700.0,
            };
    let payload = InsertFireExposure { index: base.fire_exposures.len(), fire_exposure: item };
    let out = protocol::MutationKind::diff(&payload, &base);
    let next = protocol::MutationDiff::apply(out.diff(), &base).unwrap();
    assert_eq!(next.fire_exposures.len(), base.fire_exposures.len() + 1);
}
