//! ↩️ `change-supervision-level` inverse.

use crate::change_supervision_level::ChangeSupervisionLevel;

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeSupervisionLevel, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_supervision_level::ChangeSupervisionLevel> {
    Ok(match previous(mutation, base) {
        Some(inverse) => vec![Op::from(inverse)],
        None => Vec::new(),
    })
}

fn previous(_mutation: &ChangeSupervisionLevel, base: &En1990Snapshot) -> Option<crate::change_supervision_level::ChangeSupervisionLevel> {
    Some(ChangeSupervisionLevel { new_supervision_level: base.supervision_level })
}
