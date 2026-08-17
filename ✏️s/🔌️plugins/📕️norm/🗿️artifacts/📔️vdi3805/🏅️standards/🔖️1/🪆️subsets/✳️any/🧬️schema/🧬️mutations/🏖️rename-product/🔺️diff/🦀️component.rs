//! 🔺️ `rename-product` — sparse diff construction; missing id is `mutation.target-missing`. Keeps
//! the `catalog.index` entry's display tags in lockstep with the new title.

use super::mutation::RenameProduct;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RenameProduct, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    let Some(product) = base.catalog.products.iter().find(|p| p.identity.article_number == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if product.title == payload.new_title {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Product \"{}\" already has that title.", payload.id));
    }
    let mut catalog = base.catalog.clone();
    if let Some(product) = catalog.products.iter_mut().find(|p| p.identity.article_number == payload.id) {
        product.title = payload.new_title.clone();
    }
    let mut index = base.index.clone();
    if let Some(entry) = index.entries.iter_mut().find(|entry| entry.product_id == payload.id) {
        entry.tags = payload.new_title.iter().map(|t| t.text.clone()).collect();
    }
    protocol::MutationOutcome::new(Vdi3805Diff { catalog: Some(catalog), index: Some(index), ..Default::default() })
}
//#endregion 🔖️Diff
