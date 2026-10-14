//! ↩️ `change-product-configuration` — undo restores BASE's configuration; missing id ⇒
//! `Vec::new()`.

use crate::change_product_configuration::ChangeProductConfiguration;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &ChangeProductConfiguration, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_product_configuration::ChangeProductConfiguration> {
    Ok((|| {
    let Some(product) = base.catalog.products.iter().find(|p| p.identity.article_number == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(ChangeProductConfiguration { id: payload.id.clone(), new_configuration: product.configuration.clone() })]

    })())
}
//#endregion 🔖️Inverse
