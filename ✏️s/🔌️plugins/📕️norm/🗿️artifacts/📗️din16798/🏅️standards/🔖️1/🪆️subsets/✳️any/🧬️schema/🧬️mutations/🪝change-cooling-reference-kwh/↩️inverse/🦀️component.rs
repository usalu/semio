//! ↩️ `change-cooling-reference-kwh` inverse — restores the pre-change `cooling_reference_kwh` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_cooling_reference_kwh::mutation::ChangeCoolingReferenceKwh;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeCoolingReferenceKwh, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeCoolingReferenceKwh(ChangeCoolingReferenceKwh { new_cooling_reference_kwh: base.cooling_reference_kwh.clone() })]
}
//#endregion 🔖️Inverse
