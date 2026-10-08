//! ↩️ Inverse of `SetRoofShape`: an absolute `SetRoofShape` carrying the base values of exactly the provided fields, none when the roof
//! is absent.

use super::SetRoofShape;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetRoofShape, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.roofs.get(&payload.id) {
        Some(roof) => vec![ModelMutation::SetRoofShape(SetRoofShape {
            id: payload.id.clone(),
            shape: payload.shape.as_ref().map(|_| roof.shape.clone()),
            overhang: payload.overhang.map(|_| roof.overhang),
            base_offset: payload.base_offset.map(|_| roof.base_offset),
        })],
        None => Vec::new(),
    }
}
