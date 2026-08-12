//! 🔺️ `change-duct-test-pressure-pa` sparse diff construction — writes only `Din16798Diff.duct_test_pressure_pa` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_duct_test_pressure_pa::mutation::ChangeDuctTestPressurePa;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDuctTestPressurePa, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { duct_test_pressure_pa: Some(payload.new_duct_test_pressure_pa.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
