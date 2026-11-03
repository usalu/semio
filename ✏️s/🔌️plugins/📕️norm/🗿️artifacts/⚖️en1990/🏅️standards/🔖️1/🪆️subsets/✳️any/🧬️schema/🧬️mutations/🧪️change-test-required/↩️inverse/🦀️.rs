//! ↩️ `change-test-required` inverse: restores the previous `required`.

use crate::change_test_required::ChangeTestRequired;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeTestRequired, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_test_required::ChangeTestRequired> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTestRequired, base: &En1990Snapshot) -> Option<ChangeTestRequired> {
    let r0 = base.test_series.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTestRequired { id: payload.id.clone(), new_required: r0.required })
}
