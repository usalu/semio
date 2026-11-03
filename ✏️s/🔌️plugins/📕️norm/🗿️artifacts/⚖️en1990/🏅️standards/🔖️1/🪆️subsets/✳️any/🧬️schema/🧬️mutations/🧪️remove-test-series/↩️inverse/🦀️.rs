//! ↩️ `remove-test-series` inverse: inserts the removed row back at its position.

use crate::remove_test_series::RemoveTestSeries;
use crate::insert_test_series::InsertTestSeries;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &RemoveTestSeries, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_test_series::InsertTestSeries> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveTestSeries, base: &En1990Snapshot) -> Option<InsertTestSeries> {
    let index = payload.index;
    let row = base.test_series.get(index)?;
    Some(InsertTestSeries { index: Some(index), item: row.clone() })
}
