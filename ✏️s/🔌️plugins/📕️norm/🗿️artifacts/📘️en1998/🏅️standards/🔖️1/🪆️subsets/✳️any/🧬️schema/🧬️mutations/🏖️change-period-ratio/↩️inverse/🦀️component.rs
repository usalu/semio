//! ↩️ `change-period-ratio` inverse — restores the pre-change `period_ratio` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_period_ratio::mutation::ChangePeriodRatio;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangePeriodRatio, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangePeriodRatio(ChangePeriodRatio { new_period_ratio: base.period_ratio.clone() })]
}
//#endregion 🔖️Inverse
