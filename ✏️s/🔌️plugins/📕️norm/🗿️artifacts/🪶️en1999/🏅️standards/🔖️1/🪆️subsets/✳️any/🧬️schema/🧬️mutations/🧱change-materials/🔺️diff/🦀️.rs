//! 🔺️ `change-materials` diff.

use crate::mutations::change_materials::ChangeMaterials;
use crate::En1999Snapshot;
use crate::diff::{En1999Diff, En1999MaterialsRows};

pub fn diff(payload: &ChangeMaterials, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if &base.materials == &payload.materials {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "List unchanged.");
    }
    if let Some((_, row)) = payload.materials.iter().enumerate().find(|(at, row)| payload.materials[..*at].iter().any(|earlier| earlier.id == row.id)) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Material id {} appears twice.", row.id), [row.id.clone()]);
    }
    protocol::MutationOutcome::new(En1999Diff { materials: Some(En1999MaterialsRows::setting(&base.materials, &payload.materials)), ..Default::default() })
}
