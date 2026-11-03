//! 🧪 `insert-test-series` diff: inserts the test series row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_test_series::InsertTestSeries;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990TestSeriesDelta};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &InsertTestSeries, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if base.test_series.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Test series {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.test_series.len());
    let outcome = protocol::MutationOutcome::new(En1990Diff { test_series: En1990TestSeriesDelta::insertion(index, payload.item.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the test series list; inserted at {index}.", payload.index.unwrap_or(index)))
}
