//! ↩️ `change-z-investigated-m` inverse — restores the pre-change `z_investigated_m` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_z_investigated_m::mutation::ChangeZInvestigatedM;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeZInvestigatedM, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeZInvestigatedM(ChangeZInvestigatedM { new_z_investigated_m: base.z_investigated_m.clone() })]
}
//#endregion 🔖️Inverse
