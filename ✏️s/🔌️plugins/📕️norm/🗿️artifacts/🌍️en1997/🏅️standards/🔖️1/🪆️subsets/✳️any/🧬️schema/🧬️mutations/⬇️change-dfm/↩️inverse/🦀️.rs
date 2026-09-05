//! ↩️ `change-dfm` inverse — restores the pre-change `d_f_m` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_d_f_m::ChangeDFM;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDFM, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeDFM(ChangeDFM { new_d_f_m: base.d_f_m.clone() })]
}
//#endregion 🔖️Inverse
