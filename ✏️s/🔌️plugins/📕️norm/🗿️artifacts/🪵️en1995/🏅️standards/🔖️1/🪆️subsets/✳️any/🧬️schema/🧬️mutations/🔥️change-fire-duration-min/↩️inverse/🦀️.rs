//! ↩️ `change-fire-duration-min` inverse — restores the pre-change `fire_duration_min` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::change_fire_duration_min::ChangeFireDurationMin;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFireDurationMin, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeFireDurationMin(ChangeFireDurationMin { new_fire_duration_min: base.fire_duration_min.clone() })]
}
//#endregion 🔖️Inverse
