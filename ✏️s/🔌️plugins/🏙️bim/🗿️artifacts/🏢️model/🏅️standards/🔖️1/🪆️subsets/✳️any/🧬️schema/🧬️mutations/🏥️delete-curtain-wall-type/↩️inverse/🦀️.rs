//! ↩️ Inverse of `DeleteCurtainWallType`: the concrete `CreateCurtainWallType` carrying the full removed record, none when the record was absent.

use super::super::create_curtain_wall_type::CreateCurtainWallType;
use super::DeleteCurtainWallType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteCurtainWallType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.curtain_wall_types.get(&payload.id) {
        Some(record) => vec![ModelMutation::CreateCurtainWallType(CreateCurtainWallType { id: payload.id.clone(), curtain_wall_type: record.clone() })],
        None => Vec::new(),
    }
}
