//! ↩️ `change-fu-mpa` — undo restores BASE's f_u_mpa.

use super::mutation::ChangeFUMpa;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeFUMpa, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeFUMpa(ChangeFUMpa { new_f_u_mpa: base.f_u_mpa })]
}
//#endregion 🔖️Inverse
