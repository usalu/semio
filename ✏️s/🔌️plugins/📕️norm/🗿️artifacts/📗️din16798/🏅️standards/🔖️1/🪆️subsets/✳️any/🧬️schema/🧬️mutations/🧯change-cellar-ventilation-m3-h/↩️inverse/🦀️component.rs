//! ↩️ `change-cellar-ventilation-m3-h` inverse — restores the pre-change `cellar_ventilation_m3_h` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_cellar_ventilation_m3_h::mutation::ChangeCellarVentilationM3H;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeCellarVentilationM3H, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeCellarVentilationM3H(ChangeCellarVentilationM3H { new_cellar_ventilation_m3_h: base.cellar_ventilation_m3_h.clone() })]
}
//#endregion 🔖️Inverse
