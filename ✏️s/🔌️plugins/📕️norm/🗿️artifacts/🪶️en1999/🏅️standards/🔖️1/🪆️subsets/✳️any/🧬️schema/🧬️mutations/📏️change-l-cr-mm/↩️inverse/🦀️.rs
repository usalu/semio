//! ↩️ `change-l-cr-mm` inverse — restores the pre-change `l_cr_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_l_cr_mm::ChangeLCrMm;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeLCrMm, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeLCrMm(ChangeLCrMm { new_l_cr_mm: base.l_cr_mm })]
}
//#endregion 🔖️Inverse
