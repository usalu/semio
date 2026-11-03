//! ↩️ `change-factor-override-factor` inverse: restores the previous `factor`.

use crate::change_factor_override_factor::ChangeFactorOverrideFactor;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeFactorOverrideFactor, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_factor_override_factor::ChangeFactorOverrideFactor> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFactorOverrideFactor, base: &En1990Snapshot) -> Option<ChangeFactorOverrideFactor> {
    let r0 = base.factor_overrides.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFactorOverrideFactor { id: payload.id.clone(), new_factor: r0.factor.clone() })
}
