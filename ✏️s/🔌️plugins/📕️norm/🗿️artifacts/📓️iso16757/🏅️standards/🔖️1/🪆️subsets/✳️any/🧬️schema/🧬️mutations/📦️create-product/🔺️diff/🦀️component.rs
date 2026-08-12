//! 🔺️ `create-product` — sparse diff construction.

use super::mutation::CreateProduct;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is a no-op — an id-keyed entity that already exists cannot be "created"
/// again; the catalogue clone is returned unchanged rather than pushing a second product.
pub fn diff(payload: &CreateProduct, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut catalogue = base.catalogue.clone();
    if !catalogue.products.iter().any(|product| product.id == payload.product.id) {
        match payload.index {
            Some(index) if index <= catalogue.products.len() => catalogue.products.insert(index, payload.product.clone()),
            _ => catalogue.products.push(payload.product.clone()),
        }
    }
    Iso16757Diff { catalogue: Some(catalogue), ..Default::default() }
}
//#endregion 🔖️Diff
