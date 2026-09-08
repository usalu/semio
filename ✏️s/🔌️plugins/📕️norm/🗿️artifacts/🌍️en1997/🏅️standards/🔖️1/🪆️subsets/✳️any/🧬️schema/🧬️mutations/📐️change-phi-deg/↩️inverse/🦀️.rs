//! ↩️ `change-phi-deg` inverse — restores the pre-change `phi_deg` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_phi_deg::ChangePhiDeg;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangePhiDeg, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangePhiDeg(ChangePhiDeg { new_phi_deg: base.phi_deg })]
}
//#endregion 🔖️Inverse
