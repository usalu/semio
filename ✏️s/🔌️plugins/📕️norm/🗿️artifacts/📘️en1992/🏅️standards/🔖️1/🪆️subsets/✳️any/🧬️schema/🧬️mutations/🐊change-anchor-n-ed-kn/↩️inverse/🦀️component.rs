//! ↩️ `change-anchor-n-ed-kn` inverse — restores the pre-change `anchor_n_ed_kn` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_anchor_n_ed_kn::mutation::ChangeAnchorNEdKn;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeAnchorNEdKn, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeAnchorNEdKn(ChangeAnchorNEdKn { new_anchor_n_ed_kn: base.anchor_n_ed_kn.clone() })]
}
//#endregion 🔖️Inverse
