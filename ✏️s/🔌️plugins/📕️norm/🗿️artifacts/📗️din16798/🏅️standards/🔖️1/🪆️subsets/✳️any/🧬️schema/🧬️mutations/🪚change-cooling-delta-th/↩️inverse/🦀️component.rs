//! ↩️ `change-cooling-delta-th` inverse — restores the pre-change `cooling_delta_t_h` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_cooling_delta_t_h::mutation::ChangeCoolingDeltaTH;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeCoolingDeltaTH, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeCoolingDeltaTH(ChangeCoolingDeltaTH { new_cooling_delta_t_h: base.cooling_delta_t_h.clone() })]
}
//#endregion 🔖️Inverse
