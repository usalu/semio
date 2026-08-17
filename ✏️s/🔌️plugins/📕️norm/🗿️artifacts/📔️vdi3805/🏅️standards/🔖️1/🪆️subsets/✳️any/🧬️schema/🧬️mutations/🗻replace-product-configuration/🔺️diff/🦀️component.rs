//! 🔺️ `replace-product-configuration` — sparse diff construction; missing id is
//! `mutation.target-missing`. Keeps the `catalog.index` entry's `dn` in lockstep with the new
//! configuration's parameters.

use super::mutation::ReplaceProductConfiguration;
use crate::artifacts::vdi3805::mutations::extract_dn;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplaceProductConfiguration, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    let Some(product) = base.catalog.products.iter().find(|p| p.identity.article_number == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if product.configuration == payload.new_configuration {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Product \"{}\" already has this configuration.", payload.id));
    }
    let mut catalog = base.catalog.clone();
    if let Some(product) = catalog.products.iter_mut().find(|p| p.identity.article_number == payload.id) {
        product.configuration = payload.new_configuration.clone();
    }
    let mut index = base.index.clone();
    if let Some(entry) = index.entries.iter_mut().find(|entry| entry.product_id == payload.id) {
        entry.dn = extract_dn(&payload.new_configuration.parameters);
    }
    protocol::MutationOutcome::new(Vdi3805Diff { catalog: Some(catalog), index: Some(index), ..Default::default() })
}
//#endregion 🔖️Diff
