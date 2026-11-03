//! ↩️ `change-fire-member-id` inverse: restores the previous `member_id`.

use crate::change_fire_member_id::ChangeFireMemberId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeFireMemberId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_member_id::ChangeFireMemberId> {
    Ok(match base.fire_scenarios.iter().find(|row| row.id == payload.fire_scenario_id) {
        Some(row) => vec![Op::from(ChangeFireMemberId { fire_scenario_id: payload.fire_scenario_id.clone(), new_member_id: row.member_id.clone() })],
        None => Vec::new(),
    })
}
