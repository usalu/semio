//! 🔺️ `change-category` — sparse diff construction.

use super::mutation::ChangeCategory;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeCategory, _base: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { category: Some(payload.new_category.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
