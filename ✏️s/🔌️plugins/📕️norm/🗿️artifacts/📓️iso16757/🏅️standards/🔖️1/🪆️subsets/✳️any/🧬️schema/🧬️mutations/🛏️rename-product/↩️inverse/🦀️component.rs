//! ↩️ `rename-product` — undo restores BASE's preferred name; missing id ⇒ `Vec::new()`.

use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::RenameProduct;

//#region 🔖️Inverse
pub async fn inverse(payload: &RenameProduct, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    let Some(product) = base.catalogue.products.iter().find(|product| product.id == payload.id) else {
        return Vec::new();
    };
    vec![Iso16757Mutation::RenameProduct(RenameProduct { id: payload.id.clone(), new_name: product.names.preferred.text.clone() })]
}
//#endregion 🔖️Inverse
