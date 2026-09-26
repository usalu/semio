//! Diff for `introduce-product-index`.

use super::mutation::IntroduceProductIndex;
use crate::{Iso16757Diff, Iso16757Snapshot};

pub fn diff(payload: &IntroduceProductIndex, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.product_indexes.iter().any(|item| item.id == payload.product_index.id) {
        return protocol::MutationOutcome::fatal(
            "mutation.duplicate-id",
            format!("An entity with id \"{}\" already exists.", payload.product_index.id),
            [payload.product_index.id.clone()],
        );
    }
    let mut catalogue = base.catalogue.clone();
    let clamped = matches!(payload.index, Some(index) if index > catalogue.product_indexes.len());
    match payload.index {
        Some(index) if index <= catalogue.product_indexes.len() => catalogue.product_indexes.insert(index, payload.product_index.clone()),
        _ => catalogue.product_indexes.push(payload.product_index.clone()),
    }
    let outcome = protocol::MutationOutcome::new(Iso16757Diff { catalogue: Some(catalogue), ..Default::default() });
    if clamped {
        outcome.warn("mutation.clamped", format!("Insert index out of range; appended \"{}\".", payload.product_index.id))
    } else {
        outcome
    }
}
