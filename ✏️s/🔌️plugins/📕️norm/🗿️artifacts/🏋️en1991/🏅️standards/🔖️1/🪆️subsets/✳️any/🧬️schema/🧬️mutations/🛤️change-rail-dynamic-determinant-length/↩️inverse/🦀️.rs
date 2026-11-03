//! ↩️ `change-rail-dynamic-determinant-length` inverse: restores the previous `determinant_length`.

use crate::change_rail_dynamic_determinant_length::ChangeRailDynamicDeterminantLength;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRailDynamicDeterminantLength, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rail_dynamic_determinant_length::ChangeRailDynamicDeterminantLength> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRailDynamicDeterminantLength, base: &En1991Snapshot) -> Option<ChangeRailDynamicDeterminantLength> {
    let r0 = base.rail_dynamics.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRailDynamicDeterminantLength { id: payload.id.clone(), new_determinant_length: r0.determinant_length })
}
