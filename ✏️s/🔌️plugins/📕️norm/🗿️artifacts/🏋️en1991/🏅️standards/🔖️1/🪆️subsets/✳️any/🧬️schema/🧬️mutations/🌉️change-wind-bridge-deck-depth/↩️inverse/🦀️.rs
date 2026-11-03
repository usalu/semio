//! ↩️ `change-wind-bridge-deck-depth` inverse: restores the previous `deck_depth`.

use crate::change_wind_bridge_deck_depth::ChangeWindBridgeDeckDepth;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeWindBridgeDeckDepth, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wind_bridge_deck_depth::ChangeWindBridgeDeckDepth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWindBridgeDeckDepth, base: &En1991Snapshot) -> Option<ChangeWindBridgeDeckDepth> {
    let r0 = base.wind_bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWindBridgeDeckDepth { id: payload.id.clone(), new_deck_depth: r0.deck_depth })
}
