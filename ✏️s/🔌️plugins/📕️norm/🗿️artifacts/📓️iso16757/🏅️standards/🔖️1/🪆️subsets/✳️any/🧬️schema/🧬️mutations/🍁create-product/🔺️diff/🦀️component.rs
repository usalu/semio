//! 🔺️ `create-product` — sparse diff construction.

use super::mutation::CreateProduct;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is `mutation.duplicate-id`; an out-of-range explicit index clamps to the
/// end with `mutation.clamped`.
pub async fn diff(payload: &CreateProduct, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.products.iter().any(|product| product.id == payload.product.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A product with id \"{}\" already exists.", payload.product.id), [payload.product.id.clone()]);
    }
    let mut catalogue = base.catalogue.clone();
    let clamped = matches!(payload.index, Some(index) if index > catalogue.products.len());
    match payload.index {
        Some(index) if index <= catalogue.products.len() => catalogue.products.insert(index, payload.product.clone()),
        _ => catalogue.products.push(payload.product.clone()),
    }
    let outcome = protocol::MutationOutcome::new(Iso16757Diff { catalogue: Some(catalogue), ..Default::default() });
    if clamped {
        outcome.warn("mutation.clamped", format!("Insert index was out of range; appended product \"{}\" at the end instead.", payload.product.id))
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
