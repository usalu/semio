//! ↩️ Inverse of `SetCeilingBoundary`: an absolute `SetCeilingBoundary` back to the base boundary and holes, none when the ceiling is absent.

use super::SetCeilingBoundary;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetCeilingBoundary, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.ceilings.get(&payload.id) {
        Some(ceiling) => vec![ModelMutation::SetCeilingBoundary(SetCeilingBoundary { id: payload.id.clone(), boundary: ceiling.boundary.clone(), holes: ceiling.holes.clone() })],
        None => Vec::new(),
    }
}
