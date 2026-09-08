//! ↩️ `change-anchor-f-yk-mpa` inverse — restores the pre-change `anchor_f_yk_mpa` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_anchor_f_yk_mpa::ChangeAnchorFYkMpa;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnchorFYkMpa, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeAnchorFYkMpa(ChangeAnchorFYkMpa { new_anchor_f_yk_mpa: base.anchor_f_yk_mpa })]
}
//#endregion 🔖️Inverse
