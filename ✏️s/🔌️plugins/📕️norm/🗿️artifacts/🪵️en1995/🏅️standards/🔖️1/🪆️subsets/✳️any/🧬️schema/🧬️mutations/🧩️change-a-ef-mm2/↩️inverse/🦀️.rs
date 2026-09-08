//! ↩️ `change-a-ef-mm2` inverse — restores the pre-change `a_ef_mm2` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_a_ef_mm2::ChangeAEfMm2;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAEfMm2, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeAEfMm2(ChangeAEfMm2 { new_a_ef_mm2: base.a_ef_mm2 })]
}
//#endregion 🔖️Inverse
