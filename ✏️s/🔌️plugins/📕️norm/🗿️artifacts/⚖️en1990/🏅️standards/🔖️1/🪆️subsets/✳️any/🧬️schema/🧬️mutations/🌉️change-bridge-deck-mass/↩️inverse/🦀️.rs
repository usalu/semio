//! ↩️ `change-bridge-deck-mass` inverse: restores the previous `deck_mass`.

use crate::change_bridge_deck_mass::ChangeBridgeDeckMass;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeDeckMass, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_deck_mass::ChangeBridgeDeckMass> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeDeckMass, base: &En1990Snapshot) -> Option<ChangeBridgeDeckMass> {
    let r0 = base.bridge_sls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeDeckMass { id: payload.id.clone(), new_deck_mass: r0.deck_mass })
}
