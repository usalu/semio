//! 🔺️ `change-material-designation` diff.

use crate::mutations::change_material_designation::ChangeMaterialDesignation;
use crate::En1999Snapshot;
use crate::diff::{En1999Diff, En1999MaterialsRows, En1999MaterialsPatch};

pub fn diff(payload: &ChangeMaterialDesignation, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !base.materials.iter().any(|material| material.id == payload.material_id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown material {}", payload.material_id), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff {
        materials: Some(En1999MaterialsRows { modified: vec![En1999MaterialsPatch { id: payload.material_id.clone(), designation: Some(payload.new_designation.clone()), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
