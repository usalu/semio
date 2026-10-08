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
    let removed: Vec<String> = base.materials.iter().filter(|row| !payload.materials.contains(row)).map(|row| row.id.clone()).collect();
    let added: Vec<_> = payload.materials.iter().filter(|row| !base.materials.contains(row)).cloned().collect();
    let mut natural: Vec<String> = base.materials.iter().filter(|row| !removed.contains(&row.id)).map(|row| row.id.clone()).collect();
    natural.extend(added.iter().map(|row| row.id.clone()));
    let wanted: Vec<String> = payload.materials.iter().map(|row| row.id.clone()).collect();
    let order = (natural != wanted).then_some(wanted);
    protocol::MutationOutcome::new(En1999Diff { materials: Some(En1999MaterialsRows { added, removed, order, ..Default::default() }), ..Default::default() })
}
