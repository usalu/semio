//! 🔺️ Sparse diff builder for `CreateMaterial`.
use super::CreateMaterial;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dMaterialsDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateMaterial, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if base.materials.iter().any(|material| material.id == payload.material.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A material with id \"{}\" already exists.", payload.material.id), [payload.material.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { materials: Some(Fem3dMaterialsDelta { added: vec![payload.material.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
