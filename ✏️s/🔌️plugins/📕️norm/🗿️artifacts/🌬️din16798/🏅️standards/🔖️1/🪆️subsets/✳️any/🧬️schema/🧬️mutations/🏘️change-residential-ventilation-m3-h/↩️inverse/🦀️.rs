//! ↩️ `change-residential-ventilation-m3-h` inverse — restores the pre-change `residential_ventilation_m3_h` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_residential_ventilation_m3_h::ChangeResidentialVentilationM3H;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeResidentialVentilationM3H, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeResidentialVentilationM3H(ChangeResidentialVentilationM3H { new_residential_ventilation_m3_h: base.residential_ventilation_m3_h })]
}
//#endregion 🔖️Inverse
