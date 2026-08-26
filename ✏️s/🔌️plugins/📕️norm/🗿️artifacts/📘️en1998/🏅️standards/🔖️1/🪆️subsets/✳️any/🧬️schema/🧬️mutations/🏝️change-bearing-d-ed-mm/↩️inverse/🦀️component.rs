//! ↩️ `change-bearing-d-ed-mm` inverse — restores the pre-change `bearing_d_ed_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_bearing_d_ed_mm::mutation::ChangeBearingDEdMm;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBearingDEdMm, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeBearingDEdMm(ChangeBearingDEdMm { new_bearing_d_ed_mm: base.bearing_d_ed_mm.clone() })]
}
//#endregion 🔖️Inverse
