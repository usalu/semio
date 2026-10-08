//! ↩️ Inverse of `DeleteDoorType`: the concrete `CreateDoorType` carrying the full removed record, none when the door type was absent.

use super::super::create_door_type::CreateDoorType;
use super::DeleteDoorType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteDoorType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.door_types.get(&payload.id) {
        Some(record) => vec![ModelMutation::CreateDoorType(CreateDoorType { id: payload.id.clone(), door_type: record.clone() })],
        None => Vec::new(),
    }
}
