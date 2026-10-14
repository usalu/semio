//! ↩️ `add-product` — undo is `remove-product`, unless `base` already had this article number
//! (then `create` was a no-op and there's nothing to undo).

use crate::add_product::AddProduct;
use crate::remove_product;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &AddProduct, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_product::RemoveProduct> {
    Ok((|| {
    if base.catalog.products.iter().any(|p| p.identity.article_number == payload.product.identity.article_number) {
        return Vec::new();
    }
    vec![Op::from(remove_product::RemoveProduct { id: payload.product.identity.article_number.clone() })]

    })())
}
//#endregion 🔖️Inverse
