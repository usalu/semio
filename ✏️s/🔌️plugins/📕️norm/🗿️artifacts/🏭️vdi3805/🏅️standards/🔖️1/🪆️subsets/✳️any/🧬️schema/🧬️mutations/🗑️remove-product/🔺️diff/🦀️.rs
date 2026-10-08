//! 🔺️ `remove-product` — sparse diff construction; keeps `catalog.index` in lockstep.

use super::RemoveProduct;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805ProductsRows, Vdi3805IndexEntriesRows};

//#region 🔖️Diff

pub fn diff(payload: &RemoveProduct, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if !base.catalog.products.iter().any(|p| p.identity.article_number == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let index_entries = base.index.entries.iter().any(|entry| entry.product_id == payload.id).then(|| Vdi3805IndexEntriesRows { removed: vec![payload.id.clone()], ..Default::default() });
    protocol::MutationOutcome::new(Vdi3805Diff {
        products: Some(Vdi3805ProductsRows { removed: vec![payload.id.clone()], ..Default::default() }),
        index_entries,
        ..Default::default()
    })
}
