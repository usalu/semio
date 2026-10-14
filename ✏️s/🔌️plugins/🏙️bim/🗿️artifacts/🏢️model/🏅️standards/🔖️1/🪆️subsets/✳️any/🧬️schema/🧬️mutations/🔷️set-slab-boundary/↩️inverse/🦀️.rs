//! ↩️ Inverse of `SetSlabBoundary`: an absolute `SetSlabBoundary` back to the base boundary and holes, none when the slab is absent.

use super::SetSlabBoundary;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &SetSlabBoundary, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.slabs.get(&payload.id) {
        Some(slab) => vec![ModelMutation::SetSlabBoundary(SetSlabBoundary { id: payload.id.clone(), boundary: slab.boundary.clone(), holes: slab.holes.clone() })],
        None => Vec::new(),
    }
}
