//! 🔺️ Sparse diff builder for `ReplaceMaterial`.
use super::mutation::ReplaceMaterial;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dMaterialsDelta, Fem2dMaterialsPatchEntry};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceMaterial, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.materials.iter().find(|material| material.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if *existing == payload.new_material {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Material \"{}\" is already equal to the replacement value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { materials: Some(Fem2dMaterialsDelta { patched: vec![Fem2dMaterialsPatchEntry { id: payload.id.clone(), item: payload.new_material.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
