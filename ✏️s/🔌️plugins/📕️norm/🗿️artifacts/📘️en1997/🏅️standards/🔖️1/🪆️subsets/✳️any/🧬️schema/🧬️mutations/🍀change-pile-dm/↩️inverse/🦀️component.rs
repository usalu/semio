//! ↩️ `change-pile-dm` inverse — restores the pre-change `pile_d_m` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_pile_d_m::mutation::ChangePileDM;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangePileDM, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangePileDM(ChangePileDM { new_pile_d_m: base.pile_d_m.clone() })]
}
//#endregion 🔖️Inverse
