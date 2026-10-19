//! ↩️ Restores only the region fields this event authors.
use super::ChangeRegion;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;
use protocol::list_delta::RowPatch;

pub fn inverse<Op: From<ChangeRegion>>(payload: &ChangeRegion, base: &Fem2dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(base.regions.iter().find(|row| row.id == payload.id)
        .filter(|row| payload.patch.candidate(row) != **row)
        .map(|row| vec![Op::from(ChangeRegion { id: payload.id.clone(), patch: payload.patch.inverse(row) })])
        .unwrap_or_default())
}
