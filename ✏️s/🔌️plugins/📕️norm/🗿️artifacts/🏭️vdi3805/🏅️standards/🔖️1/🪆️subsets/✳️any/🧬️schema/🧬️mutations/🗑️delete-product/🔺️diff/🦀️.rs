//! 🔺️ `delete-product` — sparse diff construction; keeps `catalog.index` in lockstep.

use super::DeleteProduct;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteProduct, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if !base.catalog.products.iter().any(|p| p.identity.article_number == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let mut catalog = base.catalog.clone();
    catalog.products.retain(|p| p.identity.article_number != payload.id);
    let mut index = base.index.clone();
    index.entries.retain(|entry| entry.product_id != payload.id);
    protocol::MutationOutcome::new(Vdi3805Diff { catalog: Some(catalog), index: Some(index), ..Default::default() })
}
//#endregion 🔖️Diff
