//! ↩️ `remove-product` — undo re-`create`s the product from BASE state, at its original index;
//! missing article number ⇒ `Vec::new()`.

use crate::remove_product::RemoveProduct;
use crate::add_product;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RemoveProduct, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::add_product::AddProduct> {
    Ok((|| {
    let Some(position) = base.catalog.products.iter().position(|p| p.identity.article_number == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(add_product::AddProduct { product: base.catalog.products[position].clone(), index: Some(position) })]

    })())
}
//#endregion 🔖️Inverse
