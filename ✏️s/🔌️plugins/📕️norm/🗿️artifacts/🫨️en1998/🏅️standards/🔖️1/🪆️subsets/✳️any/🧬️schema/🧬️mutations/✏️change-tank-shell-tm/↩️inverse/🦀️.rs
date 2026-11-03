//! ↩️ `change-tank-shell-tm` inverse: restores the previous `shell_thickness_m`.

use crate::change_tank_shell_t_m::ChangeTankShellTM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTankShellTM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tank_shell_t_m::ChangeTankShellTM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTankShellTM, base: &En1998Snapshot) -> Option<ChangeTankShellTM> {
    let r0 = base.tanks.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTankShellTM { id: payload.id.clone(), new_shell_thickness_m: r0.shell_thickness_m })
}
