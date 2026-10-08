//! 🔺️ Sparse diff builder for `CreateMaterial`.
use super::CreateMaterial;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dMaterialInsertion, Fem3dMaterialsDelta};
use crate::standards::v1::subsets::any::schema::mutations::{invariant,material_breach};

use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateMaterial, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if base.materials.iter().any(|material| material.id == payload.material.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A material with id \"{}\" already exists.", payload.material.id), [payload.material.id.clone()]);
    }
    if let Some(breach) = material_breach(&payload.material) {
        return invariant(breach, vec![payload.material.id.clone()]);
    }
    if payload.index.is_some_and(|at| at > base.materials.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.materials.len()), [&payload.material.id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { materials: Some(Fem3dMaterialsDelta { inserted: vec![Fem3dMaterialInsertion { index: payload.index.unwrap_or(base.materials.len()), row: payload.material.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
