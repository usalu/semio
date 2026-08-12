//! 🔺️ `delete-product-group` — sparse diff construction.

use super::mutation::DeleteProductGroup;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteProductGroup, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut catalogue = base.catalogue.clone();
    catalogue.product_groups.retain(|group| group.id != payload.id);
    Iso16757Diff { catalogue: Some(catalogue), ..Default::default() }
}
//#endregion 🔖️Diff
