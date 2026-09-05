//! ↩️ `change-as-mm2` inverse — restores the pre-change `a_s_mm2` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_a_s_mm2::ChangeASMm2;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeASMm2, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeASMm2(ChangeASMm2 { new_a_s_mm2: base.a_s_mm2.clone() })]
}
//#endregion 🔖️Inverse
