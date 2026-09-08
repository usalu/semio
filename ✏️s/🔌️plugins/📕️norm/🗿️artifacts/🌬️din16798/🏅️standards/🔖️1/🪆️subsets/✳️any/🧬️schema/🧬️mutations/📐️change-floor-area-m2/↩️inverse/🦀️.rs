//! ↩️ `change-floor-area-m2` inverse — restores the pre-change `floor_area_m2` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_floor_area_m2::ChangeFloorAreaM2;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFloorAreaM2, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeFloorAreaM2(ChangeFloorAreaM2 { new_floor_area_m2: base.floor_area_m2 })]
}
//#endregion 🔖️Inverse
