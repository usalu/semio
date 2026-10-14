//! ↩️ `rename-product` — undo restores BASE's preferred name; missing id ⇒ `Vec::new()`.

use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

use crate::rename_product::mutation::RenameProduct;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RenameProduct, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::rename_product::mutation::RenameProduct> {
    Ok((|| {
    let Some(product) = base.catalogue.products.iter().find(|product| product.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(RenameProduct { id: payload.id.clone(), new_name: product.names.preferred.text.clone() })]

    })())
}
//#endregion 🔖️Inverse
