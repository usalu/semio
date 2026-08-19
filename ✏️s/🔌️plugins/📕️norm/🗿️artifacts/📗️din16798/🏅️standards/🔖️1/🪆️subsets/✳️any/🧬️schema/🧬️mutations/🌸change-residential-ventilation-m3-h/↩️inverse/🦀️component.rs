//! ↩️ `change-residential-ventilation-m3-h` inverse — restores the pre-change `residential_ventilation_m3_h` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_residential_ventilation_m3_h::mutation::ChangeResidentialVentilationM3H;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeResidentialVentilationM3H, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeResidentialVentilationM3H(ChangeResidentialVentilationM3H { new_residential_ventilation_m3_h: base.residential_ventilation_m3_h.clone() })]
}
//#endregion 🔖️Inverse
