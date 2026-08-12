//! 🔺️ `rename-product-group` — sparse diff construction; missing id is a no-op clone.

use super::mutation::RenameProductGroup;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RenameProductGroup, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut catalogue = base.catalogue.clone();
    if let Some(group) = catalogue.product_groups.iter_mut().find(|group| group.id == payload.id) {
        group.names.preferred.text = payload.new_name.clone();
    }
    Iso16757Diff { catalogue: Some(catalogue), ..Default::default() }
}
//#endregion 🔖️Diff
