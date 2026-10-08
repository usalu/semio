//! Diff for `introduce-product-series`.

use super::mutation::IntroduceProductSeries;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ProductSeriesRows};

pub fn diff(payload: &IntroduceProductSeries, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.product_series.iter().any(|item| item.id == payload.product_series.id) {
        return protocol::MutationOutcome::fatal(
            "mutation.duplicate-id",
            format!("An entity with id \"{}\" already exists.", payload.product_series.id),
            [payload.product_series.id.clone()],
        );
    }
    let len = base.catalogue.product_series.len();
    if let Some(index) = payload.index.filter(|index| *index > len) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {index} is past the end ({len} rows) for \"{}\".", payload.product_series.id), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(Iso16757Diff { product_series: Some(Iso16757ProductSeriesRows::insertion(payload.index.unwrap_or(len), payload.product_series.clone())), ..Default::default() })
}
