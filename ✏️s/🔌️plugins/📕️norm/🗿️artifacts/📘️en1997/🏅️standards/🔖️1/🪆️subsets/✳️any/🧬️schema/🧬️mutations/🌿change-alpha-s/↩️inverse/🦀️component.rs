//! ↩️ `change-alpha-s` inverse — restores the pre-change `alpha_s` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_alpha_s::mutation::ChangeAlphaS;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeAlphaS, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeAlphaS(ChangeAlphaS { new_alpha_s: base.alpha_s.clone() })]
}
//#endregion 🔖️Inverse
