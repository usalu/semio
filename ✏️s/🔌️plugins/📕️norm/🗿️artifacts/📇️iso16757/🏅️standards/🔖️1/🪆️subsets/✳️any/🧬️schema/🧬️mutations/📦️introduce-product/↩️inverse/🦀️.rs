//! ↩️ `introduce-product` — undo is `retire-product`, unless `base` already had this id (then
//! `create` was a no-op and there's nothing to undo).

use crate::mutations::retire_product;
use crate::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::IntroduceProduct;

//#region 🔖️Inverse
pub fn inverse(payload: &IntroduceProduct, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    if base.catalogue.products.iter().any(|product| product.id == payload.product.id) {
        return Vec::new();
    }
    vec![Iso16757Mutation::RetireProduct(retire_product::mutation::RetireProduct { id: payload.product.id.clone() })]
}
//#endregion 🔖️Inverse
