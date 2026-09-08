//! ↩️ `change-rh-percent` inverse — restores the pre-change `rh_percent` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_rh_percent::ChangeRhPercent;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeRhPercent, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeRhPercent(ChangeRhPercent { new_rh_percent: base.rh_percent })]
}
//#endregion 🔖️Inverse
