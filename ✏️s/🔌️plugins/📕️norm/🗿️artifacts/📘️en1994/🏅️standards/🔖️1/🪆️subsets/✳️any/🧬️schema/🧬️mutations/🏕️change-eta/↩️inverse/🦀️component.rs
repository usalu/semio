//! ↩️ `change-eta` — undo restores BASE's eta.

use super::mutation::ChangeEta;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeEta, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeEta(ChangeEta { new_eta: base.eta })]
}
//#endregion 🔖️Inverse
