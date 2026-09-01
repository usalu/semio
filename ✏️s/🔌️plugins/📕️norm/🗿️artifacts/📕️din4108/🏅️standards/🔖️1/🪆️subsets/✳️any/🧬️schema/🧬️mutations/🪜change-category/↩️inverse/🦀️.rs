//! ↩️ `change-category` — undo restores BASE's `category`.

use super::ChangeCategory;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeCategory, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeCategory(ChangeCategory { new_category: base.category.clone() })]
}
//#endregion 🔖️Inverse
