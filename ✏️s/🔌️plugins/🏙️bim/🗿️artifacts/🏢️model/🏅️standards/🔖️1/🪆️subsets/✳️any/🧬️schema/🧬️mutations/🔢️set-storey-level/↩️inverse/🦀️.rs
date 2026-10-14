//! ↩️ Inverse of `SetStoreyLevel`: an absolute `SetStoreyLevel` back to the base level, none when the storey is absent.

use super::SetStoreyLevel;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &SetStoreyLevel, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.storeys.get(&payload.id) {
        Some(storey) => vec![ModelMutation::SetStoreyLevel(SetStoreyLevel { id: payload.id.clone(), level: storey.level })],
        None => Vec::new(),
    }
}
