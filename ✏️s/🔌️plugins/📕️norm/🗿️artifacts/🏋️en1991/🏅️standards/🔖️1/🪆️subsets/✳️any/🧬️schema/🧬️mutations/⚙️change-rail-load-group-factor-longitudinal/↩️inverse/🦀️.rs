//! ↩️ `change-rail-load-group-factor-longitudinal` inverse: restores the previous `factor_longitudinal`.

use crate::change_rail_load_group_factor_longitudinal::ChangeRailLoadGroupFactorLongitudinal;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRailLoadGroupFactorLongitudinal, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rail_load_group_factor_longitudinal::ChangeRailLoadGroupFactorLongitudinal> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRailLoadGroupFactorLongitudinal, base: &En1991Snapshot) -> Option<ChangeRailLoadGroupFactorLongitudinal> {
    let r0 = base.rail_load_groups.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRailLoadGroupFactorLongitudinal { id: payload.id.clone(), new_factor_longitudinal: r0.factor_longitudinal })
}
