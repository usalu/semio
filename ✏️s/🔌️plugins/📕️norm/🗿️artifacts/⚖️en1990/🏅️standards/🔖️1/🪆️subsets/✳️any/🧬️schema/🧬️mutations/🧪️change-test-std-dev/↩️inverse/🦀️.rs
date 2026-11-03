//! ↩️ `change-test-std-dev` inverse: restores the previous `std_dev`.

use crate::change_test_std_dev::ChangeTestStdDev;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeTestStdDev, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_test_std_dev::ChangeTestStdDev> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTestStdDev, base: &En1990Snapshot) -> Option<ChangeTestStdDev> {
    let r0 = base.test_series.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTestStdDev { id: payload.id.clone(), new_std_dev: r0.std_dev })
}
