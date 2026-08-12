//! ↩️ `change-permanent-action` — undo restores BASE's `g_k`; `change` is its own inverse partner
//! (per `📓️taxonomy.md`).

use super::mutation::ChangePermanentAction;
use crate::artifacts::en1990::{En1990Mutation, En1990Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangePermanentAction, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    vec![En1990Mutation::ChangePermanentAction(ChangePermanentAction { new_g_k: base.g_k })]
}
//#endregion 🔖️Inverse
