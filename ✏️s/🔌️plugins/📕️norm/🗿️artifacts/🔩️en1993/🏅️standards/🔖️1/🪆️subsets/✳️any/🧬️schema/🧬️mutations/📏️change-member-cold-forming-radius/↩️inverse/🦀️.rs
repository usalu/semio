//! ↩️ `change-member-cold-forming-radius` inverse: restores the previous `cold_forming_radius`.

use crate::change_member_cold_forming_radius::ChangeMemberColdFormingRadius;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberColdFormingRadius, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_cold_forming_radius::ChangeMemberColdFormingRadius> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberColdFormingRadius { id: payload.id.clone(), new_cold_forming_radius: row.cold_forming_radius })],
        None => Vec::new(),
    })
}
