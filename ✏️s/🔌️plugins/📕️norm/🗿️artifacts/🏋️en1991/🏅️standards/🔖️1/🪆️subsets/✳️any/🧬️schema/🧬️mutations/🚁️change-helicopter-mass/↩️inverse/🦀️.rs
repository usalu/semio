//! ↩️ `change-helicopter-mass` inverse: restores the previous `mass`.

use crate::change_helicopter_mass::ChangeHelicopterMass;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeHelicopterMass, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_helicopter_mass::ChangeHelicopterMass> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeHelicopterMass, base: &En1991Snapshot) -> Option<ChangeHelicopterMass> {
    let r0 = base.helicopter_landings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeHelicopterMass { id: payload.id.clone(), new_mass: r0.mass })
}
