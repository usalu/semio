//! ↩️ `change-k-foundation` inverse — restores the pre-change `k_foundation` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_k_foundation::ChangeKFoundation;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeKFoundation, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeKFoundation(ChangeKFoundation { new_k_foundation: base.k_foundation.clone() })]
}
//#endregion 🔖️Inverse
