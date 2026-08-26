//! ↩️ `change-wall-thickness-mm` inverse — restores the pre-change `wall_thickness_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_wall_thickness_mm::mutation::ChangeWallThicknessMm;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeWallThicknessMm, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeWallThicknessMm(ChangeWallThicknessMm { new_wall_thickness_mm: base.wall_thickness_mm.clone() })]
}
//#endregion 🔖️Inverse
