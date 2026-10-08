//! ↩️ Inverse of `CreateGridLine`: the concrete `DeleteGridLine` of the id it created, none when the id was already taken.

use super::super::delete_grid_line::DeleteGridLine;
use super::CreateGridLine;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateGridLine, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.grids.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteGridLine(DeleteGridLine { id: payload.id.clone() })]
}
