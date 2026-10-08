//! Diff for `retire-product-index`.

use super::mutation::RetireProductIndex;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ProductIndexesRows};

pub fn diff(payload: &RetireProductIndex, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.product_indexes.iter().all(|item| item.id != payload.id) {
        return protocol::MutationOutcome::error(
            "mutation.target-missing",
            format!("No entity with id \"{}\" exists.", payload.id),
            [payload.id.clone()],
        );
    }
    protocol::MutationOutcome::new(Iso16757Diff { product_indexes: Some(Iso16757ProductIndexesRows { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
