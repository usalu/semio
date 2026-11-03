//! 🧪 `remove-test-series` diff: removes the addressed test series row.

use crate::remove_test_series::RemoveTestSeries;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990TestSeriesDelta};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &RemoveTestSeries, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let index = payload.index;
    if index >= base.test_series.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "test_series index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1990Diff { test_series: En1990TestSeriesDelta::removal(&base.test_series, index), ..Default::default() })
}
