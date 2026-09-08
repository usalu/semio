//! 🔺️ Sparse diff builder for `CreateMaterial`.
use super::CreateMaterial;
use crate::diff::{Fem3dDiff, Fem3dMaterialsDelta};
use crate::mutations::{invariant, material_breach};
use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateMaterial, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if base.materials.iter().any(|material| material.id == payload.material.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A material with id \"{}\" already exists.", payload.material.id), [payload.material.id.clone()]);
    }
    if let Some(breach) = material_breach(&payload.material) {
        return invariant(breach, vec![payload.material.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { materials: Some(Fem3dMaterialsDelta { added: vec![payload.material.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
