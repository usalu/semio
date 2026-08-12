//! ↩️ `change-provided-axis-distance-mm` inverse — restores the pre-change `provided_axis_distance_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_provided_axis_distance_mm::mutation::ChangeProvidedAxisDistanceMm;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeProvidedAxisDistanceMm, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeProvidedAxisDistanceMm(ChangeProvidedAxisDistanceMm { new_provided_axis_distance_mm: base.provided_axis_distance_mm.clone() })]
}
//#endregion 🔖️Inverse
