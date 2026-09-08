//! ↩️ `change-z-investigated-m` inverse — restores the pre-change `z_investigated_m` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_z_investigated_m::ChangeZInvestigatedM;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeZInvestigatedM, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeZInvestigatedM(ChangeZInvestigatedM { new_z_investigated_m: base.z_investigated_m })]
}
//#endregion 🔖️Inverse
