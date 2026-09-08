//! ↩️ `change-t-ef-mm` inverse — restores the pre-change `t_ef_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_t_ef_mm::ChangeTEfMm;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeTEfMm, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeTEfMm(ChangeTEfMm { new_t_ef_mm: base.t_ef_mm })]
}
//#endregion 🔖️Inverse
