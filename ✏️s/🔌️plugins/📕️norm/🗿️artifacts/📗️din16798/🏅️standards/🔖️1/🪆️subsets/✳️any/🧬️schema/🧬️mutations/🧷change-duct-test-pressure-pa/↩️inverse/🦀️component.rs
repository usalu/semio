//! ↩️ `change-duct-test-pressure-pa` inverse — restores the pre-change `duct_test_pressure_pa` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_duct_test_pressure_pa::mutation::ChangeDuctTestPressurePa;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeDuctTestPressurePa, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeDuctTestPressurePa(ChangeDuctTestPressurePa { new_duct_test_pressure_pa: base.duct_test_pressure_pa.clone() })]
}
//#endregion 🔖️Inverse
