//! ↩️ `change-accidental-explosion-mass` inverse: restores the previous `explosion_mass`.

use crate::change_accidental_explosion_mass::ChangeAccidentalExplosionMass;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeAccidentalExplosionMass, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_accidental_explosion_mass::ChangeAccidentalExplosionMass> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAccidentalExplosionMass, base: &En1991Snapshot) -> Option<ChangeAccidentalExplosionMass> {
    let r0 = base.accidental_cases.iter().find(|row| row.id == payload.id)?;
    let current = r0.explosion.first()?;
    Some(ChangeAccidentalExplosionMass { id: payload.id.clone(), new_explosion_mass: current.explosion_mass })
}
