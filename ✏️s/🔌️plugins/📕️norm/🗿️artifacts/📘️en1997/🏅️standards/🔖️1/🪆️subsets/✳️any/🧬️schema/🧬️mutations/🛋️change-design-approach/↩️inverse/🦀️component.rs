//! ↩️ `change-design-approach` inverse — restores the pre-change `design_approach` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_design_approach::mutation::ChangeDesignApproach;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeDesignApproach, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeDesignApproach(ChangeDesignApproach { new_design_approach: base.design_approach.clone() })]
}
//#endregion 🔖️Inverse
