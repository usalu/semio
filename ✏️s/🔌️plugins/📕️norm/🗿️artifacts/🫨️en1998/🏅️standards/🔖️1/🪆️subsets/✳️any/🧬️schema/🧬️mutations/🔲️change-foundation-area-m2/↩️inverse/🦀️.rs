//! ↩️ `change-foundation-area-m2` inverse — restores the pre-change `foundation_area_m2` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_foundation_area_m2::ChangeFoundationAreaM2;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFoundationAreaM2, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeFoundationAreaM2(ChangeFoundationAreaM2 { new_foundation_area_m2: base.foundation_area_m2.clone() })]
}
//#endregion 🔖️Inverse
