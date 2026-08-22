//! 🔺️ Sparse diff builder for `CreateMaterial`.
use super::mutation::CreateMaterial;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dMaterialsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateMaterial, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if base.materials.iter().any(|material| material.id == payload.material.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A material with id \"{}\" already exists.", payload.material.id), [payload.material.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { materials: Some(Fem2dMaterialsDelta { added: vec![payload.material.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
