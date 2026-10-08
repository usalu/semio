//! ↩️ Inverse of `MoveOpening`: an absolute `MoveOpening` back to the base offset, and back to the base host when the move named one,
//! none when the opening is absent.

use super::MoveOpening;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &MoveOpening, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.openings.get(&payload.id) {
        Some(opening) => vec![ModelMutation::MoveOpening(MoveOpening { id: payload.id.clone(), offset: opening.offset, host: payload.host.as_ref().map(|_| opening.host.clone()) })],
        None => Vec::new(),
    }
}
