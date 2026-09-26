//! ↩️ `change-inspection-level` inverse.

use super::ChangeInspectionLevel;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeInspectionLevel, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    let _ = mutation;
    vec![En1990Mutation::ChangeInspectionLevel(ChangeInspectionLevel { new_inspection_level: base.inspection_level.clone() })]
}
