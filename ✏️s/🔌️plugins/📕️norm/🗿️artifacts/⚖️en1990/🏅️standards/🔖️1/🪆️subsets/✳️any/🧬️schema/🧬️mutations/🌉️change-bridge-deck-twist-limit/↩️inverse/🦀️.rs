//! ↩️ `change-bridge-deck-twist-limit` inverse: restores the previous `deck_twist_limit`.

use crate::change_bridge_deck_twist_limit::ChangeBridgeDeckTwistLimit;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeDeckTwistLimit, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_deck_twist_limit::ChangeBridgeDeckTwistLimit> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeDeckTwistLimit, base: &En1990Snapshot) -> Option<ChangeBridgeDeckTwistLimit> {
    let r0 = base.bridge_sls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeDeckTwistLimit { id: payload.id.clone(), new_deck_twist_limit: r0.deck_twist_limit })
}
