//! ↩️ `change-provided-axis-distance-mm` inverse — restores the pre-change `provided_axis_distance_mm` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_provided_axis_distance_mm::ChangeProvidedAxisDistanceMm;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeProvidedAxisDistanceMm, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeProvidedAxisDistanceMm(ChangeProvidedAxisDistanceMm { new_provided_axis_distance_mm: base.provided_axis_distance_mm })]
}
//#endregion 🔖️Inverse
