//! 🔺️ `rename-product` — sparse diff construction; missing id is `mutation.target-missing`. The derived
//! `catalog.index` follows in `Vdi3805Diff::apply`.

use super::RenameProduct;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805ProductsRows, Vdi3805ProductsPatch};

//#region 🔖️Diff

pub fn diff(payload: &RenameProduct, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    let Some(product) = base.catalog.products.iter().find(|p| p.identity.article_number == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if product.title == payload.new_title {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Product \"{}\" already has that title.", payload.id));
    }
    protocol::MutationOutcome::new(Vdi3805Diff {
        products: Some(Vdi3805ProductsRows::modification(&payload.id, Vdi3805ProductsPatch { title: Some(payload.new_title.clone()), ..Default::default() })),
        ..Default::default()
    })
}
