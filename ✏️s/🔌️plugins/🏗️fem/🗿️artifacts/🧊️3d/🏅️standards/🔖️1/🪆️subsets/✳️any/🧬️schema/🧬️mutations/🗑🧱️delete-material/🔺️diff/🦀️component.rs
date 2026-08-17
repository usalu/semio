//! 🔺️ Sparse diff builder for `DeleteMaterial`.
use super::mutation::DeleteMaterial;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dMaterialsDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteMaterial, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if !base.materials.iter().any(|material| material.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { materials: Some(Fem3dMaterialsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
