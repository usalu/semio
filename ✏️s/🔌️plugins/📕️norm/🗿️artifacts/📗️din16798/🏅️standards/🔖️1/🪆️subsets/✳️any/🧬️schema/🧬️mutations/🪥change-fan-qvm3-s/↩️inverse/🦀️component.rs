//! ↩️ `change-fan-qvm3-s` inverse — restores the pre-change `fan_q_v_m3_s` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_fan_q_v_m3_s::mutation::ChangeFanQVM3S;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFanQVM3S, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeFanQVM3S(ChangeFanQVM3S { new_fan_q_v_m3_s: base.fan_q_v_m3_s.clone() })]
}
//#endregion 🔖️Inverse
