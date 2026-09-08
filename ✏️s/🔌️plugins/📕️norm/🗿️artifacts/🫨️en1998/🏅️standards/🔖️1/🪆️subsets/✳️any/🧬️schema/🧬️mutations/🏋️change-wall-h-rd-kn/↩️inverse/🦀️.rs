//! ↩️ `change-wall-h-rd-kn` inverse — restores the pre-change `wall_h_rd_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_wall_h_rd_kn::ChangeWallHRdKn;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeWallHRdKn, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeWallHRdKn(ChangeWallHRdKn { new_wall_h_rd_kn: base.wall_h_rd_kn })]
}
//#endregion 🔖️Inverse
