//! ↩️ `create-product` — undo is `delete-product`, unless `base` already had this article number
//! (then `create` was a no-op and there's nothing to undo).

use super::CreateProduct;
use crate::artifacts::vdi3805::mutations::delete_product;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &CreateProduct, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    if base.catalog.products.iter().any(|p| p.identity.article_number == payload.product.identity.article_number) {
        return Vec::new();
    }
    vec![Vdi3805Mutation::DeleteProduct(delete_product::DeleteProduct { id: payload.product.identity.article_number.clone() })]
}
//#endregion 🔖️Inverse
