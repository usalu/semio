//! ↩️ `change-cooling-reference-kwh` inverse — restores the pre-change `cooling_reference_kwh` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_cooling_reference_kwh::ChangeCoolingReferenceKwh;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeCoolingReferenceKwh, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeCoolingReferenceKwh(ChangeCoolingReferenceKwh { new_cooling_reference_kwh: base.cooling_reference_kwh })]
}
//#endregion 🔖️Inverse
