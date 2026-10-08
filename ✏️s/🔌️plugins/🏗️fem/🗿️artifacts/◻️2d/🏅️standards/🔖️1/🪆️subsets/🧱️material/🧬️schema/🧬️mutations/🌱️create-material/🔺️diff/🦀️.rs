//! 🔺️ Sparse diff builder for `CreateMaterial`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), then the shared
//! `guards::material_plausibility` elasticity bounds (`mutation.invariant`, Fatal).
use super::CreateMaterial;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dMaterialsDelta, insertion_order};
use crate::standards::v1::subsets::any::schema::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateMaterial, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if base.materials.iter().any(|material| material.id == payload.material.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A material with id \"{}\" already exists.", payload.material.id), [payload.material.id.clone()]);
    }
    if let Some(rejection) = guards::material_plausibility(&payload.material) {
        return rejection;
    }
    protocol::MutationOutcome::new(Fem2dDiff { materials: Some(Fem2dMaterialsDelta { added: vec![payload.material.clone()], reordered: insertion_order(base.materials.iter().map(|item| item.id.as_str()), &payload.material.id, payload.index), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
