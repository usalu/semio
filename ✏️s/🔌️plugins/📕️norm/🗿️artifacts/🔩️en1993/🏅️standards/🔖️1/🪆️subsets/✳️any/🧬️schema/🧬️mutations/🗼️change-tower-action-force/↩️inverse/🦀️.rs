//! ↩️ `change-tower-action-force` inverse: restores the previous `force`.

use crate::change_tower_action_force::ChangeTowerActionForce;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerActionForce, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_action_force::ChangeTowerActionForce> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id).and_then(|row| row.actions.iter().find(|row| row.id == payload.element_id)) {
        Some(row) => vec![Op::from(ChangeTowerActionForce { id: payload.id.clone(), element_id: payload.element_id.clone(), new_force: row.force })],
        None => Vec::new(),
    })
}
