//! ↩️ `insert-test-series` inverse: removes the inserted row again.

use crate::insert_test_series::InsertTestSeries;
use crate::remove_test_series::RemoveTestSeries;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &InsertTestSeries, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_test_series::RemoveTestSeries> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertTestSeries, base: &En1990Snapshot) -> Option<RemoveTestSeries> {
    let index = payload.index.unwrap_or(usize::MAX).min(base.test_series.len());
    Some(RemoveTestSeries { index })
}
