//! ↩️ `change-design-situation` inverse — restores the pre-change `design_situation` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_design_situation::ChangeDesignSituation;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDesignSituation, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeDesignSituation(ChangeDesignSituation { new_design_situation: base.design_situation })]
}
//#endregion 🔖️Inverse
