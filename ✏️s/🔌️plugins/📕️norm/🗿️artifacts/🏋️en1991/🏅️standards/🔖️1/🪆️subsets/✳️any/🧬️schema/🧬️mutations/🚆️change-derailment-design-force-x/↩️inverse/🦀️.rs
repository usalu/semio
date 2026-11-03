//! ↩️ `change-derailment-design-force-x` inverse: restores the previous `design_force_x`.

use crate::change_derailment_design_force_x::ChangeDerailmentDesignForceX;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDerailmentDesignForceX, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_derailment_design_force_x::ChangeDerailmentDesignForceX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDerailmentDesignForceX, base: &En1991Snapshot) -> Option<ChangeDerailmentDesignForceX> {
    let r0 = base.derailment_cases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDerailmentDesignForceX { id: payload.id.clone(), new_design_force_x: r0.design_force_x })
}
