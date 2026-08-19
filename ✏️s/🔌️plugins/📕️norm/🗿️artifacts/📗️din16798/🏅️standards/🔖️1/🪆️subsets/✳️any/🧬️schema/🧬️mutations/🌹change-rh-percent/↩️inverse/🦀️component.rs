//! ↩️ `change-rh-percent` inverse — restores the pre-change `rh_percent` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_rh_percent::mutation::ChangeRhPercent;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeRhPercent, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeRhPercent(ChangeRhPercent { new_rh_percent: base.rh_percent.clone() })]
}
//#endregion 🔖️Inverse
