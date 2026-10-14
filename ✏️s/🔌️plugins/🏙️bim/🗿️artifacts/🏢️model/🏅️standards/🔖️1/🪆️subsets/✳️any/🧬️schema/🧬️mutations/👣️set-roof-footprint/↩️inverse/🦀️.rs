//! ↩️ Inverse of `SetRoofFootprint`: an absolute `SetRoofFootprint` back to the base footprint, none when the roof is absent.

use super::SetRoofFootprint;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &SetRoofFootprint, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.roofs.get(&payload.id) {
        Some(roof) => vec![ModelMutation::SetRoofFootprint(SetRoofFootprint { id: payload.id.clone(), footprint: roof.footprint.clone() })],
        None => Vec::new(),
    }
}
