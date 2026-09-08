//! ↩️ `change-fatigue-m` inverse — restores the pre-change `fatigue_m` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_fatigue_m::ChangeFatigueM;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFatigueM, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeFatigueM(ChangeFatigueM { new_fatigue_m: base.fatigue_m })]
}
//#endregion 🔖️Inverse
