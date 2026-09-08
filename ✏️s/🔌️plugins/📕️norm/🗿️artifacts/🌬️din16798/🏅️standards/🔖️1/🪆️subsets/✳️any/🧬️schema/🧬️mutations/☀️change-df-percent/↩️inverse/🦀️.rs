//! ↩️ `change-df-percent` inverse — restores the pre-change `df_percent` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_df_percent::ChangeDfPercent;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDfPercent, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeDfPercent(ChangeDfPercent { new_df_percent: base.df_percent })]
}
//#endregion 🔖️Inverse
