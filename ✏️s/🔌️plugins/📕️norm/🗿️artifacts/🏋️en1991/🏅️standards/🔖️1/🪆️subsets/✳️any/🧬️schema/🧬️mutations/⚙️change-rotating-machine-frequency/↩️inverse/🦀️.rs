//! ↩️ `change-rotating-machine-frequency` inverse: restores the previous `frequency`.

use crate::change_rotating_machine_frequency::ChangeRotatingMachineFrequency;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeRotatingMachineFrequency, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rotating_machine_frequency::ChangeRotatingMachineFrequency> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeRotatingMachineFrequency, base: &En1991Snapshot) -> Option<ChangeRotatingMachineFrequency> {
    let r0 = base.rotating_machines.iter().find(|row| row.id == payload.id)?;
    Some(ChangeRotatingMachineFrequency { id: payload.id.clone(), new_frequency: r0.frequency })
}
