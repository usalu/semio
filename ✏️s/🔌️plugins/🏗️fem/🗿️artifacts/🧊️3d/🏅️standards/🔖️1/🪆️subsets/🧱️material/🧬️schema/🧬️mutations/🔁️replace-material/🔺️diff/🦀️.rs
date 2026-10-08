//! 🔺️ Sparse diff builder for `ReplaceMaterial`.
use super::ReplaceMaterial;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dMaterialsDelta, Fem3dMaterialsModification};
use crate::standards::v1::subsets::any::schema::mutations::{target_mismatch,invariant,material_breach};

use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceMaterial, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.materials.iter().find(|material| material.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.new_material.id != payload.id {
        return target_mismatch("Material", &payload.id, &payload.new_material.id);
    }
    if existing == &payload.new_material {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Material \"{}\" already has that value.", payload.id));
    }
    if let Some(breach) = material_breach(&payload.new_material) {
        return invariant(breach, vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { materials: Some(Fem3dMaterialsDelta { modified: vec![Fem3dMaterialsModification { id: payload.id.clone(), patch: payload.new_material.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
