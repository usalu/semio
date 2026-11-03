//! ↩️ `remove-member` inverse: inserts the removed row back at its position.

use crate::remove_member::RemoveMember;
use crate::insert_member::InsertMember;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveMember, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_member::InsertMember> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveMember, base: &En1998Snapshot) -> Option<InsertMember> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let (index, row) = r0.members.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertMember { building_id: payload.building_id.clone(), index: Some(index), member: row.clone() })
}
