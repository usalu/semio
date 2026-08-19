//! ↩️ `change-wall-h-rd-kn` inverse — restores the pre-change `wall_h_rd_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_wall_h_rd_kn::mutation::ChangeWallHRdKn;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeWallHRdKn, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeWallHRdKn(ChangeWallHRdKn { new_wall_h_rd_kn: base.wall_h_rd_kn.clone() })]
}
//#endregion 🔖️Inverse
