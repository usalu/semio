//! ↩️ `change-drift-mm` inverse — restores the pre-change `drift_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_drift_mm::ChangeDriftMm;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDriftMm, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeDriftMm(ChangeDriftMm { new_drift_mm: base.drift_mm })]
}
//#endregion 🔖️Inverse
