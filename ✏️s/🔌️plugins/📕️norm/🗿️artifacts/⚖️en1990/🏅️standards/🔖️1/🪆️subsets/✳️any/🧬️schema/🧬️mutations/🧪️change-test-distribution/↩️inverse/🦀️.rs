//! ↩️ `change-test-distribution` inverse: restores the previous `distribution`.

use crate::change_test_distribution::ChangeTestDistribution;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeTestDistribution, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_test_distribution::ChangeTestDistribution> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTestDistribution, base: &En1990Snapshot) -> Option<ChangeTestDistribution> {
    let r0 = base.test_series.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTestDistribution { id: payload.id.clone(), new_distribution: r0.distribution })
}
