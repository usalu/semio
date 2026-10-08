//! 🔺️ `introduce-product` — sparse diff construction.

use super::mutation::IntroduceProduct;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ProductsRows};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is `mutation.duplicate-id`; an out-of-range explicit index clamps to the
/// end with `mutation.clamped`.

pub fn diff(payload: &IntroduceProduct, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.products.iter().any(|product| product.id == payload.product.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A product with id \"{}\" already exists.", payload.product.id), [payload.product.id.clone()]);
    }
    let ids: Vec<String> = base.catalogue.products.iter().map(|item| item.id.clone()).collect();
    let clamped = matches!(payload.index, Some(index) if index > ids.len());
    let at = payload.index.filter(|index| *index <= ids.len()).unwrap_or(ids.len());
    let order = (at < ids.len()).then(|| {
        let mut order = ids.clone();
        order.insert(at, payload.product.id.clone());
        order
    });
    let outcome = protocol::MutationOutcome::new(Iso16757Diff { products: Some(Iso16757ProductsRows { added: vec![payload.product.clone()], order, ..Default::default() }), ..Default::default() });
    if clamped {
        outcome.warning("mutation.clamped", format!("Insert index was out of range; appended product \"{}\" at the end instead.", payload.product.id))
    } else {
        outcome
    }
}
