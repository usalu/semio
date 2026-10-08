//! ↩️ Inverse of `DeleteWallType`: the concrete `CreateWallType` carrying the full removed record, none when the wall type was absent.

use super::super::create_wall_type::CreateWallType;
use super::DeleteWallType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteWallType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.wall_types.get(&payload.id) {
        Some(wall_type) => vec![ModelMutation::CreateWallType(CreateWallType { id: payload.id.clone(), wall_type: wall_type.clone() })],
        None => Vec::new(),
    }
}
