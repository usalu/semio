//! ↩️ `change-wind-bridge-deck-width` inverse: restores the previous `deck_width`.

use crate::change_wind_bridge_deck_width::ChangeWindBridgeDeckWidth;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeWindBridgeDeckWidth, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_bridge_deck_width::ChangeWindBridgeDeckWidth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWindBridgeDeckWidth, base: &En1991Snapshot) -> Option<ChangeWindBridgeDeckWidth> {
    let r0 = base.wind_bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWindBridgeDeckWidth { id: payload.id.clone(), new_deck_width: r0.deck_width })
}
