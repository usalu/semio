//! ↩️ `change-derailment-design-force-y` inverse: restores the previous `design_force_y`.

use crate::change_derailment_design_force_y::ChangeDerailmentDesignForceY;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDerailmentDesignForceY, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_derailment_design_force_y::ChangeDerailmentDesignForceY> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDerailmentDesignForceY, base: &En1991Snapshot) -> Option<ChangeDerailmentDesignForceY> {
    let r0 = base.derailment_cases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDerailmentDesignForceY { id: payload.id.clone(), new_design_force_y: r0.design_force_y })
}
