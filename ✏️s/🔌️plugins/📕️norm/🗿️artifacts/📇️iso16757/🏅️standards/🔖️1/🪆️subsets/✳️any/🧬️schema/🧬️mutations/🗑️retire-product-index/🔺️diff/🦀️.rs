//! Diff for `retire-product-index`.

use super::mutation::RetireProductIndex;
use crate::{Iso16757Diff, Iso16757Snapshot};

pub fn diff(payload: &RetireProductIndex, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.product_indexes.iter().all(|item| item.id != payload.id) {
        return protocol::MutationOutcome::fatal(
            "mutation.missing-id",
            format!("No entity with id \"{}\" exists.", payload.id),
            [payload.id.clone()],
        );
    }
    let mut catalogue = base.catalogue.clone();
    catalogue.product_indexes.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(Iso16757Diff { catalogue: Some(catalogue), ..Default::default() })
}
