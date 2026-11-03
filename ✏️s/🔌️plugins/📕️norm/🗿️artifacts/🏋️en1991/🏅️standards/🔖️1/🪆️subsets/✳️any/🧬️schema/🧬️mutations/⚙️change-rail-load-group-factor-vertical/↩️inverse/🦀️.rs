//! ↩️ `change-rail-load-group-factor-vertical` inverse: restores the previous `factor_vertical`.

use crate::change_rail_load_group_factor_vertical::ChangeRailLoadGroupFactorVertical;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRailLoadGroupFactorVertical, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rail_load_group_factor_vertical::ChangeRailLoadGroupFactorVertical> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRailLoadGroupFactorVertical, base: &En1991Snapshot) -> Option<ChangeRailLoadGroupFactorVertical> {
    let r0 = base.rail_load_groups.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRailLoadGroupFactorVertical { id: payload.id.clone(), new_factor_vertical: r0.factor_vertical })
}
