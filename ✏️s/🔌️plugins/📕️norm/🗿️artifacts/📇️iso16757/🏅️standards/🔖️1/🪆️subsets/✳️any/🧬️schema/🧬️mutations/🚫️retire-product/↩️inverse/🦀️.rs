//! ↩️ `retire-product` — undo re-`create`s the product from BASE state, at its original index;
//! missing id ⇒ `Vec::new()`.

use crate::introduce_product;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

use crate::retire_product::mutation::RetireProduct;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RetireProduct, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::introduce_product::mutation::IntroduceProduct> {
    Ok((|| {
    let Some(position) = base.catalogue.products.iter().position(|product| product.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(introduce_product::mutation::IntroduceProduct { product: base.catalogue.products[position].clone(), index: Some(position) })]

    })())
}
//#endregion 🔖️Inverse
