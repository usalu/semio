//! ↩️ `change-tank-shell-str-mpa` inverse: restores the previous `shell_strength_mpa`.

use crate::change_tank_shell_str_mpa::ChangeTankShellStrMpa;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTankShellStrMpa, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tank_shell_str_mpa::ChangeTankShellStrMpa> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTankShellStrMpa, base: &En1998Snapshot) -> Option<ChangeTankShellStrMpa> {
    let r0 = base.tanks.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTankShellStrMpa { id: payload.id.clone(), new_shell_strength_mpa: r0.shell_strength_mpa })
}
