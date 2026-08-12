//! ↩️ `change-annex` — undo restores BASE's annex.

use super::mutation::ChangeAnnex;
use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnnex, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::ChangeAnnex(ChangeAnnex {
        new_annex: base.annex,
    })]
}
//#endregion 🔖️Inverse
