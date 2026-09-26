//! Diff for `retire-product-class`.

use super::mutation::RetireProductClass;
use crate::{Iso16757Diff, Iso16757Snapshot};

pub fn diff(payload: &RetireProductClass, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.product_classes.iter().all(|item| item.id != payload.id) {
        return protocol::MutationOutcome::fatal(
            "mutation.missing-id",
            format!("No entity with id \"{}\" exists.", payload.id),
            [payload.id.clone()],
        );
    }
    let mut catalogue = base.catalogue.clone();
    catalogue.product_classes.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(Iso16757Diff { catalogue: Some(catalogue), ..Default::default() })
}
