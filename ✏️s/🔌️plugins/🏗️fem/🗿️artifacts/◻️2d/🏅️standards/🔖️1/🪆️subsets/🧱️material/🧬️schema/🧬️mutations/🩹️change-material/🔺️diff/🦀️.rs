//! 🔺️ Validates field intent on the current material and raises the authored sparse patch.

use super::ChangeMaterial;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dMaterialsDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dMaterialsModification;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

pub fn diff(payload: &ChangeMaterial, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.materials.iter().find(|material| material.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let candidate = payload.patch.candidate(existing);
    if let Some(rejection) = guards::material_plausibility(&candidate) { return rejection; }
    if candidate == *existing {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Material \"{}\" already has the supplied properties.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { materials: Some(Fem2dMaterialsDelta {
        modified: vec![Fem2dMaterialsModification { id: payload.id.clone(), patch: payload.patch.clone() }],
        ..Default::default()
    }), ..Default::default() })
}
