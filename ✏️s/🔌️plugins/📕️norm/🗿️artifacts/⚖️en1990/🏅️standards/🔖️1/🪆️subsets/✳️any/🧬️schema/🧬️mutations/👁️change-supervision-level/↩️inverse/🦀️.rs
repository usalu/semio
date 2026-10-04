//! ↩️ `change-supervision-level` inverse.

use super::ChangeSupervisionLevel;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeSupervisionLevel, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let _ = mutation;
    vec![En1990Mutation::ChangeSupervisionLevel(ChangeSupervisionLevel { new_supervision_level: base.supervision_level.clone() })]

    })())
}
