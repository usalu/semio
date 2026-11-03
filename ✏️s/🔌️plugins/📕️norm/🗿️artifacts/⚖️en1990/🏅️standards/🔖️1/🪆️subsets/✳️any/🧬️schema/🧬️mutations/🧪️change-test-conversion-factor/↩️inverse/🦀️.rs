//! ↩️ `change-test-conversion-factor` inverse: restores the previous `conversion_factor`.

use crate::change_test_conversion_factor::ChangeTestConversionFactor;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeTestConversionFactor, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_test_conversion_factor::ChangeTestConversionFactor> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTestConversionFactor, base: &En1990Snapshot) -> Option<ChangeTestConversionFactor> {
    let r0 = base.test_series.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTestConversionFactor { id: payload.id.clone(), new_conversion_factor: r0.conversion_factor })
}
