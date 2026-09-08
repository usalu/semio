//! ↩️ `change-period-ratio` inverse — restores the pre-change `period_ratio` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_period_ratio::ChangePeriodRatio;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangePeriodRatio, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangePeriodRatio(ChangePeriodRatio { new_period_ratio: base.period_ratio })]
}
//#endregion 🔖️Inverse
