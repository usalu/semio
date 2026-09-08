//! ↩️ `change-humidification-provided-kg-h` inverse — restores the pre-change `humidification_provided_kg_h` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_humidification_provided_kg_h::ChangeHumidificationProvidedKgH;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHumidificationProvidedKgH, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeHumidificationProvidedKgH(ChangeHumidificationProvidedKgH { new_humidification_provided_kg_h: base.humidification_provided_kg_h })]
}
//#endregion 🔖️Inverse
