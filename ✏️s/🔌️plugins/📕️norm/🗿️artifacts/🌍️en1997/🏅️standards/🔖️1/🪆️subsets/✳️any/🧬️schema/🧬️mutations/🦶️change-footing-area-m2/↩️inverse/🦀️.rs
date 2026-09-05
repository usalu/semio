//! ↩️ `change-footing-area-m2` inverse — restores the pre-change `footing_area_m2` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_footing_area_m2::ChangeFootingAreaM2;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFootingAreaM2, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeFootingAreaM2(ChangeFootingAreaM2 { new_footing_area_m2: base.footing_area_m2.clone() })]
}
//#endregion 🔖️Inverse
