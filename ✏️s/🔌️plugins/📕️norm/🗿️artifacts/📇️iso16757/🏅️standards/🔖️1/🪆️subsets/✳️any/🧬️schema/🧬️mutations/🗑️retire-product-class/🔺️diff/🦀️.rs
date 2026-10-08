//! Diff for `retire-product-class`.

use super::mutation::RetireProductClass;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ProductClassesRows};

pub fn diff(payload: &RetireProductClass, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.product_classes.iter().all(|item| item.id != payload.id) {
        return protocol::MutationOutcome::error(
            "mutation.target-missing",
            format!("No entity with id \"{}\" exists.", payload.id),
            [payload.id.clone()],
        );
    }
    protocol::MutationOutcome::new(Iso16757Diff { product_classes: Some(Iso16757ProductClassesRows { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
