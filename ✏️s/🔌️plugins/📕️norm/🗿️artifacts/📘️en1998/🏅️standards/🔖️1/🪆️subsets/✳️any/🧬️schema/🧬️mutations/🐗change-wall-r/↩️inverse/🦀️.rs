//! ↩️ `change-wall-r` inverse — restores the pre-change `wall_r` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_wall_r::ChangeWallR;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeWallR, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeWallR(ChangeWallR { new_wall_r: base.wall_r.clone() })]
}
//#endregion 🔖️Inverse
