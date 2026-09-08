//! ↩️ `change-bearing-d-rd-mm` inverse — restores the pre-change `bearing_d_rd_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_bearing_d_rd_mm::ChangeBearingDRdMm;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBearingDRdMm, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeBearingDRdMm(ChangeBearingDRdMm { new_bearing_d_rd_mm: base.bearing_d_rd_mm })]
}
//#endregion 🔖️Inverse
