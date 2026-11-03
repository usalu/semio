//! ↩️ `change-factor-override-value` inverse: restores the previous `value`.

use crate::change_factor_override_value::ChangeFactorOverrideValue;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeFactorOverrideValue, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_factor_override_value::ChangeFactorOverrideValue> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFactorOverrideValue, base: &En1990Snapshot) -> Option<ChangeFactorOverrideValue> {
    let r0 = base.factor_overrides.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFactorOverrideValue { id: payload.id.clone(), new_value: r0.value })
}
