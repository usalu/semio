//! ↩️ `change-test-mean` inverse: restores the previous `mean`.

use crate::change_test_mean::ChangeTestMean;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeTestMean, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_test_mean::ChangeTestMean> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTestMean, base: &En1990Snapshot) -> Option<ChangeTestMean> {
    let r0 = base.test_series.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTestMean { id: payload.id.clone(), new_mean: r0.mean })
}
