//! ↩️ Inverse of `SplitSlab` in storage order, replayed reversed by the store: the original outline and holes are restored first as one
//! `SetSlabBoundary` of the exact base loops, then the created slab, which holds no data of its own by then, is deleted. Empty when the
//! split is refused.

use super::super::delete_slab::DeleteSlab;
use super::super::set_slab_boundary::SetSlabBoundary;
use super::diff::pieces;
use super::SplitSlab;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SplitSlab, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if pieces(payload, base).is_err() {
        return Vec::new();
    }
    let Some(slab) = base.slabs.get(&payload.id) else {
        return Vec::new();
    };
    vec![
        ModelMutation::DeleteSlab(DeleteSlab { id: payload.new_id.clone() }),
        ModelMutation::SetSlabBoundary(SetSlabBoundary { id: payload.id.clone(), boundary: slab.boundary.clone(), holes: slab.holes.clone() }),
    ]
}
