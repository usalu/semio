//! 🔺️ Sparse diff builder for `ReplaceMaterial`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on the selected id,
//! `mutation.id-mismatch` (Fatal) when the replacement renames it, the SAME elasticity bounds
//! `create-material` runs (`mutation.invariant`, Fatal), and finally `mutation.no-op`.
use super::ReplaceMaterial;
use crate::diff::{Fem2dDiff, Fem2dMaterialsDelta, Fem2dMaterialsPatchEntry};
use crate::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceMaterial, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.materials.iter().find(|material| material.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(rejection) = guards::identity_matches("material", &payload.id, &payload.new_material.id) {
        return rejection;
    }
    if let Some(rejection) = guards::material_plausibility(&payload.new_material) {
        return rejection;
    }
    if *existing == payload.new_material {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Material \"{}\" is already equal to the replacement value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { materials: Some(Fem2dMaterialsDelta { patched: vec![Fem2dMaterialsPatchEntry { id: payload.id.clone(), item: payload.new_material.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
