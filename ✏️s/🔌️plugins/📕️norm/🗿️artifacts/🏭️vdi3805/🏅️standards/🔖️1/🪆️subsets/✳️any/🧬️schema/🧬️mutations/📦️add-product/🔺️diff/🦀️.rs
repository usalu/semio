//! 🔺️ `add-product` — sparse diff construction; keeps `catalog.index` in lockstep with
//! `catalog.products` (see `mutations::catalog_index_entry_for`).

use super::AddProduct;
use crate::mutations::catalog_index_entry_for;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805ProductsRows, Vdi3805IndexEntriesRows};

//#region 🔖️Diff
/// 🔺️ A duplicate article number is `mutation.duplicate-id`; an out-of-range explicit index
/// clamps to the end with `mutation.clamped`.

pub fn diff(payload: &AddProduct, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.catalog.products.iter().any(|p| p.identity.article_number == payload.product.identity.article_number) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A product with article number \"{}\" already exists.", payload.product.identity.article_number), [payload.product.identity.article_number.clone()]);
    }
    let ids: Vec<String> = base.catalog.products.iter().map(|product| product.identity.article_number.clone()).collect();
    let clamped = matches!(payload.index, Some(position) if position > ids.len());
    let at = payload.index.filter(|position| *position <= ids.len()).unwrap_or(ids.len());
    let order = (at < ids.len()).then(|| {
        let mut order = ids.clone();
        order.insert(at, payload.product.identity.article_number.clone());
        order
    });
    let entry = catalog_index_entry_for(&payload.product);
    let mut natural: Vec<String> = base.index.entries.iter().map(|existing| existing.product_id.clone()).collect();
    natural.push(entry.product_id.clone());
    let mut sorted = natural.clone();
    sorted.sort();
    let index_order = (sorted != natural).then_some(sorted);
    let outcome = protocol::MutationOutcome::new(Vdi3805Diff {
        products: Some(Vdi3805ProductsRows { added: vec![payload.product.clone()], order, ..Default::default() }),
        index_entries: Some(Vdi3805IndexEntriesRows { added: vec![entry], order: index_order, ..Default::default() }),
        ..Default::default()
    });
    if clamped {
        outcome.warning("mutation.clamped", format!("Insert index was out of range; appended product \"{}\" at the end instead.", payload.product.identity.article_number))
    } else {
        outcome
    }
}
