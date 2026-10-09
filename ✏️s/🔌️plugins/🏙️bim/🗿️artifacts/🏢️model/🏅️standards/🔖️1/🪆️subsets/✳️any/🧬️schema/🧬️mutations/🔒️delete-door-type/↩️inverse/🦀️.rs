//! ↩️ Inverse of `DeleteDoorType`: the setters of the properties and classifications of the type, then the concrete `CreateDoorType` carrying the full removed record, none when the door type was absent.

use super::super::create_door_type::CreateDoorType;
use super::super::cascade;
use super::DeleteDoorType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteDoorType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.door_types.get(&payload.id) {
        Some(record) => cascade::data_rows(base, &payload.id).into_iter().chain(std::iter::once(ModelMutation::CreateDoorType(CreateDoorType { id: payload.id.clone(), door_type: record.clone() }))).collect(),
        None => Vec::new(),
    }
}
