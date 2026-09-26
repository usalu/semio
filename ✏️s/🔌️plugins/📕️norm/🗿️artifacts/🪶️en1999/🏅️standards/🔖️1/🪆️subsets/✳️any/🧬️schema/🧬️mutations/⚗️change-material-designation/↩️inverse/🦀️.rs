//! ↩️ `change-material-designation` inverse.

use crate::mutations::change_material_designation::ChangeMaterialDesignation;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

pub fn inverse(payload: &ChangeMaterialDesignation, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    let d = base.materials.iter().find(|m| m.id == payload.material_id).map(|m| m.designation.clone()).unwrap_or_default();
    vec![En1999Mutation::ChangeMaterialDesignation(ChangeMaterialDesignation { material_id: payload.material_id.clone(), new_designation: d })]
}
