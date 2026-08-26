//! ↩️ `change-anchor-f-uk-mpa` inverse — restores the pre-change `anchor_f_uk_mpa` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_anchor_f_uk_mpa::mutation::ChangeAnchorFUkMpa;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnchorFUkMpa, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeAnchorFUkMpa(ChangeAnchorFUkMpa { new_anchor_f_uk_mpa: base.anchor_f_uk_mpa.clone() })]
}
//#endregion 🔖️Inverse
