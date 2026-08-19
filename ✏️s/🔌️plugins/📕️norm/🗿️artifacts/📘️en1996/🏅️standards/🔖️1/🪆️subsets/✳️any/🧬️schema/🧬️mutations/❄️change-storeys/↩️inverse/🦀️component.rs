//! ↩️ `change-storeys` inverse — restores the pre-change `storeys` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_storeys::mutation::ChangeStoreys;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeStoreys, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeStoreys(ChangeStoreys { new_storeys: base.storeys.clone() })]
}
//#endregion 🔖️Inverse
