//! ↩️ `change-wall-height-m` inverse — restores the pre-change `wall_height_m` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_wall_height_m::ChangeWallHeightM;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeWallHeightM, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeWallHeightM(ChangeWallHeightM { new_wall_height_m: base.wall_height_m.clone() })]
}
//#endregion 🔖️Inverse
