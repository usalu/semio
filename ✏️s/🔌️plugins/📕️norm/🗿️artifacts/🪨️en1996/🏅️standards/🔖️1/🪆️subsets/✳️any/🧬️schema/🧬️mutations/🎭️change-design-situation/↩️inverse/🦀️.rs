//! ↩️ `change-design-situation` inverse — restores the pre-change `design_situation` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_design_situation::ChangeDesignSituation;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDesignSituation, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeDesignSituation(ChangeDesignSituation { new_design_situation: base.design_situation.clone() })]
}
//#endregion 🔖️Inverse
