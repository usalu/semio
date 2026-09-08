//! ↩️ `change-wall-phi-deg` inverse — restores the pre-change `wall_phi_deg` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_wall_phi_deg::ChangeWallPhiDeg;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeWallPhiDeg, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeWallPhiDeg(ChangeWallPhiDeg { new_wall_phi_deg: base.wall_phi_deg })]
}
//#endregion 🔖️Inverse
