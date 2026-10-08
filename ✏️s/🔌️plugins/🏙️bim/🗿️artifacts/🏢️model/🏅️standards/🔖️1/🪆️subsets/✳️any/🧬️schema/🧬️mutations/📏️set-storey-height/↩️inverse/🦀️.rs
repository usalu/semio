//! ↩️ Inverse of `SetStoreyHeight`: an absolute `SetStoreyHeight` back to the base height, none when the storey is absent.

use super::SetStoreyHeight;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetStoreyHeight, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.storeys.get(&payload.id) {
        Some(storey) => vec![ModelMutation::SetStoreyHeight(SetStoreyHeight { id: payload.id.clone(), height: storey.height })],
        None => Vec::new(),
    }
}
