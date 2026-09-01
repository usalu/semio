//! ↩️ `change-fan-t-run-h` inverse — restores the pre-change `fan_t_run_h` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_fan_t_run_h::ChangeFanTRunH;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFanTRunH, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeFanTRunH(ChangeFanTRunH { new_fan_t_run_h: base.fan_t_run_h.clone() })]
}
//#endregion 🔖️Inverse
