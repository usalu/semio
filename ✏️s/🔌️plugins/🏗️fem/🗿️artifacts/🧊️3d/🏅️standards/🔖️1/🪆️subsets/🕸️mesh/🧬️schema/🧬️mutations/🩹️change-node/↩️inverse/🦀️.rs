//! ↩️ Restores only the node fields this event authors.
use super::ChangeNode;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use protocol::list_delta::RowPatch;

pub fn inverse<Op: From<ChangeNode>>(payload: &ChangeNode, base: &Fem3dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(base.nodes.iter().find(|row| row.id == payload.id)
        .filter(|row| payload.patch.candidate(row) != **row)
        .map(|row| vec![Op::from(ChangeNode { id: payload.id.clone(), patch: payload.patch.inverse(row) })])
        .unwrap_or_default())
}
