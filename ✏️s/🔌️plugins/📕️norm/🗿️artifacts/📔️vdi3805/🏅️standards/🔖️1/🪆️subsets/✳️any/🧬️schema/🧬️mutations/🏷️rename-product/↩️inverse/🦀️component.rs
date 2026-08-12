//! ↩️ `rename-product` — undo restores BASE's title; missing id ⇒ `Vec::new()`.

use super::mutation::RenameProduct;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &RenameProduct, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    let Some(product) = base.catalog.products.iter().find(|p| p.identity.article_number == payload.id) else {
        return Vec::new();
    };
    vec![Vdi3805Mutation::RenameProduct(RenameProduct { id: payload.id.clone(), new_title: product.title.clone() })]
}
//#endregion 🔖️Inverse
