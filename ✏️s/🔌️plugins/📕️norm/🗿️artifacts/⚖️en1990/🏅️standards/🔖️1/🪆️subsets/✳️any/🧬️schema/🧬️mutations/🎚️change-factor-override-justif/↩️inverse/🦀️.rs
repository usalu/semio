//! ↩️ `change-factor-override-justif` inverse: restores the previous `justification`.

use crate::change_factor_override_justif::ChangeFactorOverrideJustif;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeFactorOverrideJustif, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_factor_override_justif::ChangeFactorOverrideJustif> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFactorOverrideJustif, base: &En1990Snapshot) -> Option<ChangeFactorOverrideJustif> {
    let r0 = base.factor_overrides.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFactorOverrideJustif { id: payload.id.clone(), new_justification: r0.justification.clone() })
}
