//! ↩️ `change-material-product-form` inverse: restores the previous `product_form`.

use crate::change_material_product_form::ChangeMaterialProductForm;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeMaterialProductForm, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_material_product_form::ChangeMaterialProductForm> {
    Ok(match base.materials.iter().find(|row| row.id == payload.material_id) {
        Some(row) => vec![Op::from(ChangeMaterialProductForm { material_id: payload.material_id.clone(), new_product_form: row.product_form })],
        None => Vec::new(),
    })
}
