//! 🔺️ `change-material-designation` diff.

use crate::diff::En1999Diff;
use crate::mutations::change_material_designation::ChangeMaterialDesignation;
use crate::En1999Snapshot;

pub fn diff(payload: &ChangeMaterialDesignation, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let mut materials = base.materials.clone();
    let Some(mat) = materials.iter_mut().find(|m| m.id == payload.material_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown material {}", payload.material_id), Vec::<String>::new());
    };
    mat.designation = payload.new_designation.clone();
    protocol::MutationOutcome::new(En1999Diff { materials: Some(materials), ..Default::default() })
}
