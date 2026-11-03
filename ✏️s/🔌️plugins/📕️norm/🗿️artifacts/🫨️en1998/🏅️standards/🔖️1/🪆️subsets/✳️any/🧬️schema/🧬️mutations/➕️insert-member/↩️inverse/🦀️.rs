//! ↩️ `insert-member` inverse: removes the inserted row again.

use crate::insert_member::InsertMember;
use crate::remove_member::RemoveMember;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertMember, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_member::RemoveMember> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertMember, base: &En1998Snapshot) -> Option<RemoveMember> {
    let _r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    Some(RemoveMember { building_id: payload.building_id.clone(), id: payload.member.id.clone() })
}
