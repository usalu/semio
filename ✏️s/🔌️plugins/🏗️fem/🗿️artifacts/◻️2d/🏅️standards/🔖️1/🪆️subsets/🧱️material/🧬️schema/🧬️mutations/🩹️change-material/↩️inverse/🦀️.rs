//! ↩️ Restores exactly the material fields this event authors.

use super::ChangeMaterial;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;
use protocol::list_delta::RowPatch;

pub fn inverse<Op: From<ChangeMaterial>>(payload: &ChangeMaterial, base: &Fem2dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(base.materials.iter().find(|material| material.id == payload.id)
        .filter(|material| payload.patch.candidate(material) != **material)
        .map(|material| vec![Op::from(ChangeMaterial { id: payload.id.clone(), patch: payload.patch.inverse(material) })])
        .unwrap_or_default())
}
