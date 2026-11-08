//! 🔺️ `change-product-configuration` — sparse diff construction; missing id is
//! `mutation.target-missing`. The derived
//! `catalog.index` follows in `Vdi3805Diff::apply`.

use crate::change_product_configuration::ChangeProductConfiguration;
use semio_s_artifact_norm_vdi3805_model::{sync_typed_attributes_into_records, CatalogueProduct, Vdi3805Snapshot};
use semio_s_artifact_norm_vdi3805_diff::{Vdi3805Diff, Vdi3805ProductsRows, Vdi3805ProductsPatch};

//#region 🔖️Diff
/// 🔁️ The native records of `product` after its configuration becomes `next`: the 210 record mirrors the typed attributes (Part 1 sync check), so a configuration that moves the attributes carries the rewritten 210 record in the same edit. A product whose 210 record does not mirror its attributes (neither as written nor as read back) keeps its records, so no unrelated divergence is rewritten.
pub fn mirrored_records(product: &CatalogueProduct, next: &semio_s_artifact_norm_vdi3805_model::Configuration) -> Option<Vec<semio_s_artifact_norm_vdi3805_model::NativeRecord>> {
    if product.configuration.attributes == next.attributes || !product.records.iter().any(|record| record.family.0 == "210") {
        return None;
    }
    let mut mirror = product.clone();
    sync_typed_attributes_into_records(&mut mirror);
    let derived = semio_s_artifact_norm_vdi3805_model::attributes_from_records(product.sheet, &product.records);
    if mirror.records != product.records && derived != product.configuration.attributes {
        return None;
    }
    mirror.configuration = next.clone();
    sync_typed_attributes_into_records(&mut mirror);
    Some(mirror.records)
}


pub fn diff(payload: &ChangeProductConfiguration, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    let Some(product) = base.catalog.products.iter().find(|p| p.identity.article_number == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if product.configuration == payload.new_configuration {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Product \"{}\" already has this configuration.", payload.id));
    }
    protocol::MutationOutcome::new(Vdi3805Diff {
        products: Some(Vdi3805ProductsRows::modification(&payload.id, Vdi3805ProductsPatch { configuration: Some(payload.new_configuration.clone()), records: mirrored_records(product, &payload.new_configuration), ..Default::default() })),
        ..Default::default()
    })
}
