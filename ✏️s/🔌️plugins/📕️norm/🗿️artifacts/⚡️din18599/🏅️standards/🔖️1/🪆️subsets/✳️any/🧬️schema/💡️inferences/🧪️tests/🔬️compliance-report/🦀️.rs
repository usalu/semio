use super::*;
use crate::document::ClimateZoneDe;
use crate::standards::v1::subsets::any::schema::{from_building, reference_wall_layers};

fn reference_100m2_inputs() -> BalancingInputs {
    from_building(&reference_wall_layers(), 100.0, 4, ClimateZoneDe::Zone2, 0.0).unwrap()
}

#[semio_framework_async_macros::async_test]
async fn balance_annual_includes_all_parts() {
    let inputs = reference_100m2_inputs();
    let report = balance_annual(&inputs).unwrap();
    assert_eq!(report.checks.len(), 12);
}

#[semio_framework_async_macros::async_test]
async fn part_1_check_reached_via_balance_annual() {
    let inputs = reference_100m2_inputs();
    let check = part_1::check(&inputs).unwrap();
    assert_eq!(check.clause.family, "DIN V 18599-1");
    let report = balance_annual(&inputs).unwrap();
    assert!(report.checks.iter().any(|c| c.clause.family == "DIN V 18599-1" && c.clause.part == "§6"));
}
