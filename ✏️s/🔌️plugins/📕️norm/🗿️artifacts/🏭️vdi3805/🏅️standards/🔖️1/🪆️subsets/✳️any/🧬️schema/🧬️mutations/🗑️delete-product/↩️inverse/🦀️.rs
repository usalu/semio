//! ↩️ `delete-product` — undo re-`create`s the product from BASE state, at its original index;
//! missing article number ⇒ `Vec::new()`.

use super::DeleteProduct;
use crate::artifacts::vdi3805::mutations::create_product;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteProduct, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    let Some(position) = base.catalog.products.iter().position(|p| p.identity.article_number == payload.id) else {
        return Vec::new();
    };
    vec![Vdi3805Mutation::CreateProduct(create_product::CreateProduct { product: base.catalog.products[position].clone(), index: Some(position) })]
}
//#endregion 🔖️Inverse
