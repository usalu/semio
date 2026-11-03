//! ↩️ `change-bridge-deck-twist` inverse: restores the previous `deck_twist`.

use crate::change_bridge_deck_twist::ChangeBridgeDeckTwist;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeDeckTwist, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_deck_twist::ChangeBridgeDeckTwist> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeDeckTwist, base: &En1990Snapshot) -> Option<ChangeBridgeDeckTwist> {
    let r0 = base.bridge_sls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeDeckTwist { id: payload.id.clone(), new_deck_twist: r0.deck_twist })
}
