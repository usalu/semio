//! ↩️ `rename-product` — undo restores BASE's title; missing id ⇒ `Vec::new()`.

use crate::rename_product::RenameProduct;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RenameProduct, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::rename_product::RenameProduct> {
    Ok((|| {
    let Some(product) = base.catalog.products.iter().find(|p| p.identity.article_number == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(RenameProduct { id: payload.id.clone(), new_title: product.title.clone() })]

    })())
}
//#endregion 🔖️Inverse
