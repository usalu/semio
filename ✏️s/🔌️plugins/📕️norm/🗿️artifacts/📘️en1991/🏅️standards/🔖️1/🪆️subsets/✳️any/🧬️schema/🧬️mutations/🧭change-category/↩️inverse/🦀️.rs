//! ↩️ `change-category` — undo restores BASE's imposed category.

use super::ChangeCategory;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeCategory, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeCategory(ChangeCategory { new_category: base.category.clone() })]
}
//#endregion 🔖️Inverse
