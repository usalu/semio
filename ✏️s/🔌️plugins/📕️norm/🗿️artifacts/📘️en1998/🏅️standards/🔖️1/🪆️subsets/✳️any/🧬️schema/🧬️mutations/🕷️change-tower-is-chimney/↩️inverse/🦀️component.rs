//! ↩️ `change-tower-is-chimney` inverse — restores the pre-change `tower_is_chimney` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_tower_is_chimney::mutation::ChangeTowerIsChimney;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeTowerIsChimney, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeTowerIsChimney(ChangeTowerIsChimney { new_tower_is_chimney: base.tower_is_chimney.clone() })]
}
//#endregion 🔖️Inverse
