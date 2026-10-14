//! ↩️ `change-supervision-level` inverse.

use crate::change_supervision_level::ChangeSupervisionLevel;

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeSupervisionLevel, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_supervision_level::ChangeSupervisionLevel> {
    Ok((|| {
    let _ = mutation;
    vec![Op::from(ChangeSupervisionLevel { new_supervision_level: base.supervision_level.clone() })]

    })())
}
