//! 🔺️ `change-duct-test-pressure-pa` sparse diff construction — writes only `Din16798Diff.duct_test_pressure_pa` from the payload.

use crate::diff::Din16798Diff;
use crate::mutations::change_duct_test_pressure_pa::ChangeDuctTestPressurePa;
use crate::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDuctTestPressurePa, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_duct_test_pressure_pa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Duct test pressure must be a finite number, got {}.", payload.new_duct_test_pressure_pa), Vec::<String>::new());
    }
    if base.duct_test_pressure_pa == payload.new_duct_test_pressure_pa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Duct test pressure is already {}.", payload.new_duct_test_pressure_pa));
    }
    protocol::MutationOutcome::new(Din16798Diff { duct_test_pressure_pa: Some(payload.new_duct_test_pressure_pa), ..Default::default() })
}
//#endregion 🔖️Diff
