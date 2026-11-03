//! ↩️ `change-effect-action-id` inverse: restores the previous `action_id`.

use crate::change_effect_action_id::ChangeEffectActionId;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeEffectActionId, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_effect_action_id::ChangeEffectActionId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeEffectActionId, base: &En1990Snapshot) -> Option<ChangeEffectActionId> {
    let r0 = base.effects.get(payload.index)?;
    Some(ChangeEffectActionId { index: payload.index, new_action_id: r0.action_id.clone() })
}
