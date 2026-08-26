//! ↩️ `change-anchor-v-ed-kn` inverse — restores the pre-change `anchor_v_ed_kn` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_anchor_v_ed_kn::mutation::ChangeAnchorVEdKn;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnchorVEdKn, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeAnchorVEdKn(ChangeAnchorVEdKn { new_anchor_v_ed_kn: base.anchor_v_ed_kn.clone() })]
}
//#endregion 🔖️Inverse
