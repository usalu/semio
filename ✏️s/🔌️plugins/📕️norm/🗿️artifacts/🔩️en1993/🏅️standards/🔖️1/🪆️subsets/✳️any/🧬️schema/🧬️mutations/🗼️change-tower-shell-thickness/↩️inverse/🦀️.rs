//! ↩️ `change-tower-shell-thickness` inverse: restores the previous `shell_thickness`.

use crate::change_tower_shell_thickness::ChangeTowerShellThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerShellThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_shell_thickness::ChangeTowerShellThickness> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerShellThickness { id: payload.id.clone(), new_shell_thickness: row.shell_thickness })],
        None => Vec::new(),
    })
}
