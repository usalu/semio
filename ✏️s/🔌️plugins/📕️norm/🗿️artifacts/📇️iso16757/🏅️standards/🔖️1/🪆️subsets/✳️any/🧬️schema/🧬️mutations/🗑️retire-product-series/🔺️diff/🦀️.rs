//! Diff for `retire-product-series`.

use super::mutation::RetireProductSeries;
use crate::{Iso16757Diff, Iso16757Snapshot};

pub fn diff(payload: &RetireProductSeries, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.product_series.iter().all(|item| item.id != payload.id) {
        return protocol::MutationOutcome::fatal(
            "mutation.missing-id",
            format!("No entity with id \"{}\" exists.", payload.id),
            [payload.id.clone()],
        );
    }
    let mut catalogue = base.catalogue.clone();
    catalogue.product_series.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(Iso16757Diff { catalogue: Some(catalogue), ..Default::default() })
}
