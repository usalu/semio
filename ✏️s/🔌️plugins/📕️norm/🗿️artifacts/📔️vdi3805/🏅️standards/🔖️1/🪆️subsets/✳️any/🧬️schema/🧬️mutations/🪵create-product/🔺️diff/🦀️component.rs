//! 🔺️ `create-product` — sparse diff construction; keeps `catalog.index` in lockstep with
//! `catalog.products` (see `mutations::catalog_index_entry_for`).

use super::mutation::CreateProduct;
use crate::artifacts::vdi3805::mutations::catalog_index_entry_for;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate article number is `mutation.duplicate-id`; an out-of-range explicit index
/// clamps to the end with `mutation.clamped`.
pub async fn diff(payload: &CreateProduct, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.catalog.products.iter().any(|p| p.identity.article_number == payload.product.identity.article_number) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A product with article number \"{}\" already exists.", payload.product.identity.article_number), [payload.product.identity.article_number.clone()]);
    }
    let mut catalog = base.catalog.clone();
    let mut index = base.index.clone();
    let clamped = matches!(payload.index, Some(position) if position > catalog.products.len());
    match payload.index {
        Some(position) if position <= catalog.products.len() => catalog.products.insert(position, payload.product.clone()),
        _ => catalog.products.push(payload.product.clone()),
    }
    index.entries.push(catalog_index_entry_for(&payload.product));
    let outcome = protocol::MutationOutcome::new(Vdi3805Diff { catalog: Some(catalog), index: Some(index), ..Default::default() });
    if clamped {
        outcome.warn("mutation.clamped", format!("Insert index was out of range; appended product \"{}\" at the end instead.", payload.product.identity.article_number))
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
