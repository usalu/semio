//! ↩️ `create-product` — undo is `delete-product`, unless `base` already had this id (then
//! `create` was a no-op and there's nothing to undo).

use crate::artifacts::iso16757::mutations::delete_product;
use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::CreateProduct;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateProduct, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    if base.catalogue.products.iter().any(|product| product.id == payload.product.id) {
        return Vec::new();
    }
    vec![Iso16757Mutation::DeleteProduct(delete_product::mutation::DeleteProduct { id: payload.product.id.clone() })]
}
//#endregion 🔖️Inverse
