//! ↩️ `change-anchor-cracked` inverse — restores the pre-change `anchor_cracked` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_anchor_cracked::ChangeAnchorCracked;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnchorCracked, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeAnchorCracked(ChangeAnchorCracked { new_anchor_cracked: base.anchor_cracked })]
}
//#endregion 🔖️Inverse
