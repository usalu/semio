//! ↩️ Inverse of `RenameStorey`: an absolute `RenameStorey` back to the base name, none when the storey is absent.

use super::RenameStorey;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &RenameStorey, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.storeys.get(&payload.id) {
        Some(storey) => vec![ModelMutation::RenameStorey(RenameStorey { id: payload.id.clone(), name: storey.name.clone() })],
        None => Vec::new(),
    }
}
