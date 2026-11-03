//! ↩️ `change-rotating-machine-mass` inverse: restores the previous `mass`.

use crate::change_rotating_machine_mass::ChangeRotatingMachineMass;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRotatingMachineMass, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rotating_machine_mass::ChangeRotatingMachineMass> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRotatingMachineMass, base: &En1991Snapshot) -> Option<ChangeRotatingMachineMass> {
    let r0 = base.rotating_machines.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRotatingMachineMass { id: payload.id.clone(), new_mass: r0.mass })
}
