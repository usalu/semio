//! ↩️ `change-tower-is-chimney` inverse — restores the pre-change `tower_is_chimney` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_tower_is_chimney::ChangeTowerIsChimney;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeTowerIsChimney, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeTowerIsChimney(ChangeTowerIsChimney { new_tower_is_chimney: base.tower_is_chimney })]
}
//#endregion 🔖️Inverse
