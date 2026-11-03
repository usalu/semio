//! ↩️ `change-test-pairs` inverse: restores the previous `pairs`.

use crate::change_test_pairs::ChangeTestPairs;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeTestPairs, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_test_pairs::ChangeTestPairs> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTestPairs, base: &En1990Snapshot) -> Option<ChangeTestPairs> {
    let r0 = base.test_series.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTestPairs { id: payload.id.clone(), new_pairs: r0.pairs.clone() })
}
