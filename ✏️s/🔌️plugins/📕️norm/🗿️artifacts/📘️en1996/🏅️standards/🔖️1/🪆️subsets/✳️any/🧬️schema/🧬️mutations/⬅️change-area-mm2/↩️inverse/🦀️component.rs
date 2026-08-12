//! ↩️ `change-area-mm2` inverse — restores the pre-change `area_mm2` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_area_mm2::mutation::ChangeAreaMm2;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAreaMm2, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeAreaMm2(ChangeAreaMm2 { new_area_mm2: base.area_mm2.clone() })]
}
//#endregion 🔖️Inverse
