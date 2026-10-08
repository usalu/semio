//! 🔺️ `add-product` — sparse diff construction; the derived `catalog.index` follows in `Vdi3805Diff::apply`.

use super::AddProduct;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805ProductsRows};

//#region 🔖️Diff
/// 🔺️ A duplicate article number is `mutation.duplicate-id`; an explicit index past the end
/// is `mutation.target-missing`.

pub fn diff(payload: &AddProduct, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.catalog.products.iter().any(|p| p.identity.article_number == payload.product.identity.article_number) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A product with article number \"{}\" already exists.", payload.product.identity.article_number), [payload.product.identity.article_number.clone()]);
    }
    let len = base.catalog.products.len();
    if let Some(position) = payload.index.filter(|position| *position > len) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {position} is past the end ({len} rows) for \"{}\".", payload.product.identity.article_number), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(Vdi3805Diff {
        products: Some(Vdi3805ProductsRows::insertion(payload.index.unwrap_or(len), payload.product.clone())),
        ..Default::default()
    })
}
