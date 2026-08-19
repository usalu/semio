//! ↩️ `change-f-ck-mpa` — undo restores BASE's f_ck_mpa.

use super::mutation::ChangeFCkMpa;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeFCkMpa, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeFCkMpa(ChangeFCkMpa { new_f_ck_mpa: base.f_ck_mpa })]
}
//#endregion 🔖️Inverse
