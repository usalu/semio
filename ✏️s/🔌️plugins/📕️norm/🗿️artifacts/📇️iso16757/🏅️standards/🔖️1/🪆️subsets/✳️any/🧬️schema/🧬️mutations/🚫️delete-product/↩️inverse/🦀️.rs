//! ↩️ `delete-product` — undo re-`create`s the product from BASE state, at its original index;
//! missing id ⇒ `Vec::new()`.

use crate::artifacts::iso16757::mutations::create_product;
use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::DeleteProduct;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteProduct, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    let Some(position) = base.catalogue.products.iter().position(|product| product.id == payload.id) else {
        return Vec::new();
    };
    vec![Iso16757Mutation::CreateProduct(create_product::mutation::CreateProduct { product: base.catalogue.products[position].clone(), index: Some(position) })]
}
//#endregion 🔖️Inverse
