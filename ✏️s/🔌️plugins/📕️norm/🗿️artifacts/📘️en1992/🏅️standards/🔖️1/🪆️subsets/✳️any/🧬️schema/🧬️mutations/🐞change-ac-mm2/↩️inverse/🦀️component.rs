//! ↩️ `change-ac-mm2` inverse — restores the pre-change `a_c_mm2` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_a_c_mm2::mutation::ChangeACMm2;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeACMm2, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeACMm2(ChangeACMm2 { new_a_c_mm2: base.a_c_mm2.clone() })]
}
//#endregion 🔖️Inverse
