//! ↩️ `change-alloy` inverse — restores the pre-change `alloy` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_alloy::ChangeAlloy;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAlloy, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeAlloy(ChangeAlloy { new_alloy: base.alloy.clone() })]
}
//#endregion 🔖️Inverse
