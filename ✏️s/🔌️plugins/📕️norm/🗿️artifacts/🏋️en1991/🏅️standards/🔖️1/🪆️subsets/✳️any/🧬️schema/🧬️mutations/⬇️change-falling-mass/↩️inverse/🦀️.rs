//! ↩️ `change-falling-mass` inverse: restores the previous `mass`.

use crate::change_falling_mass::ChangeFallingMass;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFallingMass, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_falling_mass::ChangeFallingMass> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFallingMass, base: &En1991Snapshot) -> Option<ChangeFallingMass> {
    let r0 = base.falling_loads.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFallingMass { id: payload.id.clone(), new_mass: r0.mass })
}
