//! ↩️ `change-bridge-deck-ei-nm2` inverse: restores the previous `deck_ei_nm2`.

use crate::change_bridge_deck_ei_nm2::ChangeBridgeDeckEiNm2;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeDeckEiNm2, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_deck_ei_nm2::ChangeBridgeDeckEiNm2> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgeDeckEiNm2, base: &En1998Snapshot) -> Option<ChangeBridgeDeckEiNm2> {
    let r0 = base.bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgeDeckEiNm2 { id: payload.id.clone(), new_deck_ei_nm2: r0.deck_ei_nm2 })
}
