//! ↩️ `change-sfp-wm3-s` inverse — restores the pre-change `sfp_w_m3_s` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_sfp_w_m3_s::mutation::ChangeSfpWM3S;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeSfpWM3S, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeSfpWM3S(ChangeSfpWM3S { new_sfp_w_m3_s: base.sfp_w_m3_s.clone() })]
}
//#endregion 🔖️Inverse
