//! ↩️ Inverse of `CreateColumnType`: the concrete `DeleteColumnType` of the id it created, none when the id was already taken.

use super::super::delete_column_type::DeleteColumnType;
use super::CreateColumnType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateColumnType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.column_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteColumnType(DeleteColumnType { id: payload.id.clone() })]
}
