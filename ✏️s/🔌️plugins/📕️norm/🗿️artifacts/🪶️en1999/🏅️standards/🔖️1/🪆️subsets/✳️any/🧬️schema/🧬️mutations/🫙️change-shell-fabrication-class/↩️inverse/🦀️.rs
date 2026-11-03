//! ↩️ `change-shell-fabrication-class` inverse: restores the previous `fabrication_class`.

use crate::change_shell_fabrication_class::ChangeShellFabricationClass;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeShellFabricationClass, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_shell_fabrication_class::ChangeShellFabricationClass> {
    Ok(match base.shells.iter().find(|row| row.id == payload.shell_id) {
        Some(row) => vec![Op::from(ChangeShellFabricationClass { shell_id: payload.shell_id.clone(), new_fabrication_class: row.fabrication_class })],
        None => Vec::new(),
    })
}
