//! 🔺️ `rename-product` — sparse diff construction; missing id is a no-op clone. Keeps the
//! `catalog.index` entry's display tags in lockstep with the new title.

use super::mutation::RenameProduct;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RenameProduct, base: &Vdi3805Snapshot) -> Vdi3805Diff {
    let mut catalog = base.catalog.clone();
    if let Some(product) = catalog.products.iter_mut().find(|p| p.identity.article_number == payload.id) {
        product.title = payload.new_title.clone();
    }
    let mut index = base.index.clone();
    if let Some(entry) = index.entries.iter_mut().find(|entry| entry.product_id == payload.id) {
        entry.tags = vec![payload.new_title.de.clone(), payload.new_title.en.clone()];
    }
    Vdi3805Diff { catalog: Some(catalog), index: Some(index), ..Default::default() }
}
//#endregion 🔖️Diff
