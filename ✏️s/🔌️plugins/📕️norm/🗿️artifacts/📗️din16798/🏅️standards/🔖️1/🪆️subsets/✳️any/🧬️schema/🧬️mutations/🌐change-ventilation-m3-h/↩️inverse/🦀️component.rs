//! ↩️ `change-ventilation-m3-h` inverse — restores the pre-change `ventilation_m3_h` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_ventilation_m3_h::mutation::ChangeVentilationM3H;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeVentilationM3H, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeVentilationM3H(ChangeVentilationM3H { new_ventilation_m3_h: base.ventilation_m3_h.clone() })]
}
//#endregion 🔖️Inverse
