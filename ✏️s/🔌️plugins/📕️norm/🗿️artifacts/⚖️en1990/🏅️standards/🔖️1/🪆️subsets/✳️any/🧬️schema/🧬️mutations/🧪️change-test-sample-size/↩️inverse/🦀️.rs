//! ↩️ `change-test-sample-size` inverse: restores the previous `sample_size`.

use crate::change_test_sample_size::ChangeTestSampleSize;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeTestSampleSize, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_test_sample_size::ChangeTestSampleSize> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTestSampleSize, base: &En1990Snapshot) -> Option<ChangeTestSampleSize> {
    let r0 = base.test_series.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTestSampleSize { id: payload.id.clone(), new_sample_size: r0.sample_size })
}
