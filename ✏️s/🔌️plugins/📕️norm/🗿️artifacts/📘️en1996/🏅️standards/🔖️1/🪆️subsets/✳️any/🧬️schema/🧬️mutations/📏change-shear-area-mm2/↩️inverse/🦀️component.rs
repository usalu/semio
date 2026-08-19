//! ↩️ `change-shear-area-mm2` inverse — restores the pre-change `shear_area_mm2` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_shear_area_mm2::mutation::ChangeShearAreaMm2;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeShearAreaMm2, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeShearAreaMm2(ChangeShearAreaMm2 { new_shear_area_mm2: base.shear_area_mm2.clone() })]
}
//#endregion 🔖️Inverse
