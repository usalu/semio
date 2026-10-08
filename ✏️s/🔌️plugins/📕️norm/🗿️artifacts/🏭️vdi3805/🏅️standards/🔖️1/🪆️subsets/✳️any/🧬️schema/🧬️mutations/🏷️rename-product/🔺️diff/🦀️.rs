//! 🔺️ `rename-product` — sparse diff construction; missing id is `mutation.target-missing`. Keeps
//! the `catalog.index` entry's display tags in lockstep with the new title.

use super::RenameProduct;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805ProductsRows, Vdi3805ProductsPatch, Vdi3805IndexEntriesRows, Vdi3805IndexEntriesPatch};

//#region 🔖️Diff

pub fn diff(payload: &RenameProduct, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    let Some(product) = base.catalog.products.iter().find(|p| p.identity.article_number == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if product.title == payload.new_title {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Product \"{}\" already has that title.", payload.id));
    }
    let tags: Vec<String> = payload.new_title.iter().map(|title| title.text.clone()).collect();
    let index_entries = base.index.entries.iter().find(|entry| entry.product_id == payload.id).filter(|entry| entry.tags != tags).map(|_| Vdi3805IndexEntriesRows {
        modified: vec![Vdi3805IndexEntriesPatch { id: payload.id.clone(), tags: Some(tags), ..Default::default() }],
        ..Default::default()
    });
    protocol::MutationOutcome::new(Vdi3805Diff {
        products: Some(Vdi3805ProductsRows { modified: vec![Vdi3805ProductsPatch { id: payload.id.clone(), title: Some(payload.new_title.clone()), ..Default::default() }], ..Default::default() }),
        index_entries,
        ..Default::default()
    })
}
