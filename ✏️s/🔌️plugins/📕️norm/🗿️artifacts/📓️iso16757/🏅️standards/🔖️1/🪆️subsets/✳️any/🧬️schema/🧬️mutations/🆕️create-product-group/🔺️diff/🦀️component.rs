//! 🔺️ `create-product-group` — sparse diff construction.

use super::mutation::CreateProductGroup;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is a no-op — an id-keyed entity that already exists cannot be "created"
/// again; the catalogue clone is returned unchanged rather than pushing a second group.
pub fn diff(payload: &CreateProductGroup, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut catalogue = base.catalogue.clone();
    if !catalogue.product_groups.iter().any(|group| group.id == payload.product_group.id) {
        match payload.index {
            Some(index) if index <= catalogue.product_groups.len() => catalogue.product_groups.insert(index, payload.product_group.clone()),
            _ => catalogue.product_groups.push(payload.product_group.clone()),
        }
    }
    Iso16757Diff { catalogue: Some(catalogue), ..Default::default() }
}
//#endregion 🔖️Diff
