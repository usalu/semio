//! 🔺️ Sparse diff builder for `CreateMaterial`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), then the shared
//! `guards::material_plausibility` elasticity bounds (`mutation.invariant`, Fatal).
use super::CreateMaterial;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dMaterialInsertion;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dMaterialsDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateMaterial, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if base.materials.iter().any(|material| material.id == payload.material.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A material with id \"{}\" already exists.", payload.material.id), [payload.material.id.clone()]);
    }
    if let Some(rejection) = guards::material_plausibility(&payload.material) {
        return rejection;
    }
    if payload.index.is_some_and(|at| at > base.materials.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.materials.len()), [&payload.material.id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { materials: Some(Fem2dMaterialsDelta { inserted: vec![Fem2dMaterialInsertion { index: payload.index.unwrap_or(base.materials.len()), row: payload.material.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
