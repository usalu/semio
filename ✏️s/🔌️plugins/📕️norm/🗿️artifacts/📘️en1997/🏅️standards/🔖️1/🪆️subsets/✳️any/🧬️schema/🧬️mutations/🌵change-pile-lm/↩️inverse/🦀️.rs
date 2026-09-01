//! ↩️ `change-pile-lm` inverse — restores the pre-change `pile_l_m` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_pile_l_m::ChangePileLM;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangePileLM, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangePileLM(ChangePileLM { new_pile_l_m: base.pile_l_m.clone() })]
}
//#endregion 🔖️Inverse
