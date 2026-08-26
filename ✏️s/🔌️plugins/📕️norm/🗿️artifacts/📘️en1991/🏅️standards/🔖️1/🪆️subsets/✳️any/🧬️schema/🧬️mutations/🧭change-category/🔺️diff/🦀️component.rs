//! 🔺️ `change-category` — sparse diff construction.

use super::mutation::ChangeCategory;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeCategory, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.category == payload.new_category {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Category already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { category: Some(payload.new_category.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
