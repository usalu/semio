//! ↩️ `change-storeys` inverse — restores the pre-change `storeys` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_storeys::ChangeStoreys;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeStoreys, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeStoreys(ChangeStoreys { new_storeys: base.storeys })]
}
//#endregion 🔖️Inverse
