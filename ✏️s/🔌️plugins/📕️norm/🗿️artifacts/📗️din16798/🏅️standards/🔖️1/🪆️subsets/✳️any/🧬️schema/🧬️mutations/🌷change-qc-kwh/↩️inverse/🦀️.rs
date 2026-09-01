//! ↩️ `change-qc-kwh` inverse — restores the pre-change `q_c_kwh` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_q_c_kwh::ChangeQCKwh;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeQCKwh, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeQCKwh(ChangeQCKwh { new_q_c_kwh: base.q_c_kwh.clone() })]
}
//#endregion 🔖️Inverse
