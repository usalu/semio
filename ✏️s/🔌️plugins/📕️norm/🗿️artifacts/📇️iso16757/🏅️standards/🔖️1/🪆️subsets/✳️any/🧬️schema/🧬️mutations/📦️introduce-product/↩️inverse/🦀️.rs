//! ↩️ `introduce-product` — undo is `retire-product`, unless `base` already had this id (then
//! `create` was a no-op and there's nothing to undo).

use crate::retire_product;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

use crate::introduce_product::mutation::IntroduceProduct;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &IntroduceProduct, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::retire_product::mutation::RetireProduct> {
    Ok((|| {
    if base.catalogue.products.iter().any(|product| product.id == payload.product.id) {
        return Vec::new();
    }
    vec![Op::from(retire_product::mutation::RetireProduct { id: payload.product.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
