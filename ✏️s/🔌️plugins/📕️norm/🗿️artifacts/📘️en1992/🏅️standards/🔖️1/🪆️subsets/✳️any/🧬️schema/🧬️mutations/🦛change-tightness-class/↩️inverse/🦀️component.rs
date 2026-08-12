//! ↩️ `change-tightness-class` inverse — restores the pre-change `tightness_class` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1992::mutations::change_tightness_class::mutation::ChangeTightnessClass;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeTightnessClass, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeTightnessClass(ChangeTightnessClass { new_tightness_class: base.tightness_class.clone() })]
}
//#endregion 🔖️Inverse
