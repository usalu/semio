//! ↩️ Inverse of `SetStoreyCutHeight`: an absolute `SetStoreyCutHeight` back to the base cut height (an assigned null when the base had
//! none), none when the storey is absent.

use super::SetStoreyCutHeight;
use crate::{Assigned, ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetStoreyCutHeight, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.storeys.get(&payload.id) {
        Some(storey) => vec![ModelMutation::SetStoreyCutHeight(SetStoreyCutHeight { id: payload.id.clone(), cut_height: Assigned::new(storey.cut_height) })],
        None => Vec::new(),
    }
}
