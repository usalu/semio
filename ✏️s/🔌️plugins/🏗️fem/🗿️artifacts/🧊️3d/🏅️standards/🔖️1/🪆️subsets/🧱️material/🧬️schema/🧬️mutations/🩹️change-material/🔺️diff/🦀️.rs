//! 🔺️ Validates field intent on the current material and raises the authored sparse patch.

use super::ChangeMaterial;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dMaterialsDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dMaterialsModification;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::invariant;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::material_breach;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

pub fn diff(payload: &ChangeMaterial, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.materials.iter().find(|material| material.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let candidate = payload.patch.candidate(existing);
    if let Some(breach) = material_breach(&candidate) { return invariant(breach, vec![payload.id.clone()]); }
    if candidate == *existing {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Material \"{}\" already has the supplied properties.", payload.id));
    }
    protocol::MutationOutcome::new(Fem3dDiff { materials: Some(Fem3dMaterialsDelta {
        modified: vec![Fem3dMaterialsModification { id: payload.id.clone(), patch: payload.patch.clone() }],
        ..Default::default()
    }), ..Default::default() })
}
