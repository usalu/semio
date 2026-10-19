//! ↩️ Restores only the element fields this event authors.
use super::ChangeElement;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;
use protocol::list_delta::RowPatch;

pub fn inverse<Op: From<ChangeElement>>(payload: &ChangeElement, base: &Fem2dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(base.elements.iter().find(|row| semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id(row) == payload.id)
        .filter(|row| payload.patch.candidate(row).is_ok_and(|candidate| candidate != **row))
        .map(|row| vec![Op::from(ChangeElement { id: payload.id.clone(), patch: payload.patch.inverse(row) })])
        .unwrap_or_default())
}
