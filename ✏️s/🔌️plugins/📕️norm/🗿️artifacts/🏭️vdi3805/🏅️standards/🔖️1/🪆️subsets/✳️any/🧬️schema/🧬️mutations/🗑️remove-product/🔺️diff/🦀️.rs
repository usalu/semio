//! 🔺️ `remove-product` — sparse diff construction; the derived `catalog.index` follows in `Vdi3805Diff::apply`.

use super::RemoveProduct;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805ProductsRows};

//#region 🔖️Diff

pub fn diff(payload: &RemoveProduct, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    let Some(index) = base.catalog.products.iter().position(|p| p.identity.article_number == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Vdi3805Diff {
        products: Some(Vdi3805ProductsRows::removal(&base.catalog.products, index)),
        ..Default::default()
    })
}
