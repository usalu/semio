//! ↩️ `change-humidification-required-kg-h` inverse — restores the pre-change `humidification_required_kg_h` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_humidification_required_kg_h::mutation::ChangeHumidificationRequiredKgH;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHumidificationRequiredKgH, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeHumidificationRequiredKgH(ChangeHumidificationRequiredKgH { new_humidification_required_kg_h: base.humidification_required_kg_h.clone() })]
}
//#endregion 🔖️Inverse
