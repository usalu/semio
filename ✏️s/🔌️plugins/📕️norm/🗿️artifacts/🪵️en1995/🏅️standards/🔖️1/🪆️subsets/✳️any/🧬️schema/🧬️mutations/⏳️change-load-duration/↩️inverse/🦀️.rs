//! ↩️ `change-load-duration` inverse — restores the pre-change `load_duration` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_load_duration::ChangeLoadDuration;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeLoadDuration, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeLoadDuration(ChangeLoadDuration { new_load_duration: base.load_duration.clone() })]
}
//#endregion 🔖️Inverse
