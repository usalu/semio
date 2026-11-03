//! ↩️ `change-tension-action-force` inverse: restores the previous `force`.

use crate::change_tension_action_force::ChangeTensionActionForce;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionActionForce, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_action_force::ChangeTensionActionForce> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id).and_then(|row| row.actions.iter().find(|row| row.id == payload.element_id)) {
        Some(row) => vec![Op::from(ChangeTensionActionForce { id: payload.id.clone(), element_id: payload.element_id.clone(), new_force: row.force })],
        None => Vec::new(),
    })
}
