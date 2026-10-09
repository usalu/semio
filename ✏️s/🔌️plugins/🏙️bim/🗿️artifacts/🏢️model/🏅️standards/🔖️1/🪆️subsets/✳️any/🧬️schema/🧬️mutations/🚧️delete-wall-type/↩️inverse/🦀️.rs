//! ↩️ Inverse of `DeleteWallType`: the setters of the properties and classifications of the type, then the concrete `CreateWallType` carrying the full removed record, none when the wall type was absent.

use super::super::create_wall_type::CreateWallType;
use super::super::cascade;
use super::DeleteWallType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteWallType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.wall_types.get(&payload.id) {
        Some(wall_type) => cascade::data_rows(base, &payload.id).into_iter().chain(std::iter::once(ModelMutation::CreateWallType(CreateWallType { id: payload.id.clone(), wall_type: wall_type.clone() }))).collect(),
        None => Vec::new(),
    }
}
