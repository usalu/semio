//! ↩️ `add-product` — undo is `remove-product`, unless `base` already had this article number
//! (then `create` was a no-op and there's nothing to undo).

use super::AddProduct;
use crate::mutations::remove_product;
use crate::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &AddProduct, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    if base.catalog.products.iter().any(|p| p.identity.article_number == payload.product.identity.article_number) {
        return Vec::new();
    }
    vec![Vdi3805Mutation::RemoveProduct(remove_product::RemoveProduct { id: payload.product.identity.article_number.clone() })]
}
//#endregion 🔖️Inverse
