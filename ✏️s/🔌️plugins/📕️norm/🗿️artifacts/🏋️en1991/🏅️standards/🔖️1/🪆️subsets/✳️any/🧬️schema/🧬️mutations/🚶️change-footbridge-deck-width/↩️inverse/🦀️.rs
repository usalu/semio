//! ↩️ `change-footbridge-deck-width` inverse: restores the previous `deck_width`.

use crate::change_footbridge_deck_width::ChangeFootbridgeDeckWidth;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFootbridgeDeckWidth, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_footbridge_deck_width::ChangeFootbridgeDeckWidth> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFootbridgeDeckWidth, base: &En1991Snapshot) -> Option<ChangeFootbridgeDeckWidth> {
    let r0 = base.footbridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFootbridgeDeckWidth { id: payload.id.clone(), new_deck_width: r0.deck_width })
}
