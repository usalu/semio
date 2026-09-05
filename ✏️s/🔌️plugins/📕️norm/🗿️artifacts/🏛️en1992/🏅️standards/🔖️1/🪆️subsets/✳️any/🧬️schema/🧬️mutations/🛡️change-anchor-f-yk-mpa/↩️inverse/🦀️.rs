//! ↩️ `change-anchor-f-yk-mpa` inverse — restores the pre-change `anchor_f_yk_mpa` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_anchor_f_yk_mpa::ChangeAnchorFYkMpa;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnchorFYkMpa, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeAnchorFYkMpa(ChangeAnchorFYkMpa { new_anchor_f_yk_mpa: base.anchor_f_yk_mpa.clone() })]
}
//#endregion 🔖️Inverse
