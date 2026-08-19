//! ↩️ `change-dwelling-ventilation-m3-h` inverse — restores the pre-change `dwelling_ventilation_m3_h` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_dwelling_ventilation_m3_h::mutation::ChangeDwellingVentilationM3H;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeDwellingVentilationM3H, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeDwellingVentilationM3H(ChangeDwellingVentilationM3H { new_dwelling_ventilation_m3_h: base.dwelling_ventilation_m3_h.clone() })]
}
//#endregion 🔖️Inverse
