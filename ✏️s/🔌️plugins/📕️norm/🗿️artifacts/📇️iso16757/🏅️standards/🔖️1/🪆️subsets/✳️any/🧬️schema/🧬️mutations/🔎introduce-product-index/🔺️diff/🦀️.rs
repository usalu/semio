//! Diff for `introduce-product-index`.

use super::mutation::IntroduceProductIndex;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ProductIndexesRows};

pub fn diff(payload: &IntroduceProductIndex, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.product_indexes.iter().any(|item| item.id == payload.product_index.id) {
        return protocol::MutationOutcome::fatal(
            "mutation.duplicate-id",
            format!("An entity with id \"{}\" already exists.", payload.product_index.id),
            [payload.product_index.id.clone()],
        );
    }
    let ids: Vec<String> = base.catalogue.product_indexes.iter().map(|item| item.id.clone()).collect();
    let clamped = matches!(payload.index, Some(index) if index > ids.len());
    let at = payload.index.filter(|index| *index <= ids.len()).unwrap_or(ids.len());
    let order = (at < ids.len()).then(|| {
        let mut order = ids.clone();
        order.insert(at, payload.product_index.id.clone());
        order
    });
    let outcome = protocol::MutationOutcome::new(Iso16757Diff { product_indexes: Some(Iso16757ProductIndexesRows { added: vec![payload.product_index.clone()], order, ..Default::default() }), ..Default::default() });
    if clamped {
        outcome.warning("mutation.clamped", format!("Insert index out of range; appended \"{}\".", payload.product_index.id))
    } else {
        outcome
    }
}
