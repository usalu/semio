//! ↩️ `change-fy-mpa` — undo restores BASE's f_y_mpa.

use super::ChangeFYMpa;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFYMpa, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeFYMpa(ChangeFYMpa { new_f_y_mpa: base.f_y_mpa })]
}
//#endregion 🔖️Inverse
