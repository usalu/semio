//! 🔺️ Sparse diff builder for `DeleteMaterial`.
use super::mutation::DeleteMaterial;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dMaterialsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteMaterial, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if !base.materials.iter().any(|material| material.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { materials: Some(Fem2dMaterialsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
