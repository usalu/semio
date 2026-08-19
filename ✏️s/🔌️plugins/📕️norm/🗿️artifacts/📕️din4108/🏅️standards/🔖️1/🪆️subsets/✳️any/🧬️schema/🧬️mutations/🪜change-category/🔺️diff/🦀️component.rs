//! 🔺️ `change-category` — sparse diff construction.

use super::mutation::ChangeCategory;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeCategory, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.category == payload.new_category {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Category already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { category: Some(payload.new_category.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
