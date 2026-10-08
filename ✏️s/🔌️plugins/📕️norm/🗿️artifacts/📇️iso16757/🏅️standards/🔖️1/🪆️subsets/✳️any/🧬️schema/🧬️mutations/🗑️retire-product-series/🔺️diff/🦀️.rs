//! Diff for `retire-product-series`.

use super::mutation::RetireProductSeries;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ProductSeriesRows};

pub fn diff(payload: &RetireProductSeries, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    let Some(index) = base.catalogue.product_series.iter().position(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error(
            "mutation.target-missing",
            format!("No entity with id \"{}\" exists.", payload.id),
            [payload.id.clone()],
        );
    };
    protocol::MutationOutcome::new(Iso16757Diff { product_series: Some(Iso16757ProductSeriesRows::removal(&base.catalogue.product_series, index)), ..Default::default() })
}
