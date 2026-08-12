//! 🔺️ `create-product` — sparse diff construction; keeps `catalog.index` in lockstep with
//! `catalog.products` (see `mutations::catalog_index_entry_for`).

use super::mutation::CreateProduct;
use crate::artifacts::vdi3805::mutations::catalog_index_entry_for;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate article number is a no-op — an id-keyed entity that already exists cannot be
/// "created" again; the catalog/index clones are returned unchanged rather than duplicating rows.
pub fn diff(payload: &CreateProduct, base: &Vdi3805Snapshot) -> Vdi3805Diff {
    let mut catalog = base.catalog.clone();
    let mut index = base.index.clone();
    if !catalog.products.iter().any(|p| p.identity.article_number == payload.product.identity.article_number) {
        match payload.index {
            Some(position) if position <= catalog.products.len() => catalog.products.insert(position, payload.product.clone()),
            _ => catalog.products.push(payload.product.clone()),
        }
        index.entries.push(catalog_index_entry_for(&payload.product));
    }
    Vdi3805Diff { catalog: Some(catalog), index: Some(index), ..Default::default() }
}
//#endregion 🔖️Diff
