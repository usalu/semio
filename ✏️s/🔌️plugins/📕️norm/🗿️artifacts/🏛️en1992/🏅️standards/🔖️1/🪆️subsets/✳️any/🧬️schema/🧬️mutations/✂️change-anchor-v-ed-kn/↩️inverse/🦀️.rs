//! ↩️ `change-anchor-v-ed-kn` inverse — restores the pre-change `anchor_v_ed_kn` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_anchor_v_ed_kn::ChangeAnchorVEdKn;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnchorVEdKn, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeAnchorVEdKn(ChangeAnchorVEdKn { new_anchor_v_ed_kn: base.anchor_v_ed_kn })]
}
//#endregion 🔖️Inverse
