//! ↩️ `change-tightness-class` inverse — restores the pre-change `tightness_class` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_tightness_class::ChangeTightnessClass;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeTightnessClass, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeTightnessClass(ChangeTightnessClass { new_tightness_class: base.tightness_class })]
}
//#endregion 🔖️Inverse
