//! ↩️ `change-a-ef-mm2` inverse — restores the pre-change `a_ef_mm2` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::change_a_ef_mm2::ChangeAEfMm2;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAEfMm2, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeAEfMm2(ChangeAEfMm2 { new_a_ef_mm2: base.a_ef_mm2.clone() })]
}
//#endregion 🔖️Inverse
