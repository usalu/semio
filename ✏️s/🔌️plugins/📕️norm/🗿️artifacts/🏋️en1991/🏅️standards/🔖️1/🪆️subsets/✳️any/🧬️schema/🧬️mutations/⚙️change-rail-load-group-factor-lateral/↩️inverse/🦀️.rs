//! ↩️ `change-rail-load-group-factor-lateral` inverse: restores the previous `factor_lateral`.

use crate::change_rail_load_group_factor_lateral::ChangeRailLoadGroupFactorLateral;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRailLoadGroupFactorLateral, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rail_load_group_factor_lateral::ChangeRailLoadGroupFactorLateral> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRailLoadGroupFactorLateral, base: &En1991Snapshot) -> Option<ChangeRailLoadGroupFactorLateral> {
    let r0 = base.rail_load_groups.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRailLoadGroupFactorLateral { id: payload.id.clone(), new_factor_lateral: r0.factor_lateral })
}
