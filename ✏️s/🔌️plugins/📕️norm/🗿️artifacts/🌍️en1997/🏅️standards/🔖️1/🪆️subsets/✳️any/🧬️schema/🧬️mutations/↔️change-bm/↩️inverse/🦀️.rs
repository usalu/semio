//! ↩️ `change-bm` inverse — restores the pre-change `b_m` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_b_m::ChangeBM;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBM, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeBM(ChangeBM { new_b_m: base.b_m.clone() })]
}
//#endregion 🔖️Inverse
