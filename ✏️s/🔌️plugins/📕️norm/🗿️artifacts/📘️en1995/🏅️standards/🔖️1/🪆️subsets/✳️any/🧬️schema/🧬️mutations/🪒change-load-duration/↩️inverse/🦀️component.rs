//! ↩️ `change-load-duration` inverse — restores the pre-change `load_duration` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::change_load_duration::mutation::ChangeLoadDuration;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeLoadDuration, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeLoadDuration(ChangeLoadDuration { new_load_duration: base.load_duration.clone() })]
}
//#endregion 🔖️Inverse
