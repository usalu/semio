//! ↩️ `change-cd` — undo restores BASE's dynamic factor c_d.

use super::ChangeCD;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeCD, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeCD(ChangeCD { new_c_d: base.c_d.clone() })]
}
//#endregion 🔖️Inverse
