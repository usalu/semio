//! ↩️ `change-supervision-level` inverse.

use super::ChangeSupervisionLevel;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeSupervisionLevel, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    let _ = mutation;
    vec![En1990Mutation::ChangeSupervisionLevel(ChangeSupervisionLevel { new_supervision_level: base.supervision_level.clone() })]
}
