//! 🔺️ `change-category` — sparse diff construction.

use super::mutation::ChangeCategory;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeCategory, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { category: Some(payload.new_category.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
