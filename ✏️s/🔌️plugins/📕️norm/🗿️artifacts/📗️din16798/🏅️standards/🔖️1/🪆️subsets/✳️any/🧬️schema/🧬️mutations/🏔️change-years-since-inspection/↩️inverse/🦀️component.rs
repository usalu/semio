//! ↩️ `change-years-since-inspection` inverse — restores the pre-change `years_since_inspection` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_years_since_inspection::mutation::ChangeYearsSinceInspection;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeYearsSinceInspection, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeYearsSinceInspection(ChangeYearsSinceInspection { new_years_since_inspection: base.years_since_inspection.clone() })]
}
//#endregion 🔖️Inverse
