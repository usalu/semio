//! Diff for `introduce-product-class`.

use super::mutation::IntroduceProductClass;
use crate::{Iso16757Diff, Iso16757Snapshot};

pub fn diff(payload: &IntroduceProductClass, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.product_classes.iter().any(|item| item.id == payload.product_class.id) {
        return protocol::MutationOutcome::fatal(
            "mutation.duplicate-id",
            format!("An entity with id \"{}\" already exists.", payload.product_class.id),
            [payload.product_class.id.clone()],
        );
    }
    let mut catalogue = base.catalogue.clone();
    let clamped = matches!(payload.index, Some(index) if index > catalogue.product_classes.len());
    match payload.index {
        Some(index) if index <= catalogue.product_classes.len() => catalogue.product_classes.insert(index, payload.product_class.clone()),
        _ => catalogue.product_classes.push(payload.product_class.clone()),
    }
    let outcome = protocol::MutationOutcome::new(Iso16757Diff { catalogue: Some(catalogue), ..Default::default() });
    if clamped {
        outcome.warn("mutation.clamped", format!("Insert index out of range; appended \"{}\".", payload.product_class.id))
    } else {
        outcome
    }
}
