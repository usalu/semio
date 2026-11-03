//! ↩️ `change-factor-override-target` inverse: restores the previous `target`.

use crate::change_factor_override_target::ChangeFactorOverrideTarget;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeFactorOverrideTarget, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_factor_override_target::ChangeFactorOverrideTarget> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFactorOverrideTarget, base: &En1990Snapshot) -> Option<ChangeFactorOverrideTarget> {
    let r0 = base.factor_overrides.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFactorOverrideTarget { id: payload.id.clone(), new_target: r0.target.clone() })
}
