//! ↩️ `change-member-detailing` inverse: restores the previous `detailing_compatible_with_q`.

use crate::change_member_detailing::ChangeMemberDetailing;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberDetailing, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_detailing::ChangeMemberDetailing> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberDetailing, base: &En1998Snapshot) -> Option<ChangeMemberDetailing> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberDetailing { building_id: payload.building_id.clone(), id: payload.id.clone(), new_detailing_compatible_with_q: r1.detailing_compatible_with_q })
}
