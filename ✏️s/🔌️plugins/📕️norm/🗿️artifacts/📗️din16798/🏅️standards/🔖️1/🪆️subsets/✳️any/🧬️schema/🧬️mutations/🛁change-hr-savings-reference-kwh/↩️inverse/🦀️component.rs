//! ↩️ `change-hr-savings-reference-kwh` inverse — restores the pre-change `hr_savings_reference_kwh` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_hr_savings_reference_kwh::mutation::ChangeHrSavingsReferenceKwh;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHrSavingsReferenceKwh, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeHrSavingsReferenceKwh(ChangeHrSavingsReferenceKwh { new_hr_savings_reference_kwh: base.hr_savings_reference_kwh.clone() })]
}
//#endregion 🔖️Inverse
