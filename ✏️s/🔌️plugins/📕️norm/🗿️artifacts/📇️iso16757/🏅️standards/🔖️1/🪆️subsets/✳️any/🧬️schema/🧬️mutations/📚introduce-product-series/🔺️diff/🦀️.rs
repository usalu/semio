//! Diff for `introduce-product-series`.

use super::mutation::IntroduceProductSeries;
use crate::{Iso16757Diff, Iso16757Snapshot};

pub fn diff(payload: &IntroduceProductSeries, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.product_series.iter().any(|item| item.id == payload.product_series.id) {
        return protocol::MutationOutcome::fatal(
            "mutation.duplicate-id",
            format!("An entity with id \"{}\" already exists.", payload.product_series.id),
            [payload.product_series.id.clone()],
        );
    }
    let mut catalogue = base.catalogue.clone();
    let clamped = matches!(payload.index, Some(index) if index > catalogue.product_series.len());
    match payload.index {
        Some(index) if index <= catalogue.product_series.len() => catalogue.product_series.insert(index, payload.product_series.clone()),
        _ => catalogue.product_series.push(payload.product_series.clone()),
    }
    let outcome = protocol::MutationOutcome::new(Iso16757Diff { catalogue: Some(catalogue), ..Default::default() });
    if clamped {
        outcome.warn("mutation.clamped", format!("Insert index out of range; appended \"{}\".", payload.product_series.id))
    } else {
        outcome
    }
}
