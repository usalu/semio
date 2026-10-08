//! Diff for `introduce-product-class`.

use super::mutation::IntroduceProductClass;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ProductClassesRows};

pub fn diff(payload: &IntroduceProductClass, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.product_classes.iter().any(|item| item.id == payload.product_class.id) {
        return protocol::MutationOutcome::fatal(
            "mutation.duplicate-id",
            format!("An entity with id \"{}\" already exists.", payload.product_class.id),
            [payload.product_class.id.clone()],
        );
    }
    let ids: Vec<String> = base.catalogue.product_classes.iter().map(|item| item.id.clone()).collect();
    let clamped = matches!(payload.index, Some(index) if index > ids.len());
    let at = payload.index.filter(|index| *index <= ids.len()).unwrap_or(ids.len());
    let order = (at < ids.len()).then(|| {
        let mut order = ids.clone();
        order.insert(at, payload.product_class.id.clone());
        order
    });
    let outcome = protocol::MutationOutcome::new(Iso16757Diff { product_classes: Some(Iso16757ProductClassesRows { added: vec![payload.product_class.clone()], order, ..Default::default() }), ..Default::default() });
    if clamped {
        outcome.warning("mutation.clamped", format!("Insert index out of range; appended \"{}\".", payload.product_class.id))
    } else {
        outcome
    }
}
