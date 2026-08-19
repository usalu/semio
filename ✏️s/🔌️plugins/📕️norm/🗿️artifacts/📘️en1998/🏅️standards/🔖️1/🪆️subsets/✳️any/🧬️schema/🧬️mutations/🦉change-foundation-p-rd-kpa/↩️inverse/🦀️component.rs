//! ↩️ `change-foundation-p-rd-kpa` inverse — restores the pre-change `foundation_p_rd_kpa` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_foundation_p_rd_kpa::mutation::ChangeFoundationPRdKpa;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeFoundationPRdKpa, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeFoundationPRdKpa(ChangeFoundationPRdKpa { new_foundation_p_rd_kpa: base.foundation_p_rd_kpa.clone() })]
}
//#endregion 🔖️Inverse
