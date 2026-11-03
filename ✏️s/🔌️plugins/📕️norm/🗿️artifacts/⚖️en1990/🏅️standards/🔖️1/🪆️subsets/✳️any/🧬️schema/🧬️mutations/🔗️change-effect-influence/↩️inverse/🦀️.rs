//! ↩️ `change-effect-influence` inverse: restores the previous `influence`.

use crate::change_effect_influence::ChangeEffectInfluence;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeEffectInfluence, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_effect_influence::ChangeEffectInfluence> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeEffectInfluence, base: &En1990Snapshot) -> Option<ChangeEffectInfluence> {
    let r0 = base.effects.get(payload.index)?;
    Some(ChangeEffectInfluence { index: payload.index, new_influence: r0.influence })
}
