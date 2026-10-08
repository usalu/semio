//! 🔺️ `introduce-product` — sparse diff construction.

use super::mutation::IntroduceProduct;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ProductsRows};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is `mutation.duplicate-id`; an explicit index past the end is
/// `mutation.target-missing`.

pub fn diff(payload: &IntroduceProduct, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.products.iter().any(|product| product.id == payload.product.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A product with id \"{}\" already exists.", payload.product.id), [payload.product.id.clone()]);
    }
    let len = base.catalogue.products.len();
    if let Some(index) = payload.index.filter(|index| *index > len) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {index} is past the end ({len} rows) for \"{}\".", payload.product.id), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(Iso16757Diff { products: Some(Iso16757ProductsRows::insertion(payload.index.unwrap_or(len), payload.product.clone())), ..Default::default() })
}
