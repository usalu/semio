//! ↩️ `change-fatigue-m` inverse — restores the pre-change `fatigue_m` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_fatigue_m::ChangeFatigueM;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFatigueM, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeFatigueM(ChangeFatigueM { new_fatigue_m: base.fatigue_m.clone() })]
}
//#endregion 🔖️Inverse
