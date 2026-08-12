//! ↩️ `change-e-cm-mpa` — undo restores BASE's e_cm_mpa.

use super::mutation::ChangeECmMpa;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeECmMpa, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeECmMpa(ChangeECmMpa { new_e_cm_mpa: base.e_cm_mpa })]
}
//#endregion 🔖️Inverse
