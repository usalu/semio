//! ↩️ `change-bearing-d-rd-mm` inverse — restores the pre-change `bearing_d_rd_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_bearing_d_rd_mm::mutation::ChangeBearingDRdMm;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBearingDRdMm, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeBearingDRdMm(ChangeBearingDRdMm { new_bearing_d_rd_mm: base.bearing_d_rd_mm.clone() })]
}
//#endregion 🔖️Inverse
