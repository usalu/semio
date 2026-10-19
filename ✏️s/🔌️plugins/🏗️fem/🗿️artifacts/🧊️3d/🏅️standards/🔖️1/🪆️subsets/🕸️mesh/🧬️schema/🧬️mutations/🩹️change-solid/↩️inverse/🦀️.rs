//! ↩️ Restores only the solid fields this event authors.
use super::ChangeSolid;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use protocol::list_delta::RowPatch;

pub fn inverse<Op: From<ChangeSolid>>(payload: &ChangeSolid, base: &Fem3dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(base.solids.iter().find(|row| row.id == payload.id)
        .filter(|row| payload.patch.candidate(row) != **row)
        .map(|row| vec![Op::from(ChangeSolid { id: payload.id.clone(), patch: payload.patch.inverse(row) })])
        .unwrap_or_default())
}
