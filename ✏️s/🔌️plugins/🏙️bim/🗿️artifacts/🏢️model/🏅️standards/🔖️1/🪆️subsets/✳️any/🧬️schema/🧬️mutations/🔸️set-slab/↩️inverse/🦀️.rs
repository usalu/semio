//! ↩️ Inverse of `SetSlab`: an absolute `SetSlab` carrying the base values of exactly the provided fields, none when the slab is absent.

use super::SetSlab;
use crate::{Assigned, ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetSlab, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.slabs.get(&payload.id) {
        Some(slab) => vec![ModelMutation::SetSlab(SetSlab {
            id: payload.id.clone(),
            slab_type: payload.slab_type.as_ref().map(|_| slab.slab_type.clone()),
            offset: payload.offset.map(|_| slab.offset),
            slope: payload.slope.as_ref().map(|_| Assigned::new(slab.slope)),
            name: payload.name.as_ref().map(|_| slab.name.clone()),
        })],
        None => Vec::new(),
    }
}
