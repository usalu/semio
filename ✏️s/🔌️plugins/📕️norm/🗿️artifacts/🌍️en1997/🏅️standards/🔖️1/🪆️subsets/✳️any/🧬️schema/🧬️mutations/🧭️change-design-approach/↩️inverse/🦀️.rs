//! ↩️ `change-design-approach` inverse — restores the pre-change `design_approach` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_design_approach::ChangeDesignApproach;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDesignApproach, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeDesignApproach(ChangeDesignApproach { new_design_approach: base.design_approach.clone() })]
}
//#endregion 🔖️Inverse
