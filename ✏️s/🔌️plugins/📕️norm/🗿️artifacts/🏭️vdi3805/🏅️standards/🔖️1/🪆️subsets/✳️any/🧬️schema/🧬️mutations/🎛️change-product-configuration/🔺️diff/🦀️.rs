//! 🔺️ `change-product-configuration` — sparse diff construction; missing id is
//! `mutation.target-missing`. Keeps the `catalog.index` entry's `dn` in lockstep with the new
//! configuration's parameters.

use super::ChangeProductConfiguration;
use crate::mutations::extract_dn;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805ProductsRows, Vdi3805ProductsPatch, Vdi3805IndexEntriesRows, Vdi3805IndexEntriesPatch, Vdi3805IndexEntriesPatchDnValue};

//#region 🔖️Diff

pub fn diff(payload: &ChangeProductConfiguration, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    let Some(product) = base.catalog.products.iter().find(|p| p.identity.article_number == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if product.configuration == payload.new_configuration {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Product \"{}\" already has this configuration.", payload.id));
    }
    let dn = extract_dn(&payload.new_configuration.attributes);
    let index_entries = base.index.entries.iter().find(|entry| entry.product_id == payload.id).filter(|entry| entry.dn != dn).map(|_| Vdi3805IndexEntriesRows {
        modified: vec![Vdi3805IndexEntriesPatch { id: payload.id.clone(), dn: Some(Vdi3805IndexEntriesPatchDnValue { value: dn }), ..Default::default() }],
        ..Default::default()
    });
    protocol::MutationOutcome::new(Vdi3805Diff {
        products: Some(Vdi3805ProductsRows { modified: vec![Vdi3805ProductsPatch { id: payload.id.clone(), configuration: Some(payload.new_configuration.clone()), ..Default::default() }], ..Default::default() }),
        index_entries,
        ..Default::default()
    })
}
