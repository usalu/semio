//! ↩️ Inverse of `DeleteColumnType`: the setters of the properties and classifications of the type, then the concrete `CreateColumnType` carrying the full removed record, none when the column type was absent.

use super::super::create_column_type::CreateColumnType;
use super::super::cascade;
use super::DeleteColumnType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteColumnType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.column_types.get(&payload.id) {
        Some(record) => cascade::data_rows(base, &payload.id).into_iter().chain(std::iter::once(ModelMutation::CreateColumnType(CreateColumnType { id: payload.id.clone(), column_type: record.clone() }))).collect(),
        None => Vec::new(),
    }
}
