//! 🔺️ `change-materials` diff.

use crate::diff::En1999Diff;
use crate::mutations::change_materials::ChangeMaterials;
use crate::En1999Snapshot;

pub fn diff(payload: &ChangeMaterials, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if &base.materials == &payload.materials {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "List unchanged.");
    }
    protocol::MutationOutcome::new(En1999Diff { materials: Some(payload.materials.clone()), ..Default::default() })
}
