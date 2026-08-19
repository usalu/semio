//! ↩️ `change-annex` — undo restores BASE's annex.

use super::mutation::ChangeAnnex;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeAnnex, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex })]
}
//#endregion 🔖️Inverse
