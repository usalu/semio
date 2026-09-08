//! ↩️ `change-cellar-area-m2` inverse — restores the pre-change `cellar_area_m2` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_cellar_area_m2::ChangeCellarAreaM2;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeCellarAreaM2, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeCellarAreaM2(ChangeCellarAreaM2 { new_cellar_area_m2: base.cellar_area_m2 })]
}
//#endregion 🔖️Inverse
