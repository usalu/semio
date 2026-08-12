//! ↩️ `replace-product-configuration` — undo restores BASE's configuration; missing id ⇒
//! `Vec::new()`.

use super::mutation::ReplaceProductConfiguration;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceProductConfiguration, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    let Some(product) = base.catalog.products.iter().find(|p| p.identity.article_number == payload.id) else {
        return Vec::new();
    };
    vec![Vdi3805Mutation::ReplaceProductConfiguration(ReplaceProductConfiguration { id: payload.id.clone(), new_configuration: product.configuration.clone() })]
}
//#endregion 🔖️Inverse
