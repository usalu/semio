//! ↩️ `change-rotating-machine-eccentricity` inverse: restores the previous `eccentricity`.

use crate::change_rotating_machine_eccentricity::ChangeRotatingMachineEccentricity;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRotatingMachineEccentricity, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rotating_machine_eccentricity::ChangeRotatingMachineEccentricity> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRotatingMachineEccentricity, base: &En1991Snapshot) -> Option<ChangeRotatingMachineEccentricity> {
    let r0 = base.rotating_machines.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRotatingMachineEccentricity { id: payload.id.clone(), new_eccentricity: r0.eccentricity })
}
