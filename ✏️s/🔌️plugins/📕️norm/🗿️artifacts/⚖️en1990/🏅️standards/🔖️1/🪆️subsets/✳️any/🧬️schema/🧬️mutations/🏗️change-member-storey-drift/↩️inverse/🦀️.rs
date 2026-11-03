//! ↩️ `change-member-storey-drift` inverse: restores the previous `storey_drift`.

use crate::change_member_storey_drift::ChangeMemberStoreyDrift;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberStoreyDrift, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_storey_drift::ChangeMemberStoreyDrift> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberStoreyDrift, base: &En1990Snapshot) -> Option<ChangeMemberStoreyDrift> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberStoreyDrift { id: payload.id.clone(), new_storey_drift: r0.storey_drift })
}
