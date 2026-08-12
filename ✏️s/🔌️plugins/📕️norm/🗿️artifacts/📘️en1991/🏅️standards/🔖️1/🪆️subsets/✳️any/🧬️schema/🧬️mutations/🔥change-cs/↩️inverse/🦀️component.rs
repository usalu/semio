//! ↩️ `change-cs` — undo restores BASE's size factor c_s.

use super::mutation::ChangeCS;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeCS, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeCS(ChangeCS { new_c_s: base.c_s.clone() })]
}
//#endregion 🔖️Inverse
