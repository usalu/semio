//! ↩️ `change-df-percent` inverse — restores the pre-change `df_percent` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_df_percent::mutation::ChangeDfPercent;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDfPercent, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeDfPercent(ChangeDfPercent { new_df_percent: base.df_percent.clone() })]
}
//#endregion 🔖️Inverse
